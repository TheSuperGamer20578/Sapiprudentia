import PubSub from "pubsub-js";

export const API_VERSION = 1;
const BASE_URL = `/api/v${API_VERSION}`;

export interface Document {
    title: string;
    content: object | null;
}

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

export async function getDocument(id: number): Promise<Document> {
    const response = await fetch(`${BASE_URL}/document/${id}`);
    throwForStatus(response);
    return await response.json();
}

export async function updateDocument(id: number, document: Partial<Document>): Promise<void> {
    const response = await fetch(`${BASE_URL}/document/${id}`, {
        method: "PATCH",
        body: JSON.stringify(document),
    });
    throwForStatus(response);
}

export async function login(login: string, password: string): Promise<User> {
    const response = await fetch(`${BASE_URL}/login`, {
        method: "POST",
        body: JSON.stringify({ login, password }),
    });
    throwForStatus(response);
    return await response.json();
}

export async function current_user(): Promise<User> {
    const response = await fetch(`${BASE_URL}/login`);
    throwForStatus(response);
    return await response.json();
}

export async function logout(): Promise<void> {
    const response = await fetch(`${BASE_URL}/login`, {
        method: "DELETE",
    });
    throwForStatus(response);
    PubSub.publish("login_required", null);
}

export async function updateUser(update: { username?: string, name?: string, email?: string, password?: string }): Promise<void> {
    const response = await fetch(`${BASE_URL}/login`, {
        method: "PATCH",
        body: JSON.stringify(update),
    });
    throwForStatus(response);
}
