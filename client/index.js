import { Image, Pixel } from "3d_engine_core";
import { memory } from "3d_engine_core/graphics_engine_core_bg";

const CELL_SIZE = 2; // px
const WHITE_COLOR = "#FFFFFF";
const BLACK_COLOR = "#000000";

const width = 512;
const height = 512;

(async () => {
    // data fetching
    let rawVertexes = [];
    let vertexRes = await fetch("http://localhost:5000/raw/v");
    console.log(vertexRes);
    if (vertexRes.ok) {
        rawVertexes = await vertexRes.json();
        console.log(rawVertexes);
    } else {
        alert("Cannot fetch vertexes");
        return;
    }

    // core stuff
    const image = Image.new(width, height, rawVertexes.length);

    const pixelsPtr = image.pixels();
    const pixels = new Uint8ClampedArray(memory.buffer, pixelsPtr, width * height * 4);

    const vertexesPtr = image.vertexes();
    const vertexes = new Float64Array(memory.buffer, vertexesPtr, rawVertexes.length * 4);
    rawVertexes.forEach((rawVertex, i) => {
        vertexes[i * 4] = rawVertex.x;
        vertexes[i * 4 + 1] = rawVertex.y;
        vertexes[i * 4 + 2] = rawVertex.z;
        vertexes[i * 4 + 3] = rawVertex.w;
    });

    console.log(vertexes);
    image.translate_to_camera();

    image.map_view_on_image();

    const viewVertexesPtr = image.view_vertexes();
    const viewVertexes = new Float64Array(memory.buffer, viewVertexesPtr, rawVertexes.length * 4);
    console.log(viewVertexes);

    // let range = {
    //     xmin: 0,
    //     xmax: 0,
    //     ymin: 0,
    //     ymax: 0,
    //     zmin: 0,
    //     zmax: 0,                
    // }
    // for (let i = 0; i < viewVertexes.length; i+=4) {
    //     if (viewVertexes[i] < range.xmin)
    //         range.xmin = viewVertexes[i];
    //     if (viewVertexes[i] > range.xmax)
    //         range.xmax = viewVertexes[i];

    //     if (viewVertexes[i+1] < range.ymin)
    //         range.ymin = viewVertexes[i+1];
    //     if (viewVertexes[i+1] > range.ymax)
    //         range.ymax = viewVertexes[i+1];

    //     if (viewVertexes[i+2] < range.zmin)
    //         range.zmin = viewVertexes[i+2];
    //     if (viewVertexes[i+2] > range.zmax)
    //         range.zmax = viewVertexes[i+2];
    // }
    // console.log(range);

    const canvas = document.getElementById("game-of-life-canvas");
    canvas.height = height;
    canvas.width = width;

    const ctx = canvas.getContext('2d');
    const palette = new ImageData(pixels, width, height);
    ctx.putImageData(palette, 0, 0);

    console.log(pixels);

    console.log(pixels.filter(p => p != 255).length);
})()



// drawing stuff
// const canvas = document.getElementById("game-of-life-canvas");
// canvas.height = height;
// canvas.width = width;

// const ctx = canvas.getContext('2d');

// const render = () => {

//     const palette = new ImageData(pixels, width, height);

//     ctx.putImageData(palette, 0, 0);

// };

// const renderLoop = () => {
//     image.tick();
//     render();
//     requestAnimationFrame(renderLoop);
// };

// render();
// start loop
// requestAnimationFrame(renderLoop);