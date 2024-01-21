import {PUBLIC_API_URL} from "$env/static/public";

export interface User {
    id: number;
    username: string;
    name: string;
    email: string;
    account_type: number;
    created_at: string;
    require_password_change: boolean;
}

export async function login(login: string, password: string): Promise<User> {
    const resp = await fetch(`${PUBLIC_API_URL}/auth/login`, {
        credentials: "include",
        method: "POST",
        body: JSON.stringify({login, password}),
    });
    if (!resp.ok) throw resp;
    return await resp.json();
}

export async function currentUser(): Promise<User | null> {
    const resp = await fetch(`${PUBLIC_API_URL}/auth/login`, {
        credentials: "include",
        method: "GET",
    });
    if (resp.status === 401) return null;
    if (!resp.ok) throw resp;
    return await resp.json();
}

export async function logout(): Promise<void> {
    const resp = await fetch(`${PUBLIC_API_URL}/auth/login`, {
        credentials: "include",
        method: "DELETE",
    });
    if (!resp.ok) throw resp;
}
