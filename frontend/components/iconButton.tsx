import React from "react";
import {ReactNode} from "react";
import styles from "./iconButton.module.sass";

interface Props {
    title: string;
    onClick: () => void;
    children: ReactNode;
}

export default function IconButton({title, onClick, children}: Props) {
    return <button title={title} className={styles.button} onClick={onClick}>{children}</button>;
}
