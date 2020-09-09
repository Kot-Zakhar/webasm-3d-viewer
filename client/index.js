import { Image, Pixel } from "3d_engine_core";
import { memory } from "3d_engine_core/graphics_engine_core_bg";

const CELL_SIZE = 2; // px
const WHITE_COLOR = "#FFFFFF";
const BLACK_COLOR = "#000000";

const width = window.innerWidth;
const height = window.innerHeight;

(async () => {
    // data fetching
    let rawVertexes = [];
    let vertexRes = await fetch("/raw/v");
    // console.log(vertexRes);
    if (vertexRes.ok) {
        rawVertexes = await vertexRes.json();
        // console.log(rawVertexes);
    } else {
        alert("Cannot fetch vertexes");
        return;
    }

    let rawFaces = [];
    let facesRes = await fetch("/raw/f");
    // console.log(facesRes);
    if (facesRes.ok) {
        rawFaces = await facesRes.json();
        // console.log(rawFaces);
    } else {
        console.log("Could not get faces.");
    }

    // core stuff
    // const image = Image.new(width, height, 8);
    const image = Image.new(width, height, rawVertexes.length);


    const vertexesPtr = image.vertexes();
    const vertexes = new Float64Array(memory.buffer, vertexesPtr, rawVertexes.length * 4);
    // const vertexes = new Float64Array(memory.buffer, vertexesPtr, 8 * 4);
    rawVertexes.forEach((rawVertex, i) => {
        vertexes[i * 4] = rawVertex.x;
        vertexes[i * 4 + 1] = rawVertex.y;
        vertexes[i * 4 + 2] = rawVertex.z;
        vertexes[i * 4 + 3] = rawVertex.w;
    });
    // let cube = [
    //     100.0, 100.0, 100.0, 1,
    //     100.0, 100.0, -100.0, 1,
    //     100.0, -100.0, 100.0, 1,
    //     100.0, -100.0, -100.0, 1,
    //     -100.0, 100.0, 100.0, 1,
    //     -100.0, 100.0, -100.0, 1,
    //     -100.0, -100.0, 100.0, 1,
    //     -100.0, -100.0, -100.0, 1,
    // ];
    // cube.forEach((coord, i) => vertexes[i] = coord);

    rawFaces.forEach(f => {
        image.add_face(
            f[0].v - 1, f[0].vt - 1, f[0].vn || 0,
            f[1].v - 1, f[1].vt - 1, f[1].vn || 0,
            f[2].v - 1, f[2].vt - 1, f[2].vn || 0
        )
    })
    // image.add_face(0, 0, 0, 1, 0, 0, 2, 0, 0);
    // image.add_face(0, 0, 0, 1, 0, 0, 4, 0, 0);
    // image.add_face(2, 0, 0, 1, 0, 0, 4, 0, 0);
    // image.add_face(7, 0, 0, 6, 0, 0, 5, 0, 0);
    // image.add_face(3, 0, 0, 5, 0, 0, 7, 0, 0);
    // image.add_face(3, 0, 0, 6, 0, 0, 5, 0, 0);

    image.upscale();

    const pixelsPtr = image.pixels();
    const pixels = new Uint8ClampedArray(memory.buffer, pixelsPtr, width * height * 4);

    const canvas = document.getElementById("game-of-life-canvas");
    canvas.height = height;
    canvas.width = width;

    const ctx = canvas.getContext('2d');

    const render = () => {
        const palette = new ImageData(pixels, width, height);
        ctx.putImageData(palette, 0, 0);
    }

    const rotationLoop = () => {
        image.rotate();
        image.translate_to_camera();
        image.clear_image();
        image.draw_dots_on_image();
        image.draw_lines_on_image();
        render();
        requestAnimationFrame(rotationLoop);
        // setTimeout(rotationLoop, 100);
    }

    render();
    requestAnimationFrame(rotationLoop);
})()
