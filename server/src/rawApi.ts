import { getCiphers } from 'crypto';
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

rawApi.get('/f_raw', (req, res) => {
    // TODO: faces must always have only 3 vertexes - split if more
    // TODO: indexes must be >= 0 - fix, if negative
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

rawApi.get('/f', (req, res) => {
    const rawLines = fs.readFileSync('public/Model.obj').toString()
        .split('\n')
        .map(line => line.split(' '));

    const faces = rawLines.filter(line => line[0] === "f");

    const normalizedFaces: any[] = [];

    faces.forEach(line => {
        
        line.shift();
        let parsedline = line.map(vertex => vertex.split('/'));
        for (let i = 2; i < parsedline.length; i++) {
            let v0 = parsedline[0];
            let v1 = parsedline[i - 1];
            let v2 = parsedline[i];
            let normFace = [
                {v: v0[0], vt: v0[1], vn: v0[2] ?? undefined},
                {v: v1[0], vt: v1[1], vn: v1[2] ?? undefined},
                {v: v2[0], vt: v2[1], vn: v2[2] ?? undefined}
            ];

            normalizedFaces.push(normFace);
        }
    })

    res.json(normalizedFaces);
    
})

export default rawApi;