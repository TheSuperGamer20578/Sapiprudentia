import React from "react";
import {ReactNode} from "react";
import styles from "./iconButton.module.sass";

interface Props {
    onClick: () => void;
    children: ReactNode;
}

export default function IconButton({onClick, children}: Props) {
    return <button className={styles.button} onClick={onClick}>{children}</button>;
}
