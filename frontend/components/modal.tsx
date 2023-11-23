import React from "react";
import ReactModal from "react-modal";
import {FaWindowClose} from "react-icons/fa";
import styles from "./modal.module.sass";

interface Props {
    open: boolean;
    setOpen: (open: boolean) => void;
    children: React.ReactNode;
}
export default function Modal({open, setOpen, children}: Props) {
    return (
        <ReactModal
            className={styles.modal}
            overlayClassName={styles.overlay}
            isOpen={open}
            onRequestClose={() => setOpen(false)}
        >
            <button className={styles.close} onClick={() => setOpen(false)}><FaWindowClose /></button>
            {children}
        </ReactModal>
    );
}
