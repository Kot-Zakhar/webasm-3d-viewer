const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');
const { experiments } = require("webpack");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: "webassembly/async"
      }
    ]
  },
  experiments: {
    asyncWebAssembly: true,
  }
};
