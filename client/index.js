import { Image } from "3d_engine_core";
import { memory } from "3d_engine_core/graphics_engine_core_bg";

const width = window.innerWidth;
const height = window.innerHeight;

const params = new URLSearchParams(window.location.search);

const model_name = params.has('model-name') ? params.get('model-name') : "Model";

const camera_speed = params.has('camera-speed') ? params.get('camera-speed') : 0.05;

const rotation_speed = params.has('rotation-speed') ? params.get('rotation-speed') : 0.1;

const model_scale = params.has('model-scale') ? params.get('model-scale') : 0.1;

let model_rotation = params.has('model-rotation') ? params.get('model-rotation') === "true" : false;

// TODO: add color-params to query

(async () => {
    // data fetching
    let model;
    let modelRes = await fetch(`/raw/model?model-name=${model_name}`);
    if (modelRes.ok) {
        model = await modelRes.json();
    } else {
        alert("Cannot fetch model");
        return;
    }

    // core stuff
    const image = Image.new(width, height);

    // gismo
    // const objHandler0 = image.new_object();
    
    // image.add_object_vertex(objHandler0, 0, 0, 0, 1);
    // image.add_object_vertex(objHandler0, 1, 0, 0, 1);
    // image.add_object_vertex(objHandler0, 0, 1, 0, 1);
    // image.add_object_vertex(objHandler0, 0, 0, 1, 1);

    // image.add_object_face(objHandler0, 0, 0, 0, 1, 0, 0, 2, 0, 0);
    // image.add_object_face(objHandler0, 0, 0, 0, 3, 0, 0, 1, 0, 0);
    // image.add_object_face(objHandler0, 0, 0, 0, 2, 0, 0, 3, 0, 0);

    // image.set_object_scale(objHandler0, 0.5);


    // objects
    // const objHandler1 = -1;
    const objHandler1 = image.new_object();

    model.v.forEach(v => {
        image.add_object_vertex(objHandler1, v.x, v.y, v.z)
    });

    model.vn.forEach(vn => {
        image.add_object_vertex_normal(objHandler1, vn.x, vn.y, vn.z);
    })

    model.f.forEach(f => {
        image.add_object_face(objHandler1,
            f[0].v - 1, f[0].vt - 1, (f[0].vn || 1) - 1,
            f[1].v - 1, f[1].vt - 1, (f[1].vn || 1) - 1,
            f[2].v - 1, f[2].vt - 1, (f[2].vn || 1) - 1
        );
    });

    image.set_object_scale(objHandler1, model_scale);

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
