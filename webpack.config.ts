import { Configuration } from "webpack";
import "webpack-dev-server";

const config: Configuration = {
    entry: "./frontend/index.ts",
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: "ts-loader",
                exclude: /node_modules/,
            },
            {
                test: /\.s[ac]ss$/i,
                use: ["style-loader", "css-loader", "sass-loader"],
            },
        ],
    },
    resolve: {
        extensions: [".ts", ".js"],
    },
    mode: "development",
    devServer: {
        devMiddleware: {
            publicPath: "/static/",
        },
        proxy: {
            "/": "http://localhost:8000",
        },
    },
};

export default config;
