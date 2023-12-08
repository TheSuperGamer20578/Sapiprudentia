import { useParams } from "react-router-dom";
import {useSync} from "../sync";
import {getSubject, updateSubject} from "../api";
import LoadingOverlay from "../components/loadingOverlay";
import React, {useState} from "react";
import {FaEdit, FaLock, FaStar} from "react-icons/fa";
import {AiOutlineStar} from "react-icons/ai";
import styles from "./subjectPage.module.sass";
import Corner from "../components/corner";
import IconButton from "../components/iconButton";

export default function SubjectPage() {
    const { id } = useParams();
    const {
        value: subject,
        pending,
        update: setSubject,
    } = useSync(parseInt(id!), getSubject, updateSubject);
    const [editable, setEditable] = useState(false);

    if (subject === undefined) {
        return <LoadingOverlay force={true} />;
    }

    return <>
        <div className={styles.block}>
            <Corner corner={
                <>
                    <IconButton onClick={() => setSubject({active: !subject?.active})}>
                        {subject.active ? <FaStar /> : <AiOutlineStar />}
                    </IconButton>
                    <IconButton onClick={() => setEditable(!editable)}>
                        {editable ? <FaLock /> : <FaEdit />}
                    </IconButton>
                </>
            }>
                {editable ? <>
                    <label>
                        <p>Subject:</p>
                        <input
                            type="text"
                            required
                            maxLength={255}
                            defaultValue={subject.name}
                            onInput={(e) => setSubject({name: e.currentTarget.value})}
                        />
                    </label>
                    <label>
                        <p>Class:</p>
                        <input
                            type="text"
                            required
                            maxLength={16}
                            defaultValue={subject.class}
                            onInput={(e) => setSubject({class: e.currentTarget.value})}
                        />
                    </label>
                </> : <>
                    <h1>{subject.name}</h1>
                    <p>{subject.class}</p>
                </>}
            </Corner>
        </div>
    </>;
}