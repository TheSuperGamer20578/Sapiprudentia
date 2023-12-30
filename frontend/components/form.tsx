import React, {FormEvent} from "react";
import styles from "./form.module.sass";

interface Props {
    submit: () => void;
    title: string;
    submitText: string;
    children: React.ReactNode;
}

export default function Form({submit, title, submitText, children}: Props) {
    return (
        <form className={styles.form} onSubmit={(e: FormEvent) => {
            e.preventDefault();
            submit();
        }}>
            <h1>{title}</h1>
            {children}
            <input type="submit" value={submitText}/>
        </form>
    );
}
