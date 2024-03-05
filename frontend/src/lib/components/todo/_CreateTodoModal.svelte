<script lang="ts">
    import Modal from "$lib/components/Modal.svelte";
    import {graphql} from "$houdini";
    import SubjectSelect from "$lib/components/SubjectSelect.svelte";
    import Form from "$lib/components/Form.svelte";
    import {DateTime} from "luxon";

    let modal: Modal;
    let title = "";
    let subject = "null";
    let due: string | null = null;
    let standing: boolean = false;
    let showSubject = true;
    let parent: number | null = null;
    let editing: number | null = null;
    let isRoot: boolean = false;

    export let subjects: {id: number; name: string}[];

    export function createRoot(withSubject: number | null = null) {
        title = "";
        subject = JSON.stringify(withSubject);
        due = null;
        standing = false;
        showSubject = withSubject === null;
        parent = null;
        editing = null;
        isRoot = true;
        modal.open();
    }

    export function createSubtask(withParent: number) {
        title = "";
        subject = "null";
        due = null;
        standing = false;
        showSubject = false;
        parent = withParent;
        editing = null;
        isRoot = false;
        modal.open();
    }

    export function edit(todo: {
        id: number;
        title: string;
        subject: number | null;
        due: DateTime | null;
        standing: boolean;
        parent: any | null;
        showSubject: boolean;
    }) {
        title = todo.title;
        subject = JSON.stringify(todo.subject);
        due = todo.due?.toISODate() ?? null;
        standing = todo.standing;
        showSubject = todo.showSubject && todo.parent === null;
        editing = todo.id;
        isRoot = !todo.parent;
        modal.open();
    }

    const createTodo = graphql(`
        mutation CreateTodo($title: String!, $subject: Int, $due: NaiveDate, $standing: Boolean!) {
            createTodo(title: $title, subject: $subject, due: $due, standing: $standing) {
                ...todos_insert
            }
        }
    `);

    const createChild = graphql(`
        mutation CreateSubtask($parent: Int!, $title: String!, $due: NaiveDate) {
            todo(id: $parent) {
                createChild(title: $title, due: $due) {
                    ...todos_insert
                }
            }
        }
    `);

    const editTodo = graphql(`
        mutation EditTodo(
            $id: Int!
            $title: String!
            $subject: Int
            $due: NaiveDate
            $standing: Boolean!
        ) {
            todo(id: $id) {
                title(title: $title) {
                    id
                    title
                }
                subject(id: $subject) {
                    id
                    subject {
                        id
                        name
                    }
                }
                due(due: $due) {
                    id
                    due
                }
                standing(standing: $standing) {
                    id
                    standing
                }
            }
        }
    `);
</script>

<Modal bind:this={modal}>
    <Form
        on:submit={() => {
            modal.close();
            if (editing !== null) {
                editTodo.mutate({
                    id: editing,
                    title,
                    subject: JSON.parse(subject),
                    due: due ? DateTime.fromISO(due) : null,
                    standing,
                });
            } else if (parent === null) {
                createTodo.mutate({
                    title,
                    subject: JSON.parse(subject),
                    due: due ? DateTime.fromISO(due) : null,
                    standing,
                });
            } else {
                createChild.mutate({
                    parent,
                    title,
                    due: due ? DateTime.fromISO(due) : null,
                });
            }
        }}
    >
        <h1>{editing === null ? "Create" : "Edit"} Todo</h1>
        <label>
            Title: <input bind:value={title} maxlength="255" required type="text" autofocus />
        </label>
        {#if showSubject}
            <label>
                Subject:
                <SubjectSelect {subjects} bind:value={subject} />
            </label>
        {/if}
        <label>
            Due: <input bind:value={due} type="date" />
        </label>
        {#if isRoot}
            <label>
                <input bind:checked={standing} type="checkbox" />
                Standing
            </label>
        {/if}
        <input type="submit" value={editing === null ? "Create" : "Save"} />
    </Form>
</Modal>
