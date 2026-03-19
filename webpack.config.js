const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

module.exports = {
    mode: 'development',
    entry: './bootstrap.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'bootstrap.js',
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: __dirname,
        }),
        new CopyWebpackPlugin({
            patterns: [
                { from: 'static', to: '.' }
            ]
        })
    ],
    experiments: {
        asyncWebAssembly: true
    }
};