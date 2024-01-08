import React from "react";
import {createRoot} from "react-dom/client";
import {createBrowserRouter, RouterProvider} from "react-router-dom";
import LayoutMain from "./templates/main";
import Index from "./routes";
import ErrorPage from "./routes/error";
import EditorPage from "./routes/editorPage";
import {AuthProvider} from "./auth";
import SubjectsPage from "./routes/subjects";
import SubjectPage from "./routes/subjectPage";
import Todos from "./routes/todos";

try {
    await navigator.serviceWorker.register("/static/serviceworker.js", {scope: "/"});
} catch (e) {
    console.warn("Error registering service worker:", e);
}

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
            },
            {
                path: "/subjects",
                element: <SubjectsPage />,
            },
            {
                path: "/subjects/:id",
                element: <SubjectPage />,
            },
            {
                path: "/todo",
                element: <Todos />,
            },
        ],
    },
]);

createRoot(document.getElementById("app")!).render(
    <React.StrictMode>
        <AuthProvider>
            <RouterProvider router={router} />
        </AuthProvider>
    </React.StrictMode>
);
