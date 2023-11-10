import React from "react";
import {createRoot} from "react-dom/client";
import {createBrowserRouter, RouterProvider} from "react-router-dom";
import LayoutMain from "./templates/main";
import Index from "./routes";
import ErrorPage from "./routes/error";
import EditorPage from "./routes/editorPage";

const router = createBrowserRouter([
    {
        path: "/",
        element: <LayoutMain />,
        errorElement: <ErrorPage />,
        children: [
            {
                index: true,
                element: <Index />,
            },
            {
                path: "/document/:id",
                element: <EditorPage />,
            }
        ],
    },
]);

createRoot(document.getElementById("app")!).render(
    <React.StrictMode>
        <RouterProvider router={router} />
    </React.StrictMode>
);
