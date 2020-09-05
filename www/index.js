import { Image, Pixel } from "3d_engine_core";
import { memory } from "3d_engine_core/graphics_engine_core_bg";

const CELL_SIZE = 2; // px
const WHITE_COLOR = "#FFFFFF";
const BLACK_COLOR = "#000000";

const image = Image.new();
const width = image.width();
const height = image.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = height;
canvas.width = width;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
    image.tick();

    drawCells();

    requestAnimationFrame(renderLoop);
};

const getIndex = (row, column) => {
    return row * width + column;
};

const pixelsPtr = image.pixels();
const pixels = new Uint8ClampedArray(memory.buffer, pixelsPtr, width * height * 4);

const drawCells = () => {

    const palette = new ImageData(pixels, width, height);

    ctx.putImageData(palette, 0, 0);

    // ctx.beginPath();

    // for (let row = 0; row < height; row++) {
    //     for (let col = 0; col < width; col++) {
    //         const idx = getIndex(row, col);
    //
    //         ctx.fillStyle = cells[idx] === Pixel.Black
    //             ? BLACK_COLOR
    //             : WHITE_COLOR;
    //
    //         ctx.put
    //         ctx.fillRect(
    //             col * CELL_SIZE,
    //             row * CELL_SIZE,
    //             CELL_SIZE,
    //             CELL_SIZE
    //         );
    //     }
    // }

    // ctx.stroke();
};

drawCells();
requestAnimationFrame(renderLoop);