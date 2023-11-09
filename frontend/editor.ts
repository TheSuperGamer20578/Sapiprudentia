import {Editor} from "@tiptap/core";
import {Blockquote} from "@tiptap/extension-blockquote";
import {Bold} from "@tiptap/extension-bold";
import {BulletList} from "@tiptap/extension-bullet-list";
import {Code} from "@tiptap/extension-code";
import {CodeBlockLowlight} from "@tiptap/extension-code-block-lowlight";
import {Dropcursor} from "@tiptap/extension-dropcursor";
import {Gapcursor} from "@tiptap/extension-gapcursor";
import {HardBreak} from "@tiptap/extension-hard-break";
import {Heading} from "@tiptap/extension-heading";
import {History} from "@tiptap/extension-history";
import {HorizontalRule} from "@tiptap/extension-horizontal-rule";
import {Image} from "@tiptap/extension-image";
import {Italic} from "@tiptap/extension-italic";
import {Link} from "@tiptap/extension-link";
import {ListItem} from "@tiptap/extension-list-item";
import {ListKeymap} from "@tiptap/extension-list-keymap";
import {OrderedList} from "@tiptap/extension-ordered-list";
import {Paragraph} from "@tiptap/extension-paragraph";
import {Placeholder} from "@tiptap/extension-placeholder";
import {Strike} from "@tiptap/extension-strike";
import {Subscript} from "@tiptap/extension-subscript";
import {Superscript} from "@tiptap/extension-superscript";
import {Table} from "@tiptap/extension-table";
import {TableCell} from "@tiptap/extension-table-cell";
import {TableHeader} from "@tiptap/extension-table-header";
import {TableRow} from "@tiptap/extension-table-row";
import {TaskItem} from "@tiptap/extension-task-item";
import {TaskList} from "@tiptap/extension-task-list";
import {Text} from "@tiptap/extension-text";
import {Typography} from "@tiptap/extension-typography";
import {Underline} from "@tiptap/extension-underline";
import {Document} from "@tiptap/extension-document";
import {all, createLowlight} from "lowlight";
import {Details} from "@tiptap-pro/extension-details";
import {DetailsContent} from "@tiptap-pro/extension-details-content";
import {DetailsSummary} from "@tiptap-pro/extension-details-summary";
import {Emoji} from "@tiptap-pro/extension-emoji";
import {Mathematics} from "@tiptap-pro/extension-mathematics";
import {TableOfContent} from "@tiptap-pro/extension-table-of-content";
import {FileHandler} from "@tiptap-pro/extension-file-handler";
import {BubbleMenu} from "@tiptap/extension-bubble-menu";
import {FloatingMenu} from "@tiptap/extension-floating-menu";
import {Highlight} from "@tiptap/extension-highlight";
// @ts-ignore
import "katex/contrib/mhchem/mhchem.js";
import {getDocument, queueChanges, updateDocument} from "./api";


export async function initEditor(editorElement: Element, titleElement: HTMLElement, documentId: number) {
    const document_ = await getDocument(documentId);

    titleElement.innerText = document_.title;

    // TODO Slash commands
    new Editor({
        element: editorElement,
        extensions: [
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
        ],
        injectCSS: false,
        autofocus: document_.title.length !== 0,
        content: document_.content,
        onUpdate: ({editor}) => {
            queueChanges(updateDocument, documentId, {
                content: editor.getJSON(),
            })
        },
    });

    titleElement.addEventListener("input", (event: any) => {
        document.querySelectorAll(`[data-title-of="${documentId}"]`).forEach((element) => {
            element.textContent = event.currentTarget.value.trim() || "Untitled";
        });
        queueChanges(updateDocument, documentId, {
            title: event.currentTarget.value.trim(),
        });
    });
}
