import React, {ReactNode} from "react";
import styles from "./chip.module.sass";

interface Props {
    children: ReactNode;
}

export default function Chip({children}: Props) {
    return <div className={styles.chip}>{children}</div>;
}
