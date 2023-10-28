export interface Document {
    title: string;
    content: object | null;
}

export async function getDocument(id: number): Promise<Document> {
    const response = await fetch(`/api/document/${id}`);
    if (!response.ok) {
        throw new Error("Failed to get document");
    }
    return await response.json();
}

export async function updateDocument(id: number, document: Partial<Document>): Promise<void> {
    const response = await fetch(`/api/document/${id}`, {
        method: "PATCH",
        body: JSON.stringify(document),
    });
    if (!response.ok) {
        throw new Error("Failed to update document");
    }
}

export let pendingChanges: boolean;

function setSaveIndicator(pending: boolean) {
    if (pending === pendingChanges) {
        return;
    }
    pendingChanges = pending;
    setTimeout(() => {
        Array.from(document.getElementsByClassName("save-indicator-saved")).map((element) => {
            element.classList.toggle("hidden", pendingChanges);
        })
        Array.from(document.getElementsByClassName("save-indicator-pending")).map((element) => {
            element.classList.toggle("hidden", !pendingChanges);
        })
    }, 250);
}

setSaveIndicator(false);
let queue: Map<(id: any, param: any) => Promise<void>, Map<any, object>> = new Map();

export async function queueChanges<I, P extends Partial<object>>(func: (id: I, param: P) => Promise<void>, id: I, param: P): Promise<void> {
    let fn_map = queue.get(func) ?? new Map();
    let current = fn_map.get(id) ?? {};
    fn_map.set(id, {...current, ...param});
    queue.set(func, fn_map);
    setSaveIndicator(true);
}

setInterval(async () => {
    if (!pendingChanges) {
        return;
    }
    for (const [func, map] of queue) {
        for (const [id, param] of map) {
            await func(id, param);
        }
    }
    queue.clear();
    setSaveIndicator(false);
}, 2500);
