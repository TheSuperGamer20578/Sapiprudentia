import { Configuration } from "webpack";
import "webpack-dev-server";
import MiniCssExtractPlugin from "mini-css-extract-plugin";

const config: Configuration = {
    plugins: [
        new MiniCssExtractPlugin(),
    ],
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
                use: [MiniCssExtractPlugin.loader, "css-loader", "sass-loader"],
            },
            {
                test: /\.css$/i,
                use: [MiniCssExtractPlugin.loader, "css-loader"],
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
