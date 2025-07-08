import * as wasm from "3d_engine_core";
import { Image as ImageJS } from 'image-js';
import { WebGPURenderer } from './webgpu-renderer.js';
import { Matrix4, Matrix3 } from './matrix-utils.js';

const width = window.innerWidth;
const height = window.innerHeight;

const params = new URLSearchParams(window.location.search);

const model_name = params.has('model-name') ? params.get('model-name') : "Head";
const camera_speed = params.has('camera-speed') ? params.get('camera-speed') : 0.05;
const rotation_speed = params.has('rotation-speed') ? params.get('rotation-speed') : 0.1;
const model_scale = params.has('model-scale') ? params.get('model-scale') : 0.1;

const use_normal_map = params.has('normal-map') ? params.get('normal-map') === "true" : true;
const use_diffuse_map = params.has('diffuse-map') ? params.get('diffuse-map') === "true" : true;
const use_specular_map = params.has('specular-map') ? params.get('specular-map') === "true" : true;
const use_emission_map = params.has('emission-map') ? params.get('emission-map') === "true" : true;

let model_rotation = params.has('model-rotation') ? params.get('model-rotation') === "true" : false;

// Check WebGPU support
async function checkWebGPUSupport() {
    if (!navigator.gpu) {
        console.error('WebGPU not supported. Falling back to software rendering.');
        return false;
    }
    
    try {
        const adapter = await navigator.gpu.requestAdapter();
        if (!adapter) {
            console.error('No WebGPU adapter available. Falling back to software rendering.');
            return false;
        }
        return true;
    } catch (error) {
        console.error('WebGPU initialization failed:', error);
        return false;
    }
}

