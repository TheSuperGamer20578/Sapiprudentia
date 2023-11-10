import React from "react";
import {isRouteErrorResponse, useRouteError} from "react-router-dom";
import styles from "./error.module.sass";

export default function ErrorPage() {
    const error = useRouteError();
    console.error(error);
    let message;
    if (isRouteErrorResponse(error)) {
        message = `${error.status} ${error.statusText}`;
    } else if (error instanceof Error) {
        message = (error as Error).message;
    } else if (typeof error === "string") {
        message = error;
    } else {
        message = "Unknown Error";
    }

    return (
        <div className={styles.error}>
            <h1>Oops!</h1>
            <p>Sorry, an unexpected error has occurred.</p>
            <p className={styles.message}>{message}</p>
        </div>
    );
}
