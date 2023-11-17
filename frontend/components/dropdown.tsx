import React from "react";
import {ReactNode, useState} from "react";
import styles from "./dropdown.module.sass";

interface Props {
    button: ReactNode;
    children: ReactNode;
}

export default function Dropdown({button, children}: Props) {
    let [open, setOpen] = useState(false);

    return <div
        className={styles.container}
        onBlur={(e) => {
            if (!e.currentTarget.contains(e.relatedTarget)) {
                setOpen(false);
            }
        }
    }>
        <button className={styles.button} onClick={() => setOpen(!open)}>{button}</button>
        {open && <div className={styles.menu} tabIndex={-1}>{children}</div>}
    </div>;
}
