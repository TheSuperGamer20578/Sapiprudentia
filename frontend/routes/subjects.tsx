import {createSubject, listSubjects, updateSubject} from "../api";
import LoadingOverlay from "../components/loadingOverlay";
import React, {useState} from "react";
import {useSyncMany} from "../sync";
import {Link} from "react-router-dom";
import styles from "./subjects.module.sass";
import Modal from "../components/modal";
import Form from "../components/form";
import IconButton from "../components/iconButton";
import {FaPlus} from "react-icons/fa";
import Corner from "../components/corner";

export default function Subjects() {
    const {
        value: subjects,
        add: addSubject,
    } = useSyncMany(listSubjects, updateSubject, createSubject, 0);
    const [createModalOpen, setCreateModalOpen] = useState(false);
    const [createName, setCreateName] = useState("");
    const [createClass, setCreateClass] = useState("");

    if (subjects === undefined) {
        return <LoadingOverlay force={true} />;
    }
    return <>
        <Modal open={createModalOpen} setOpen={setCreateModalOpen}>
            <Form
                title="Create Subject"
                submitText="Create"
                submit={() => {
                    setCreateModalOpen(false);
                    addSubject({
                        name: createName,
                        class: createClass,
                        active: true,
                        google_classroom_id: null,
                    });
                }}
            >
                <label>
                    <p>Subject:</p>
                    <input
                        type="text"
                        required
                        maxLength={255}
                        onInput={(e) => setCreateName(e.currentTarget.value)}
                    />
                </label>
                <label>
                    <p>Class:</p>
                    <input
                        type="text"
                        required
                        maxLength={16}
                        onInput={(e) => setCreateClass(e.currentTarget.value)}
                    />
                </label>
            </Form>
        </Modal>
        <Corner corner={<IconButton onClick={() => setCreateModalOpen(true)}><FaPlus /></IconButton>}>
            <h1>Subjects</h1>
            <ul className={styles.subjectList}>
                {Array.from(subjects.values()).map((subject) =>
                    <li key={subject.id}>
                        <Link to={`/subjects/${subject.id}`} className={subject.active && styles.active}>
                            <p>{subject.name}</p>
                            <p className={styles.class}>{subject.class}</p>
                        </Link>
                    </li>
                )}
            </ul>
        </Corner>
    </>;
}