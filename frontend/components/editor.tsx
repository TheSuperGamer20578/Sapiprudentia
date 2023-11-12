import React, {ChangeEvent, useRef, useState} from 'react';
import styles from './editor.module.sass';
import {BubbleMenu, EditorProvider, useCurrentEditor} from "@tiptap/react";
import {Blockquote} from "@tiptap/extension-blockquote";
import {Bold} from "@tiptap/extension-bold";
import {BulletList} from "@tiptap/extension-bullet-list";
import {Code} from "@tiptap/extension-code";
import {CodeBlockLowlight} from "@tiptap/extension-code-block-lowlight";
import {all, createLowlight} from "lowlight";
import {Details} from "@tiptap-pro/extension-details";
import {DetailsContent} from "@tiptap-pro/extension-details-content";
import {DetailsSummary} from "@tiptap-pro/extension-details-summary";
import {Document} from "@tiptap/extension-document";
import {Dropcursor} from "@tiptap/extension-dropcursor";
import {Emoji} from "@tiptap-pro/extension-emoji";
import {FileHandler} from "@tiptap-pro/extension-file-handler";
import {FloatingMenu} from "@tiptap/extension-floating-menu";
import {Gapcursor} from "@tiptap/extension-gapcursor";
import {HardBreak} from "@tiptap/extension-hard-break";
import {Heading} from "@tiptap/extension-heading";
import {Highlight} from "@tiptap/extension-highlight";
import {History} from "@tiptap/extension-history";
import {HorizontalRule} from "@tiptap/extension-horizontal-rule";
import {Image} from "@tiptap/extension-image";
import {Italic} from "@tiptap/extension-italic";
import {Link} from "@tiptap/extension-link";
import {ListItem} from "@tiptap/extension-list-item";
import {ListKeymap} from "@tiptap/extension-list-keymap";
import {Mathematics} from "@tiptap-pro/extension-mathematics";
import {OrderedList} from "@tiptap/extension-ordered-list";
import {Paragraph} from "@tiptap/extension-paragraph";
import {Placeholder} from "@tiptap/extension-placeholder";
import {Strike} from "@tiptap/extension-strike";
import {Subscript} from "@tiptap/extension-subscript";
import {Superscript} from "@tiptap/extension-superscript";
import {Table} from "@tiptap/extension-table";
import {TableCell} from "@tiptap/extension-table-cell";
import {TableHeader} from "@tiptap/extension-table-header";
import {TableOfContent} from "@tiptap-pro/extension-table-of-content";
import {TableRow} from "@tiptap/extension-table-row";
import {TaskItem} from "@tiptap/extension-task-item";
import {TaskList} from "@tiptap/extension-task-list";
import {Text} from "@tiptap/extension-text";
import {Typography} from "@tiptap/extension-typography";
import {Underline} from "@tiptap/extension-underline";
import {getDocument, updateDocument} from "../api";
import {useSync} from "../sync";
import Loader from "./loader";
import Centre from "./centre";
import {
    FaBold,
    FaCode,
    FaHighlighter,
    FaItalic,
    FaStrikethrough,
    FaSubscript,
    FaSuperscript,
    FaUnderline
} from "react-icons/fa";
import "highlight.js/scss/base16/onedark.scss";
import "katex/dist/katex.css";

interface Props {
    documentId: number,
    showTitle: boolean,
    autofocus: boolean,
}

