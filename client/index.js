import * as wasm from "3d_engine_core";
// import { memory } from "3d_engine_core";
import * as wasm_bg from "3d_engine_core/core_bg.wasm";
import { Image as ImageJS } from 'image-js';
import { WebGPURenderer } from './webgpu-renderer.js';
import { Matrix4, Matrix3 } from './matrix-utils.js';

const width = window.innerWidth;
const height = window.innerHeight;

const params = new URLSearchParams(window.location.search);

const model_name = params.has('model-name') ? params.get('model-name') : "Head";
const camera_speed = params.has('camera-speed') ? params.get('camera-speed') : 0.05;
const rotation_speed = params.has('rotation-speed') ? params.get('rotation-speed') : 0.01;
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
    // WebAssembly module is already initialized
    // Memory is imported directly
    
    // Check if we should use the original software renderer
    const urlParams = new URLSearchParams(window.location.search);
    if (urlParams.get('fallback') === 'true') {
        console.log('Using original software renderer (fallback mode)');
        document.getElementById('renderer-info').textContent = 'Using Software Renderer (Fallback)';
        await loadOriginalRendererDirectly(wasm_bg.memory);
        return;
    }
    
    // Check WebGPU support
    const hasWebGPU = await checkWebGPUSupport();
    
    if (!hasWebGPU) {
        // Fallback to original software rendering
        console.log('Using software rendering (original method)');
        document.getElementById('renderer-info').textContent = 'Using Software Renderer (WebGPU not supported)';
        await loadOriginalRendererDirectly(wasm_bg.memory);
        return;
    }
    
    console.log('Using WebGPU hardware acceleration');
    document.getElementById('renderer-info').textContent = 'Using WebGPU Hardware Acceleration';
    
    // Initialize WebGPU renderer
    const canvas = document.getElementById("viewer-canvas");
    canvas.height = height;
    canvas.width = width;
    
    const renderer = new WebGPURenderer();
    
    try {
        await renderer.initialize(canvas);
        document.getElementById('renderer-info').textContent = 'WebGPU Hardware Acceleration Active';
    } catch (error) {
        console.error('WebGPU initialization failed:', error);
        console.log('Falling back to software rendering');
        document.getElementById('renderer-info').textContent = 'WebGPU Failed - Using Software Renderer';
        await loadOriginalRendererDirectly(wasm_bg.memory);
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

    // Generate vertex data directly from model data (alternative approach)
    console.log('Generating vertex data from model data...');
    console.log('Model faces:', model.f.length);
    console.log('Model vertices:', model.v.length);
    console.log('Model normals:', model.vn.length);
    console.log('Model texture vertices:', model.vt.length);
    
    // Create vertex data directly from model
    const generatedVertexData = [];
    const generatedIndexData = [];
    
    for (let faceIndex = 0; faceIndex < model.f.length; faceIndex++) {
        const face = model.f[faceIndex];
        
        for (let vertexIndex = 0; vertexIndex < 3; vertexIndex++) {
            const faceVertex = face[vertexIndex];
            
            // Get vertex position (1-indexed in OBJ format)
            const vertex = model.v[faceVertex.v - 1];
            if (!vertex) {
                console.error('Missing vertex for face', faceIndex, 'vertex', vertexIndex, 'index', faceVertex.v);
                continue;
            }
            
            // Get normal (1-indexed in OBJ format)
            const normalIndex = faceVertex.vn ? faceVertex.vn - 1 : 0;
            const normal = model.vn[normalIndex] || { x: 0, y: 0, z: 1 };
            
            // Get texture coordinate (1-indexed in OBJ format)
            const texCoordIndex = faceVertex.vt ? faceVertex.vt - 1 : 0;
            const texCoord = model.vt[texCoordIndex] || { x: 0, y: 0 };
            
            // Add vertex data (position, normal, texture coordinates)
            generatedVertexData.push(vertex.x, vertex.y, vertex.z);
            generatedVertexData.push(normal.x, normal.y, normal.z);
            generatedVertexData.push(texCoord.x, texCoord.y);
            
            // Add index
            generatedIndexData.push(faceIndex * 3 + vertexIndex);
        }
    }
    
    console.log('Generated vertex data length:', generatedVertexData.length);
    console.log('Generated index data length:', generatedIndexData.length);
    
    // Compare with Rust-generated data (using the new memory-safe API)
    const vertexDataLength = image.get_vertex_data_length(objHandler1);
    const indexDataLength = image.get_index_data_length(objHandler1);
    
    // Update the vertex data cache first
    const vertexDataPtr = image.get_vertex_data(objHandler1);
    const indexDataPtr = image.get_index_data(objHandler1);
    
    const vertexDataRaw = new Float32Array(wasm_bg.memory.buffer, vertexDataPtr, vertexDataLength);
    const vertexData = new Float32Array(vertexDataRaw);

    const indexDataRaw = new Uint16Array(wasm_bg.memory.buffer, indexDataPtr, indexDataLength);
    const indexData = new Uint16Array(indexDataRaw);
    
    console.log('Rust vertex data length:', vertexDataLength, 'vs Generated:', generatedVertexData.length);
    console.log('Rust index data length:', indexDataLength, 'vs Generated:', generatedIndexData.length);
    
    // Debug: compare first few vertices
    console.log('First few Rust vertices:', vertexData.slice(0, 24));
    console.log('First few generated vertices:', generatedVertexData.slice(0, 24));
    
    // Use URL parameter to choose approach
    const useGeneratedData = new URLSearchParams(window.location.search).get('use-generated') !== 'false';
    console.log('Using generated data:', useGeneratedData);
    
    // Check if we need 32-bit indices due to large vertex count
    const estimatedVertexCount = useGeneratedData ? generatedVertexData.length / 8 : vertexDataLength / 8;
    const needs32BitIndices = estimatedVertexCount > 65535;
    console.log('Estimated vertex count:', estimatedVertexCount);
    console.log('Needs 32-bit indices:', needs32BitIndices);
    
    // Use appropriate index buffer type based on vertex count
    const finalVertexData = useGeneratedData ? new Float32Array(generatedVertexData) : vertexData;
    const finalIndexData = useGeneratedData ? 
        (needs32BitIndices ? new Uint32Array(generatedIndexData) : new Uint16Array(generatedIndexData)) : 
        indexData;

    // Debug: Log vertex data info
    console.log('Final vertex data length:', finalVertexData.length);
    console.log('Final index data length:', finalIndexData.length);
    console.log('Vertices per vertex:', finalVertexData.length / 8); // Should be number of vertices
    console.log('Triangles:', finalIndexData.length / 3); // Should be number of triangles
    console.log('First few vertices:', finalVertexData.slice(0, 24)); // First 3 vertices (8 floats each)
    console.log('First few indices:', finalIndexData.slice(0, 12)); // First 4 triangles
    
    // Check for invalid vertex data
    let invalidVertexCount = 0;
    let invalidIndexCount = 0;
    let maxIndex = -1;
    
    for (let i = 0; i < finalVertexData.length; i++) {
        if (!isFinite(finalVertexData[i])) {
            invalidVertexCount++;
        }
    }
    
    for (let i = 0; i < finalIndexData.length; i++) {
        if (!isFinite(finalIndexData[i]) || finalIndexData[i] < 0) {
            invalidIndexCount++;
        }
        maxIndex = Math.max(maxIndex, finalIndexData[i]);
    }
    
    console.log('Invalid vertex values:', invalidVertexCount);
    console.log('Invalid index values:', invalidIndexCount);
    console.log('Max index:', maxIndex, 'vs vertices available:', Math.floor(finalVertexData.length / 8));
    console.log('Index buffer type:', finalIndexData.constructor.name, 'can handle max index:', finalIndexData.constructor === Uint32Array ? '4,294,967,295' : '65,535');
    
    // Check for index out of bounds
    const vertexCount = Math.floor(finalVertexData.length / 8);
    if (maxIndex >= vertexCount) {
        console.error('INDEX OUT OF BOUNDS! Max index:', maxIndex, 'but only', vertexCount, 'vertices available');
    } else {
        console.log('✓ Index bounds validation passed');
    }
    
    // Validate vertex data structure
    console.log('Expected vertex structure validation:');
    for (let i = 0; i < Math.min(3, vertexCount); i++) {
        const offset = i * 8;
        const pos = [finalVertexData[offset], finalVertexData[offset + 1], finalVertexData[offset + 2]];
        const normal = [finalVertexData[offset + 3], finalVertexData[offset + 4], finalVertexData[offset + 5]];
        const texCoord = [finalVertexData[offset + 6], finalVertexData[offset + 7]];
        
        console.log(`Vertex ${i}:`, {
            position: pos,
            normal: normal,
            texCoord: texCoord,
            positionMagnitude: Math.sqrt(pos[0] * pos[0] + pos[1] * pos[1] + pos[2] * pos[2]),
            normalMagnitude: Math.sqrt(normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2])
        });
    }

    // Update renderer with vertex data
    renderer.updateVertexData(finalVertexData, finalIndexData);

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

        const viewProjectionMatrix = new Float32Array(wasm_bg.memory.buffer, viewProjectionPtr, 16);
        const modelMatrix = new Float32Array(wasm_bg.memory.buffer, modelMatrixPtr, 16);
        const cameraPosition = new Float32Array(wasm_bg.memory.buffer, cameraPosPtr, 3);

        // Calculate normal matrix (proper inverse transpose)
        const modelMat4 = new Matrix4(modelMatrix);
        // Extract upper-left 3x3, then invert and transpose
        const normalMat3 = new Matrix3().fromMatrix4(modelMat4).invert().transpose();

        // Debug: Check for invalid matrices
        const hasInvalidViewProjection = viewProjectionMatrix.some(v => !isFinite(v));
        const hasInvalidModel = modelMatrix.some(v => !isFinite(v));
        const hasInvalidNormal = normalMat3.elements.some(v => !isFinite(v));
        
        if (hasInvalidViewProjection || hasInvalidModel || hasInvalidNormal) {
            console.warn('Invalid matrices detected:', {
                viewProjection: hasInvalidViewProjection,
                model: hasInvalidModel,
                normal: hasInvalidNormal
            });
        }

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
    console.log('Loading original software renderer...');
    
    // Just reload the page with a fallback flag
    const url = new URL(window.location);
    url.searchParams.set('fallback', 'true');
    window.location.href = url.toString();
}

