import React from "react";
import {ClimbingBoxLoader} from "react-spinners";
import styles from "./loader.module.sass";

export default function Loader() {
    return <ClimbingBoxLoader className={styles.loader} />;
}
