import React from "react";
import {createRoot} from "react-dom/client";
import {createBrowserRouter, RouterProvider} from "react-router-dom";
import LayoutMain from "./templates/main";

const router = createBrowserRouter([
    {
        path: "/react/",
        element: <LayoutMain />,
    },
]);

createRoot(document.getElementById("app")!).render(
    <React.StrictMode>
        <RouterProvider router={router} />
    </React.StrictMode>
);
