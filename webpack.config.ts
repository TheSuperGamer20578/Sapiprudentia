import { Configuration } from "webpack";
import "webpack-dev-server";
import MiniCssExtractPlugin from "mini-css-extract-plugin";

const config: Configuration = {
    entry: {
        bundle: "./frontend/router.tsx",
    },
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
        extensions: [".ts", ".tsx", ".js", ".jsx"],
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
    devtool: "source-map",
};

export default config;
