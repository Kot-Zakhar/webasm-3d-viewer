import { getCiphers } from 'crypto';
import express from 'express';
import fs from 'fs';

const rawApi = express.Router();

rawApi.get('/model', (req, res) => {
    let modelName = req.query["model-name"];
    console.log(modelName);
    modelName == modelName || "Model";

    let model: any = {};
    const rawLines = fs.readFileSync(`public/source/${modelName}/Model.obj`).toString()
        .split('\n')
        .map(line => line.split(' '));

    model.v = rawLines
    .filter(line => line[0] === "v")
    .map(line => ({ x: parseFloat(line[1]), y: parseFloat(line[2]), z: parseFloat(line[3]) }));

    model.vn = rawLines
    .filter(line => line[0] === "vn")
    .map(line => ({ x: parseFloat(line[1]), y: parseFloat(line[2]), z: parseFloat(line[3]) }));


    const faces = rawLines.filter(line => line[0] === "f");
    // TODO: indexes must be >= 0 - fix, if negative

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

    model.f = normalizedFaces;

    res.json(model);
});

export default rawApi;