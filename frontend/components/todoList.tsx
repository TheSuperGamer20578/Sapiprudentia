import {useSyncMany} from "../sync";
import {createTodo, deleteTodo, getSubject, listTodos, Todo, updateTodo} from "../api";
import React, {useEffect, useState} from "react";
import LoadingOverlay from "./loadingOverlay";
import Modal from "./modal";
import Form from "./form";
import SubjectSelect from "./subjectSelect";
import Corner from "./corner";
import IconButton from "./iconButton";
import {FaArchive, FaBook, FaCalendar, FaEdit, FaInbox, FaPlus, FaTrash} from "react-icons/fa";
import Checkbox from "./checkbox";
import Chip from "./chip";
import styles from "./todoList.module.sass";
import FormatDate from "../date";

interface Props {
    subject?: number;
    children?: React.ReactNode;
}

export default function TodoList({subject, children}: Props) {
    const {
        value: todos,
        update: setTodo,
        add: addTodo,
        delete: delTodo,
    } = useSyncMany(listTodos, updateTodo, createTodo, deleteTodo, 0);
    const [createModalOpen, setCreateModalOpen] = useState(false);
    const [createTitle, setCreateName] = useState("");
    const [createSubject, setCreateSubject] = useState<number | null>(null);
    const [createDue, setCreateDue] = useState<string | null>(null);
    const [createParent, setCreateParent] = useState<number | null>(null);
    const [createEdit, setCreateEdit] = useState<Todo | null>(null);

    if (todos === undefined) {
        return <LoadingOverlay force={true} />;
    }

    return <>
        <Modal open={createModalOpen} setOpen={setCreateModalOpen}>
            <Form
                title={createEdit === null ? "Create Todo" : "Edit Todo"}
                submitText={createEdit === null ? "Create" : "Save"}
                submit={() => {
                    setCreateModalOpen(false);
                    if (createEdit === null) {
                        addTodo({
                            title: createTitle,
                            completed: false,
                            archived: false,
                            subject: subject ?? createSubject ?? undefined,
                            due: createDue === null ? undefined : FormatDate.fromString(createDue) ?? undefined,
                            parent: createParent ?? undefined,
                        });
                    } else {
                        setTodo(createEdit.id, {
                            title: createTitle || undefined,
                            subject: subject ?? createSubject ?? undefined,
                            due: createDue === null ? undefined : FormatDate.fromString(createDue) ?? undefined,
                        });
                    }
                }}
            >
                <label>
                    <p>Title:</p>
                    <input
                        type="text"
                        required
                        maxLength={255}
                        onInput={(e) => setCreateName(e.currentTarget.value)}
                        defaultValue={createEdit?.title}
                    />
                </label>
                {subject === undefined && (createEdit === null ? createParent === null : createEdit.parent === null) &&
                    <label>
                        <p>Subject:</p>
                        <SubjectSelect required={false} onInput={setCreateSubject} defaultValue={createEdit?.subject} />
                    </label>
                }
                <label>
                    <p>Due:</p>
                    <input
                        type="date"
                        onInput={(e) => setCreateDue(e.currentTarget.value)}
                        defaultValue={createEdit?.due?.toISOString()}
                    />
                </label>
            </Form>
        </Modal>
        <Corner
            corner={
                <IconButton
                    title="Create Todo"
                    onClick={() => {
                        setCreateParent(null);
                        setCreateEdit(null);
                        setCreateModalOpen(true);
                    }
                }>
                    <FaPlus />
                </IconButton>
            }
        >
            {children}
            <ul className={styles.list}>
                {Array.from(todos.values()).filter((todo) => todo.parent === null && !todo.archived && (subject === undefined || todo.subject == subject)).map((todo) =>
                    <li key={todo.id}><Todo todo={todo} showSubject={subject === undefined} setCreateModalOpen={setCreateModalOpen} setTodo={setTodo} delTodo={delTodo} setCreateEdit={setCreateEdit} setCreateParent={setCreateParent} todos={todos} /></li>
                )}
            </ul>
            <details>
            <summary>Archived:</summary>
                <ul className={styles.list}>
                {Array.from(todos.values()).filter((todo) => todo.parent === null && todo.archived && (subject === undefined || todo.subject == subject)).map((todo) =>
                    <li key={todo.id}><Todo todo={todo} showSubject={subject === undefined} setCreateModalOpen={setCreateModalOpen} setTodo={setTodo} delTodo={delTodo} setCreateEdit={setCreateEdit} setCreateParent={setCreateParent} todos={todos} /></li>
                )}
                </ul>
            </details>
        </Corner>
    </>;
}

interface TodoProps {
    todo: Todo;
    nested?: boolean;
    setCreateParent: (parent: number | null) => void;
    setCreateEdit: (todo: Todo | null) => void;
    setCreateModalOpen: (open: boolean) => void;
    todos: Map<number, Todo>;
    setTodo: (id: number, todo: Partial<Todo>) => void;
    delTodo: (id: number) => void;
    showSubject?: boolean;
}

function Todo({todo, nested, setCreateParent, setCreateModalOpen, setCreateEdit, todos, setTodo, delTodo, showSubject}: TodoProps) {
    const [subjectName, setSubjectName] = useState<string | null>(null)
    useEffect(() => {
        if (todo.subject !== undefined && todo.subject !== null) {
            getSubject(todo.subject).then((subject) => setSubjectName(subject.name));
        }
    }, []);
    return <>
        <div className={styles.todo}>
            <Checkbox checked={todo.completed} setChecked={(checked) => setTodo(todo.id, {completed: checked})}/>
            <p>{todo.title}</p>
            <div className={styles.buttonContainer}>
                <IconButton
                    title="Create Subtask"
                    onClick={() => {
                        setCreateParent(todo.id);
                        setCreateEdit(null);
                        setCreateModalOpen(true);
                    }
                    }>
                    <FaPlus/>
                </IconButton>
                <IconButton
                    title="Edit"
                    onClick={() => {
                        setCreateEdit(todo);
                        setCreateModalOpen(true);
                    }}
                >
                    <FaEdit/>
                </IconButton>
                {!nested && (todo.archived
                    ? <IconButton
                        title="Restore"
                        onClick={() => setTodo(todo.id, {archived: false})}
                    >
                        <FaInbox/>
                    </IconButton>
                    : <IconButton
                        title="Archive"
                        onClick={() => setTodo(todo.id, {archived: true})}
                    >
                        <FaArchive/>
                    </IconButton>)
                }
                <IconButton
                    title="Delete"
                    onClick={() => delTodo(todo.id)}
                >
                    <FaTrash/>
                </IconButton>
            </div>
            {showSubject && !nested && todo.subject !== null &&
                <Chip>
                    <FaBook/>
                    <p>{subjectName}</p>
                </Chip>}
            {todo.due !== undefined && <Chip><FaCalendar/><p>{todo.due?.toRelativeOrAbsoluteString()}</p></Chip>}
        </div>
        <ul className={styles.nestList}>
            {Array.from(todos.values()).filter((t) => t.parent === todo.id).map((todo) =>
                <li key={todo.id}><Todo todo={todo} nested setCreateModalOpen={setCreateModalOpen} setTodo={setTodo}
                                        delTodo={delTodo} setCreateEdit={setCreateEdit}
                                        setCreateParent={setCreateParent} todos={todos}/></li>
            )}
        </ul>
    </>;
}
