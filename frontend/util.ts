export function append<T>(set: (value: (current: T[] | null) => T[]) => void, value: T): void {
    set((current) => {
        if (current === null) {
            return [value];
        }
        return [...current, value];
    });
}
