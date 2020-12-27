import { Image } from "3d_engine_core";
import { Image as ImageJS} from 'image-js';
import { memory } from "3d_engine_core/graphics_engine_core_bg";

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

(async () => {
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
    const image = Image.new(width, height);
    

    // objects
    // const objHandler1 = -1;
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
        const diffuseTexturePixels = new Uint8ClampedArray(memory.buffer, diffuseTexturePixelsPtr, diffuseMapImage.width * diffuseMapImage.height * 4);
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
        const normalTexturePixels = new Uint8ClampedArray(memory.buffer, normalTexturePixelsPtr, normalMapImage.width * normalMapImage.height * 4);
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
        const specularTexturePixels = new Uint8ClampedArray(memory.buffer, specularTexturePixelsPtr, specularMapImage.width * specularMapImage.height * 4);
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
        const emissionTexturePixels = new Uint8ClampedArray(memory.buffer, emissionTexturePixelsPtr, emissionMapImage.width * emissionMapImage.height * 4);
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
    const pixels = new Uint8ClampedArray(memory.buffer, pixelsPtr, width * height * 4);

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
        console.log(e);
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
})()