(async () => {
    // Check WebGPU support
    const hasWebGPU = await checkWebGPUSupport();
    
    if (!hasWebGPU) {
        // Fallback to original software rendering
        console.log('Using software rendering (original method)');
        await loadOriginalRenderer();
        return;
    }
    
    console.log('Using WebGPU hardware acceleration');
    
    // Initialize WebGPU renderer
    const canvas = document.getElementById("viewer-canvas");
    canvas.height = height;
    canvas.width = width;
    
    const renderer = new WebGPURenderer();
    
    try {
        await renderer.initialize(canvas);
    } catch (error) {
        console.error('WebGPU initialization failed:', error);
        console.log('Falling back to software rendering');
        await loadOriginalRenderer();
        return;
    }
    
    // Data fetching
    let modelRes = await fetch(`/raw/model?model-name=${model_name}`);
    if (!modelRes.ok) {
        alert("Cannot fetch model");
        return;
    }
    let model = await modelRes.json();

    let diffuseMapRes = await fetch(`/source/${model_name}/Diffuse map.png`);
    let diffuseMapImage = null;
    if (diffuseMapRes.ok) {
        diffuseMapImage = await ImageJS.load(await diffuseMapRes.arrayBuffer());
    } else {
        alert("Cannot fetch diffuse map");
    }

    let normalMapRes = await fetch(`/source/${model_name}/Normal map.png`);
    let normalMapImage = null;
    if (normalMapRes.ok) {
        normalMapImage = await ImageJS.load(await normalMapRes.arrayBuffer());
    } else {
        alert("Cannot fetch normal map");
    }

    let specularMapRes = await fetch(`/source/${model_name}/Specular map.png`);
    let specularMapImage = null;
    if (specularMapRes.ok) {
        specularMapImage = await ImageJS.load(await specularMapRes.arrayBuffer());
    } else {
        alert("Cannot fetch specular map");
    }

    let emissionMapRes = await fetch(`/source/${model_name}/Emission map.png`);
    let emissionMapImage = null;
    if (emissionMapRes.ok) {
        emissionMapImage = await ImageJS.load(await emissionMapRes.arrayBuffer());
    }

    // Core stuff
    const image = wasm.Image.new(width, height);
    const objHandler1 = image.new_object();

    // Load model data
    model.v.forEach(v => {
        image.add_object_vertex(objHandler1, v.x, v.y, v.z);
    });

    model.vn.forEach(vn => {
        image.add_object_vertex_normal(objHandler1, vn.x, vn.y, vn.z);
    });

    model.vt.forEach(vt => {
        image.add_object_texture_vertex(objHandler1, vt.x, vt.y, vt.z);
    });

    model.f.forEach(f => {
        image.add_object_face(objHandler1,
            f[0].v - 1, f[0].vt - 1, (f[0].vn || 1) - 1,
            f[1].v - 1, f[1].vt - 1, (f[1].vn || 1) - 1,
            f[2].v - 1, f[2].vt - 1, (f[2].vn || 1) - 1
        );
    });

    image.set_object_scale(objHandler1, model_scale);

    // Create textures for WebGPU
    let diffuseTexture = null;
    let normalTexture = null;
    let specularTexture = null;
    let emissionTexture = null;

    if (use_diffuse_map && diffuseMapImage) {
        diffuseTexture = renderer.createTextureFromImageData({
            data: diffuseMapImage.getRGBAData({ clamped: true }),
            width: diffuseMapImage.width,
            height: diffuseMapImage.height
        });
        console.log('using diffuse map');
    }

    if (use_normal_map && normalMapImage) {
        normalTexture = renderer.createTextureFromImageData({
            data: normalMapImage.getRGBAData({ clamped: true }),
            width: normalMapImage.width,
            height: normalMapImage.height
        });
        console.log('using normal map');
    }

    if (use_specular_map && specularMapImage) {
        specularTexture = renderer.createTextureFromImageData({
            data: specularMapImage.getRGBAData({ clamped: true }),
            width: specularMapImage.width,
            height: specularMapImage.height
        });
        console.log('using specular map');
    }

    if (use_emission_map && emissionMapImage) {
        emissionTexture = renderer.createTextureFromImageData({
            data: emissionMapImage.getRGBAData({ clamped: true }),
            width: emissionMapImage.width,
            height: emissionMapImage.height
        });
        console.log('using emission map');
    }

    // Update renderer textures
    renderer.updateTextures(diffuseTexture, normalTexture, specularTexture, emissionTexture);

    // Get vertex data from Rust
    const vertexDataLength = image.get_vertex_data_length(objHandler1);
    const vertexDataPtr = image.get_vertex_data(objHandler1);
    const vertexData = new Float32Array(wasm.memory.buffer, vertexDataPtr, vertexDataLength);

    const indexDataLength = image.get_index_data_length(objHandler1);
    const indexDataPtr = image.get_index_data(objHandler1);
    const indexData = new Uint16Array(wasm.memory.buffer, indexDataPtr, indexDataLength);

    // Update renderer with vertex data
    renderer.updateVertexData(vertexData, indexData);

    // FPS stuff
    let lastLoop = new Date();
    let fpsLabel = document.getElementById("fps-label");
    let angle = 0;
    let pressedKeys = {};

    const renderLoop = () => {
        // Update camera and model transformations
        image.set_camera_param(1, 0); // Reset camera movement
        image.set_camera_param(2, 0);
        image.set_camera_param(3, 0);
        image.set_camera_param(11, 0);
        image.set_camera_param(12, 0);

        if (model_rotation) {
            angle += rotation_speed;
            image.set_object_rotation(objHandler1, 0, angle, 0);
        }

        // Update camera based on key presses
        for (const [key, pressed] of Object.entries(pressedKeys)) {
            if (pressed) {
                switch (key) {
                    case "KeyD":
                        image.set_camera_param(1, camera_speed);
                        break;
                    case "KeyA":
                        image.set_camera_param(1, -camera_speed);
                        break;
                    case "KeyQ":
                        image.set_camera_param(2, camera_speed);
                        break;
                    case "KeyE":
                        image.set_camera_param(2, -camera_speed);
                        break;
                    case "KeyW":
                        image.set_camera_param(3, camera_speed);
                        break;
                    case "KeyS":
                        image.set_camera_param(3, -camera_speed);
                        break;
                    case "ArrowUp":
                        image.set_camera_param(11, -camera_speed);
                        break;
                    case "ArrowDown":
                        image.set_camera_param(11, camera_speed);
                        break;
                    case "ArrowRight":
                        image.set_camera_param(12, camera_speed);
                        break;
                    case "ArrowLeft":
                        image.set_camera_param(12, -camera_speed);
                        break;
                }
            }
        }

        // Update Rust state (for camera and transformations)
        image.compute();

        // Get updated matrices from Rust
        const viewProjectionPtr = image.get_view_projection_matrix();
        const modelMatrixPtr = image.get_model_matrix(objHandler1);
        const cameraPosPtr = image.get_camera_position();

        const viewProjectionMatrix = new Float32Array(wasm.memory.buffer, viewProjectionPtr, 16);
        const modelMatrix = new Float32Array(wasm.memory.buffer, modelMatrixPtr, 16);
        const cameraPosition = new Float32Array(wasm.memory.buffer, cameraPosPtr, 3);

        // Calculate normal matrix (inverse transpose of model matrix)
        const modelMat4 = new Matrix4(modelMatrix);
        const normalMat3 = new Matrix3().fromMatrix4(modelMat4.clone().invert().transpose());

        // Update uniforms
        renderer.updateUniforms(
            viewProjectionMatrix,
            modelMatrix,
            normalMat3.elements,
            cameraPosition
        );

        // Render with WebGPU
        renderer.render();

        // Update FPS
        const thisLoop = new Date();
        const fps = 1000 / (thisLoop - lastLoop);
        lastLoop = thisLoop;
        fpsLabel.innerHTML = Math.round(fps);

        requestAnimationFrame(renderLoop);
    };

    requestAnimationFrame(renderLoop);

    // Input handling
    document.addEventListener('keyup', (e) => {
        pressedKeys[e.code] = false;
    });

    document.addEventListener('keydown', (e) => {
        if (pressedKeys[e.code]) return;
        pressedKeys[e.code] = true;

        if (e.code === "Space") {
            model_rotation = !model_rotation;
        }
    });

})();

// Fallback to original software rendering
async function loadOriginalRenderer() {
    // Load the original index.js content here
    const originalScript = document.createElement('script');
    originalScript.src = './index-original.js';
    document.head.appendChild(originalScript);
}