export default function Editor({ documentId, showTitle, autofocus }: Props) {
    const [doc, pending, setDoc] = useSync(documentId, getDocument, updateDocument);
    const [loading, setLoading] = useState(true);

    if (doc === undefined) {
        return (
            <Centre>
                <Loader />
            </Centre>
        );
    }

    return <>
        <div className={styles.titleBlock}>
            {showTitle && <input
                className={styles.title}
                aria-label="Title"
                placeholder="Title"
                maxLength={255}
                defaultValue={doc.title}
                autoFocus={autofocus && doc.title?.length === 0}
                onInput={(e: ChangeEvent<HTMLInputElement>) => {
                    setDoc({title: e.currentTarget.value});
            }} />}
            <p className={styles.saveIndicator}>{pending ? "Saving..." : "Changes Saved"}</p>
        </div>
        <div className={styles.editorBody}>
            <EditorProvider
                extensions={[
                    Blockquote,
                    Bold,
                    BulletList,
                    Code,
                    CodeBlockLowlight.configure({
                        lowlight: createLowlight(all),
                    }),
                    Details,  // TODO Test and style
                    DetailsContent,
                    DetailsSummary,
                    Document,
                    Dropcursor,  // TODO Test
                    Emoji,
                    FileHandler,  // TODO Configure
                    Gapcursor,  // TODO Test
                    HardBreak,
                    Heading,
                    Highlight,  // TODO Style
                    History,
                    HorizontalRule,
                    Image.configure({  // TODO Test
                        allowBase64: true,
                    }),
                    Italic,
                    Link,  // TODO Style
                    ListItem,
                    ListKeymap,
                    Mathematics,
                    OrderedList,
                    Paragraph,
                    Placeholder.configure(({
                        placeholder: (props) => {
                            if (props.editor.isEmpty) {
                                return "Type Something..."
                            }
                            return props.node.type.name;
                        },
                        showOnlyCurrent: false,
                        includeChildren: true,
                    })),
                    Strike,
                    Subscript,
                    Superscript,
                    Table,  // TODO Test and style
                    TableCell,
                    TableHeader,
                    TableOfContent,  // TODO Implement
                    TableRow,
                    TaskItem.configure({
                        nested: true,
                    }),
                    TaskList,
                    Text,
                    Typography,
                    Underline,
                ]}
                injectCSS={false}
                autofocus={autofocus && doc.title?.length !== 0}
                onUpdate={({editor}) => {
                    setDoc({content: editor.getJSON()});
                }}
                content={doc.content}
                onCreate={() => setLoading(false)}
            >
                <BubbleMenu className={styles.bubbleMenu}>
                    <Bubble />
                </BubbleMenu>
            </EditorProvider>
        </div>
        {loading && <Loader/>}
    </>;
}

function Bubble() {
    const {editor} = useCurrentEditor();

    const disabled = editor === null;

    return <>
        <button
            disabled={disabled || !editor?.can().toggleBold()}
            onClick={() => editor?.chain().focus().toggleBold().run()}
            title="Bold"
            className={editor?.isActive("bold") ? styles.active : undefined}
        >
            <FaBold />
        </button>
        <button
            disabled={disabled || !editor?.can().toggleItalic()}
            onClick={() => editor?.chain().focus().toggleItalic().run()}
            title="Italic"
            className={editor?.isActive("italic") ? styles.active : undefined}
        >
            <FaItalic />
        </button>
        <button
            disabled={disabled || !editor?.can().toggleUnderline()}
            onClick={() => editor?.chain().focus().toggleUnderline().run()}
            title="Underline"
            className={editor?.isActive("underline") ? styles.active : undefined}
        >
            <FaUnderline />
        </button>
        <button
            disabled={disabled || !editor?.can().toggleStrike()}
            onClick={() => editor?.chain().focus().toggleStrike().run()}
            title="Strikethrough"
            className={editor?.isActive("strike") ? styles.active : undefined}
        >
            <FaStrikethrough />
        </button>
        <button
            disabled={disabled || !editor?.can().toggleCode()}
            onClick={() => editor?.chain().focus().toggleCode().run()}
            title="Code"
            className={editor?.isActive("code") ? styles.active : undefined}
        >
            <FaCode />
        </button>
        <button
            disabled={disabled || !editor?.can().toggleSubscript()}
            onClick={() => editor?.chain().focus().toggleSubscript().run()}
            title="Subscript"
            className={editor?.isActive("subscript") ? styles.active : undefined}
        >
            <FaSubscript />
        </button>
        <button
            disabled={disabled || !editor?.can().toggleSuperscript()}
            onClick={() => editor?.chain().focus().toggleSuperscript().run()}
            title="Superscript"
            className={editor?.isActive("superscript") ? styles.active : undefined}
        >
            <FaSuperscript />
        </button>
    </>;
}
