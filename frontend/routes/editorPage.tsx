import {useParams} from "react-router-dom";
import Editor from "../components/editor";
import React from "react";

export default function EditorPage() {
    const params = useParams();
    const id = parseInt(params.id!);

    return <Editor documentId={id} showTitle={true} autofocus={true} />;
}