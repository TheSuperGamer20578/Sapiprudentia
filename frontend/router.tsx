import React from "react";
import {createRoot} from "react-dom/client";
import {
    createBrowserRouter,
    createRoutesFromChildren,
    matchRoutes,
    RouterProvider,
    Routes,
    useLocation, useNavigationType
} from "react-router-dom";
import LayoutMain from "./templates/main";
import Index from "./routes";
import ErrorPage from "./routes/error";
import EditorPage from "./routes/editorPage";
import {AuthProvider} from "./auth";
import SubjectsPage from "./routes/subjects";
import SubjectPage from "./routes/subjectPage";
import Todos from "./routes/todos";
import {getWebInstrumentations, initializeFaro} from "@grafana/faro-web-sdk";
import {Faro, FaroErrorBoundary, ReactIntegration, ReactRouterVersion} from "@grafana/faro-react";
import {TracingInstrumentation} from "@grafana/faro-web-tracing";

declare global {
    interface Window {
        faro: Faro;
    }
}

initializeFaro({
    url: "https://faro-collector-prod-au-southeast-1.grafana.net/collect/cc9facb7b41b31a05af57dc11c34460f",
    app: {
        name: "Sapiprudentia",
    },
    instrumentations: [
        ...getWebInstrumentations(),
        new TracingInstrumentation(),
        new ReactIntegration({
            router: {
                version: ReactRouterVersion.V6,
                dependencies: {
                    createRoutesFromChildren,
                    matchRoutes,
                    Routes,
                    useLocation,
                    useNavigationType,
                },
            },
        }),
    ],
});

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
        <FaroErrorBoundary>
            <AuthProvider>
                <RouterProvider router={router} />
            </AuthProvider>
        </FaroErrorBoundary>
    </React.StrictMode>
);
