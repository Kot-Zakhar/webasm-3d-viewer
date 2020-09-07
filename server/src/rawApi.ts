import express from 'express';
import fs from 'fs';

const rawApi = express.Router();

rawApi.get('/v', (req, res) => {

    const rawLines = fs.readFileSync('public/Model.obj').toString()
        .split('\n')
        .map(line => line.split(' '));

    const vertexes = rawLines
        .filter(line => line[0] === "v")
        .map(line => ({x: line[1], y: line[2], z: line[3], w: line[4] || 1}));
        
    res.json(vertexes);
})

rawApi.get('/f', (req, res) => {
    const rawLines = fs.readFileSync('public/Model.obj').toString()
        .split('\n')
        .map(line => line.split(' '));

    const faces = rawLines
        .filter(line => line[0] === "f")
        .map(line => {
            line.shift();
            return line.map(vertex => {
                let parsedVertex = vertex.split('/');
                return {v: parsedVertex[0], vt: parsedVertex[1], vn: parsedVertex[2] ?? undefined};
            })
        });

    res.json(faces);
})

export default rawApi;