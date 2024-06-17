import {env} from "$env/dynamic/public";

export interface User {
    id: number;
    username: string;
    name: string;
    email: string;
    account_type: number;
    created_at: string;
    require_password_change: boolean;
}

export async function login(login: string, password: string): Promise<{token: string; user: User}> {
    const resp = await fetch(`${env.PUBLIC_API_URL}/auth/login`, {
        credentials: "include",
        method: "POST",
        body: JSON.stringify({login, password}),
        headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
        },
    });
    if (!resp.ok) throw resp;
    return await resp.json();
}

export async function currentUser(token: string): Promise<User | null> {
    const resp = await fetch(`${env.PUBLIC_API_URL}/auth/login`, {
        credentials: "include",
        method: "GET",
        headers: {
            Authorization: `Bearer ${token}`,
            Accept: "application/json",
        },
    });
    if (resp.status === 401) return null;
    if (!resp.ok) throw resp;
    return await resp.json();
}

export async function logout(token: string): Promise<void> {
    const resp = await fetch(`${env.PUBLIC_API_URL}/auth/login`, {
        credentials: "include",
        method: "DELETE",
        headers: {
            Authorization: `Bearer ${token}`,
        },
    });
    if (!resp.ok) throw resp;
}
