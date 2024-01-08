import { Configuration } from "webpack";
import "webpack-dev-server";
import MiniCssExtractPlugin from "mini-css-extract-plugin";
import WebpackCopyPlugin from "copy-webpack-plugin";
import WorkboxPlugin from "workbox-webpack-plugin";

const config: Configuration = {
    plugins: [
        new MiniCssExtractPlugin(),
        new WebpackCopyPlugin({
            patterns: [
                "./static/",
            ],
        }),
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
        headers: {
            "Service-Worker-Allowed": "/",
        }
    },
    devtool: "source-map",
};

if (!process.env.SKIP_SERVICEWORKER) {
    config.plugins!.push(new WorkboxPlugin.InjectManifest({
        swSrc: "./frontend/serviceworker.ts",
        injectionPoint: "WB_MANIFEST",
        maximumFileSizeToCacheInBytes: 1024**3,  // 1GB
    }));
}

export default config;
