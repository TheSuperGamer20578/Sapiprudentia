import PubSub from "pubsub-js";
import FormatDate from "./date";
import {objectMap} from "./util";

export const API_VERSION = 1;
const BASE_URL = `/api/v${API_VERSION}`;

export enum AccountType {
    USER = 0,
    ADMIN = 1,
}

export interface User {
    id: number;
    username: string;
    name: string;
    email: string;
    account_type: AccountType;
    created_at: string;
    require_password_change: boolean;
}

export class HttpError extends Error {
    constructor(public readonly response: Response) {
        super(response.statusText);
    }
}

export class UnauthorizedError extends HttpError {
    constructor(response: Response) {
        window.faro.api.resetUser();
        PubSub.publish("login_required", null);
        super(response);
    }
}

function throwForStatus(response: Response) {
    if (response.status === 401) {
        throw new UnauthorizedError(response);
    }
    if (!response.ok) {
        throw new HttpError(response);
    }
}

function setUser(user: User) {
    window.faro.api.setUser({
        email: user.email,
        id: user.id.toString(),
        username: user.username,
        attributes: objectMap<User, Record<string, string>>(user, (key, value) => [key, value.toString()]),
    });
}

function crud<T extends object & {id: number}>(endpoint: string, deserialize?: Partial<{[K in keyof T]: (value: unknown) => T[K]}>) {
    return {
        async list(): Promise<T[]> {
            const response = await fetch(`${BASE_URL}${endpoint}`, {
                headers: {
                    accept: "application/json",
                },
            });
            throwForStatus(response);
            return (await response.json() as {[K in keyof T]: unknown}[])
                .map((item) => {
                    for (const [key, deserializer] of Object.entries(deserialize ?? {})) {
                        item[key as keyof T] = deserializer(item[key as keyof T]);
                    }
                    return item as T;
                });
        },
        async create(data: Omit<T, "id">): Promise<T> {
            const response = await fetch(`${BASE_URL}${endpoint}`, {
                method: "POST",
                body: JSON.stringify(data),
                headers: {
                    "content-type": "application/json",
                    accept: "application/json",
                },
            });
            throwForStatus(response);
            return await response.json();
        },
        async get(id: number): Promise<T> {
            const response = await fetch(`${BASE_URL}${endpoint}/${id}`, {
                headers: {
                    accept: "application/json",
                },
            });
            throwForStatus(response);
            return await response.json();
        },
        async update(id: number, data: Partial<Omit<T, "id">>): Promise<void> {
            const response = await fetch(`${BASE_URL}${endpoint}/${id}`, {
                method: "PATCH",
                body: JSON.stringify(data),
                headers: {
                    "content-type": "application/json",
                },
            });
            throwForStatus(response);
        },
        async delete(id: number): Promise<void> {
            const response = await fetch(`${BASE_URL}${endpoint}/${id}`, {
                method: "DELETE",
            });
            throwForStatus(response);
        },
    };
}


export interface Document {
    id: number,
    title: string,
    content: object | null,
    created_at: string,
    last_modified: string,
}
export const {
    get: getDocument,
    update: updateDocument,
} = crud<Document>("/document");

export interface Subject {
    id: number,
    name: string,
    class: string,
    active: boolean,
    google_classroom_id: string | null,
}
export const {
    list: listSubjects,
    create: createSubject,
    get: getSubject,
    update: updateSubject,
    delete: deleteSubject,
} = crud<Subject>("/subject");

export interface Todo {
    id: number,
    title: string,
    completed: boolean,
    subject?: number,
    parent?: number,
    due?: FormatDate,
    archived: boolean,
}
export const {
    list: listTodos,
    create: createTodo,
    get: getTodo,
    update: updateTodo,
    delete: deleteTodo,
} = crud<Todo>("/todo", {due: (value) => FormatDate.fromString(value as string | null) ?? undefined});

export async function login(login: string, password: string): Promise<User> {
    const response = await fetch(`${BASE_URL}/login`, {
        method: "POST",
        body: JSON.stringify({ login, password }),
    });
    throwForStatus(response);
    const user: User = await response.json();
    setUser(user);
    return user;
}

export async function current_user(): Promise<User> {
    const response = await fetch(`${BASE_URL}/login`);
    throwForStatus(response);
    const user: User = await response.json();
    setUser(user);
    return user;
}

export async function logout(): Promise<void> {
    const response = await fetch(`${BASE_URL}/login`, {
        method: "DELETE",
    });
    throwForStatus(response);
    window.faro.api.resetUser();
    PubSub.publish("login_required", null);
}

export async function updateUser(update: { username?: string, name?: string, email?: string, password?: string }): Promise<void> {
    const response = await fetch(`${BASE_URL}/login`, {
        method: "PATCH",
        body: JSON.stringify(update),
    });
    throwForStatus(response);
}
