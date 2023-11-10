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
