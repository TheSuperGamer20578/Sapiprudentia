import React from "react";
import styles from "./centre.module.sass";

interface Props {
    children: React.ReactNode;
}

export default function Centre({children}: Props) {
    return <div className={styles.centre}>{children}</div>;
}
