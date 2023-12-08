import React, {ReactNode} from "react";
import styles from "./corner.module.sass";

interface Props {
    corner: ReactNode;
    children: ReactNode;
}

export default function Corner({corner, children}: Props) {
    return <div className={styles.container}>
        <div className={styles.corner}>
            {corner}
        </div>
        {children}
    </div>;
}
