import { Image } from "3d_engine_core";
import { memory } from "3d_engine_core/graphics_engine_core_bg";

const width = window.innerWidth;
const height = window.innerHeight;

const camera_speed = 0.1;

let scale = 0.0;

(async () => {
    // data fetching
    let rawVertexes = [];
    let vertexRes = await fetch("/raw/v");
    if (vertexRes.ok) {
        rawVertexes = await vertexRes.json();
    } else {
        alert("Cannot fetch vertexes");
        return;
    }

    let rawFaces = [];
    let facesRes = await fetch("/raw/f");
    if (facesRes.ok) {
        rawFaces = await facesRes.json();
    } else {
        console.log("Could not get faces.");
    }

    // core stuff
    const image = Image.new(width, height);

    // gismo
    // const objHandler0 = image.new_object();
    const objHandler0 = -1;
    
    image.add_object_vertex(objHandler0, 0, 0, 0, 1);
    image.add_object_vertex(objHandler0, 1, 0, 0, 1);
    image.add_object_vertex(objHandler0, 0, 1, 0, 1);
    image.add_object_vertex(objHandler0, 0, 0, 1, 1);

    image.add_object_face(objHandler0, 0, 0, 0, 1, 0, 0, 2, 0, 0);
    image.add_object_face(objHandler0, 0, 0, 0, 3, 0, 0, 1, 0, 0);
    image.add_object_face(objHandler0, 0, 0, 0, 2, 0, 0, 3, 0, 0);

    image.set_object_scale(objHandler0, 0.5);

    // objects
    // const objHandler1 = -1;
    const objHandler1 = image.new_object();

    // const objHandler2 = image.new_object();
    // const objHandler3 = image.new_object();

    rawVertexes.forEach(v => {
        image.add_object_vertex(objHandler1, v.x, v.y, v.z, v.w)
        // image.add_vertex(objHandler2, v.x, v.y, v.z, v.w)
        // image.add_vertex(objHandler3, v.x, v.y, v.z, v.w)
    });

    rawFaces.forEach(f => {
        image.add_object_face(objHandler1,
            f[0].v - 1, f[0].vt - 1, (f[0].vn || 1) - 1,
            f[1].v - 1, f[1].vt - 1, (f[1].vn || 1) - 1,
            f[2].v - 1, f[2].vt - 1, (f[2].vn || 1) - 1
        );
    });

    image.set_object_scale(objHandler1, 0.5);
    // image.set_object_scale(objHandler1, 10);
    // image.set_object_translaiton(objHandler1, 0, 1, 0);

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
    let loop = false;

    const rotationLoop = () => {
        if (loop) {
            angle += 0.01;
            image.set_object_rotation(objHandler1, 0, angle, 0);
        }
        image.compute();
        render();
        requestAnimationFrame(rotationLoop);
        
        var thisLoop = new Date();
        var fps = 1000 / (thisLoop - lastLoop);
        lastLoop = thisLoop;
        fpsLabel.innerHTML = Math.round(fps);
    }

    requestAnimationFrame(rotationLoop);

    let pressedKeys = {};

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
                image.set_camera_param(1, 0.01);
                break;
            case "KeyA": // left
                image.set_camera_param(1, -0.01);
                break;
            case "KeyQ": // up
                image.set_camera_param(2, 0.01);
                break;
            case "KeyE": // down
                image.set_camera_param(2, -0.01);
                break;
            case "KeyW": // toward
                image.set_camera_param(3, 0.01);
                break;
            case "KeyS": // backward
                image.set_camera_param(3, -0.01);
                break;
            case "ArrowUp":
                image.set_camera_param(11, -0.01);
                break;
            case "ArrowDown":
                image.set_camera_param(11, 0.01);
                break;
            case "ArrowRight":
                image.set_camera_param(12, 0.01);
                break;
            case "ArrowLeft":
                image.set_camera_param(12, -0.01);
                break;
            case "Space":
                loop = !loop;
                break;
                
        }
        image.compute()
        render();
    });
})()
