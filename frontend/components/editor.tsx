import React, {ChangeEvent, useState} from 'react';
import './editor.sass';
import {EditorProvider} from "@tiptap/react";
import {Blockquote} from "@tiptap/extension-blockquote";
import {Bold} from "@tiptap/extension-bold";
import {BubbleMenu} from "@tiptap/extension-bubble-menu";
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
        <div className="document-title-block">
            {showTitle && <input
                className="title"
                aria-label="Title"
                placeholder="Title"
                maxLength={255}
                defaultValue={doc.title}
                onInput={(e: ChangeEvent<HTMLInputElement>) => {
                    setDoc({title: e.currentTarget.value});
            }} />}
            <p className="save-indicator">{pending ? "Saving..." : "Changes Saved"}</p>
        </div>
        <EditorProvider
            extensions={[
                Blockquote,
                Bold,
                BubbleMenu,  // TODO Configure
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
                FloatingMenu,  // TODO Configure
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
            autofocus={autofocus}
            onUpdate={({editor}) => {
                setDoc({content: editor.getJSON()});
            }}
            content={doc.content}
            onCreate={() => setLoading(false)}
        >
        </EditorProvider>
        {loading && <Loader />}
    </>;
}
