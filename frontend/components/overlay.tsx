import React from "react";
import styles from "./overlay.module.sass";
import Centre from "./centre";

interface Props {
    children: React.ReactNode;
}

export default function Overlay({children}: Props) {
    return (
        <div className={styles.overlay}>
            <Centre>
                {children}
            </Centre>
        </div>
    );
}
