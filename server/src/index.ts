import express from 'express';
import rawApi from './rawApi';

let server = express();

server.use(express.static('public'));
server.use(express.static('../client/dist'));

server.use('/raw', rawApi);

server.listen(5000, () => console.log("Server is listening on 5000 port"));