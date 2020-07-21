// Never remove this line
const path = require("path");

const WebpackBar = require("webpackbar");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");

// Webpack generates `css_classes.rs` with this config.
// This config is used in command `yarn generate:css_classes`.
// See `webpack.config.js` for more info about individual settings.

module.exports = (env, argv) => {
    return {
        entry: path.resolve(__dirname, "./index.tailwind.ts"),
        // output: {
        //     path: path.resolve(__dirname, "./dist"),
        //     filename: "css_classes.js"
        // },
        mode: argv.mode,
        plugins: [
            new WebpackBar(),
            new CleanWebpackPlugin({ cleanAfterEveryBuildPatterns: ["main.js", "dist"] })
        ],
        module: {
            rules: [
                {
                    test: /\.css$/,
                    use: [
                        "style-loader",
                        "css-loader",
                        {
                            loader: "postcss-loader",
                            options: {
                                ident: 'postcss',
                                options: {
                                    config: {
                                        path: __dirname
                                    }
                                },
                                plugins: [
                                    require("postcss-import"),
                                    require("tailwindcss"),
                                    require("postcss-typed-css-classes")({
                                        generator: "rust",
                                        purge: argv.mode === "production",
                                        output_filepath: "../src/generated/tailwind_classes.rs",
                                    }),
                                    require("autoprefixer")
                                ]
                            }
                        }
                    ]
                }
            ]
        }
    };
};
