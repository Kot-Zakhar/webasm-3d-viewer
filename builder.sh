#!/bin/bash
cd core

webasm-pack build

cd ../client

npm ci

npm run build

cd ../server

npm run start