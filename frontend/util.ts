export function append<T>(set: (value: (current: T[] | null) => T[]) => void, value: T): void {
    set((current) => {
        if (current === null) {
            return [value];
        }
        return [...current, value];
    });
}

export function objectMap<T extends object, O extends object>(object: T, func: (key: string, value: T[keyof T]) => [keyof O, O[keyof O]]): O {
    let result = {} as O;
    for (const [key, value] of Object.entries(object)) {
        const [newKey, newValue] = func(key, value);
        result[newKey] = newValue;
    }
    return result;
}