// Original software renderer implementation
async function loadOriginalRendererDirectly(memory) {
    // This is the original implementation from index-original.js
    
    // data fetching
    let modelRes = await fetch(`/raw/model?model-name=${model_name}`);
    if (!modelRes.ok) {
        alert("Cannot fetch model");
        return;
    }
    let model = await modelRes.json();

    let diffuseMapRes = await fetch(`/source/${model_name}/Diffuse map.png`);
    let diffuseMapImage = null;
    if (diffuseMapRes.ok) {
        diffuseMapImage = await ImageJS.load(await diffuseMapRes.arrayBuffer())
    } else {
        alert("Cannot fetch diffuse map");
    }

    let normalMapRes = await fetch(`/source/${model_name}/Normal map.png`);
    let normalMapImage = null;
    if (normalMapRes.ok) {
        normalMapImage = await ImageJS.load(await normalMapRes.arrayBuffer())
    } else {
        alert("Cannot fetch normal map");
    }

    let specularMapRes = await fetch(`/source/${model_name}/Specular map.png`);
    let specularMapImage = null;
    if (specularMapRes.ok) {
        specularMapImage = await ImageJS.load(await specularMapRes.arrayBuffer())
    } else {
        alert("Cannot fetch specular map");
    }

    let emissionMapRes = await fetch(`/source/${model_name}/Emission map.png`);
    let emissionMapImage = null;
    if (emissionMapRes.ok) {
        emissionMapImage = await ImageJS.load(await emissionMapRes.arrayBuffer())
    }

    // core stuff
    const image = wasm.Image.new(width, height);
    const objHandler1 = image.new_object();

    model.v.forEach(v => {
        image.add_object_vertex(objHandler1, v.x, v.y, v.z)
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

    if (use_diffuse_map && diffuseMapImage) {
        image.set_object_texture_size(objHandler1, 1, diffuseMapImage.width, diffuseMapImage.height);
        const diffuseTexturePixelsPtr = image.get_object_texture_pixels(objHandler1, 1);
        const diffuseTexturePixels = new Uint8ClampedArray(wasm_bg.memory.buffer, diffuseTexturePixelsPtr, diffuseMapImage.width * diffuseMapImage.height * 4);
        diffuseTexturePixels.set(diffuseMapImage.getRGBAData({clamped: true}));
        image.set_object_use_texture(objHandler1, 1, true);
        console.log('using diffuse map');
    } else {
        image.set_object_use_texture(objHandler1, 1, false);
        console.log('not using diffuse map');
    }

    if (use_normal_map && normalMapImage) {
        image.set_object_texture_size(objHandler1, 2, normalMapImage.width, normalMapImage.height);
        const normalTexturePixelsPtr = image.get_object_texture_pixels(objHandler1, 2);
        const normalTexturePixels = new Uint8ClampedArray(wasm_bg.memory.buffer, normalTexturePixelsPtr, normalMapImage.width * normalMapImage.height * 4);
        normalTexturePixels.set(normalMapImage.getRGBAData({clamped: true}));
        image.set_object_use_texture(objHandler1, 2, true);
        console.log('using normal map');
    } else {
        image.set_object_use_texture(objHandler1, 2, false);
        console.log('not using normal map');
    }

    if (use_specular_map && specularMapImage) {
        image.set_object_texture_size(objHandler1, 3, specularMapImage.width, specularMapImage.height);
        const specularTexturePixelsPtr = image.get_object_texture_pixels(objHandler1, 3);
        const specularTexturePixels = new Uint8ClampedArray(wasm_bg.memory.buffer, specularTexturePixelsPtr, specularMapImage.width * specularMapImage.height * 4);
        specularTexturePixels.set(specularMapImage.getRGBAData({clamped: true}));
        image.set_object_use_texture(objHandler1, 3, true);
        console.log('using specular map');
    } else {
        image.set_object_use_texture(objHandler1, 3, false);
        console.log('not using specular map');
    }

    if (use_emission_map && emissionMapImage) {
        image.set_object_texture_size(objHandler1, 4, emissionMapImage.width, emissionMapImage.height);
        const emissionTexturePixelsPtr = image.get_object_texture_pixels(objHandler1, 4);
        const emissionTexturePixels = new Uint8ClampedArray(wasm_bg.memory.buffer, emissionTexturePixelsPtr, emissionMapImage.width * emissionMapImage.height * 4);
        emissionTexturePixels.set(emissionMapImage.getRGBAData({clamped: true}));
        image.set_object_use_texture(objHandler1, 4, true);
        console.log('using emission map');
    } else {
        image.set_object_use_texture(objHandler1, 4, false);
        console.log('not using emission map');
    }

    // fps stuff
    let lastLoop = new Date();
    let fpsLabel = document.getElementById("fps-label");

    const pixelsPtr = image.get_pixels();
    const pixels = new Uint8ClampedArray(wasm_bg.memory.buffer, pixelsPtr, width * height * 4);

    // drawing on canvas
    const canvas = document.getElementById("viewer-canvas");
    canvas.height = height;
    canvas.width = width;

    const ctx = canvas.getContext('2d');

    const render = () => {
        const palette = new ImageData(pixels, width, height);
        ctx.putImageData(palette, 0, 0);
    }

    image.compute();
    render();

    let angle = 0;
    let pressedKeys = {};

    const rotationLoop = () => {
        if (model_rotation) {
            angle += rotation_speed;
            image.set_object_rotation(objHandler1, 0, angle, 0);
        } 
        if (!Object.values(pressedKeys).every(v => !v) || model_rotation) {
            image.compute();
            render();
            
            var thisLoop = new Date();
            var fps = 1000 / (thisLoop - lastLoop);
            lastLoop = thisLoop;
            fpsLabel.innerHTML = Math.round(fps);
        }
        requestAnimationFrame(rotationLoop);
    }

    requestAnimationFrame(rotationLoop);

    document.addEventListener('keyup', (e) => {
        pressedKeys[e.code] = false;
        switch (e.code) {
            case "KeyA": // left
            case "KeyD": // right
                image.set_camera_param(1, 0);
                break;
            case "KeyQ": // up
            case "KeyE": // down
                image.set_camera_param(2, 0);
                break;
            case "KeyW": // toward
            case "KeyS": // backward
                image.set_camera_param(3, 0);
                break;
            case "ArrowUp":
            case "ArrowDown":
                image.set_camera_param(11, 0);
                break;
            case "ArrowRight":
            case "ArrowLeft":
                image.set_camera_param(12, 0);
                break;
        }
    });

    document.addEventListener('keydown', (e) => {
        if (pressedKeys[e.code])
            return;
        pressedKeys[e.code] = true;

        switch (e.code) {
            case "KeyD": // right
                image.set_camera_param(1, camera_speed);
                break;
            case "KeyA": // left
                image.set_camera_param(1, -camera_speed);
                break;
            case "KeyQ": // up
                image.set_camera_param(2, camera_speed);
                break;
            case "KeyE": // down
                image.set_camera_param(2, -camera_speed);
                break;
            case "KeyW": // toward
                image.set_camera_param(3, camera_speed);
                break;
            case "KeyS": // backward
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
            case "Space":
                model_rotation = !model_rotation;
                break;
        }
        image.compute()
        render();
    });
}
