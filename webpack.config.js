const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");
const jsPath = path.resolve(__dirname, "js");
module.exports = (env, argv) => {
  return {
    devServer: {
      static: {
        directory: distPath,
      },
      compress: argv.mode === 'production',
			historyApiFallback: true,
      port: 8000
    },
    entry: './bootstrap.js',
    output: {
      path: distPath,
      filename: "polymesh_yew.js",
      webassemblyModuleFilename: "polymesh_yew.wasm"
    },
    experiments: {
      asyncWebAssembly: true,
      syncWebAssembly: true
    },
    module: {
      rules: [
        {
          test: /\.(?:js|mjs|cjs)$/,
          exclude: /node_modules/,
          use: {
            loader: 'babel-loader',
            options: {
              presets: [
                ['@babel/preset-env', { targets: "defaults" }]
              ]
            }
          }
        },
        {
          test: /\.s[ac]ss$/i,
          use: [
            'style-loader',
            'css-loader',
            'sass-loader',
          ],
        },
      ],
    },
    plugins: [
      new CopyWebpackPlugin({
        patterns: [
          { from: './static', to: distPath },
        ],
      }),
      new WasmPackPlugin({
        crateDirectory: ".",
				watchDirectories: [jsPath],
        extraArgs: "--no-typescript",
      })
    ],
    watch: argv.mode !== 'production'
  };
};
