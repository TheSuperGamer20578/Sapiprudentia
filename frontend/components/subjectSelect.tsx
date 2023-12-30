import {useEffect, useState} from "react";
import {listSubjects, Subject} from "../api";
import React from "react";

interface Props {
    required: boolean;
    defaultValue?: number;
    onInput: (subject: number) => void;
}

export default function SubjectSelect({required, defaultValue, onInput}: Props) {
    const [subjects, setSubjects] = useState<Subject[] | null>(null);
    useEffect(() => {
        listSubjects().then(setSubjects);
    }, []);

    if (subjects === null) {
        return <></>;
    }

    return (
        <select
            required={required}
            defaultValue={defaultValue ?? "null"}
            onInput={(e) => {
                onInput(parseInt(e.currentTarget.value));
            }}
        >
            <option value="null" disabled={required}>{required ? "Pick a subject" : "None"}</option>
            {subjects.map((subject) => (
                <option key={subject.id} value={subject.id}>{subject.name}</option>
            ))}
        </select>
    );
}