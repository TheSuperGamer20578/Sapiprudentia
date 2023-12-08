import {useCallback, useEffect, useState} from "react";
import {useBeforeUnload} from "react-router-dom";

const UPDATE_INTERVAL_MS = 2500;

export function useSync<I, T>(
    id: I,
    initial: (id: I) => Promise<T>,
    sync: (id: I, value: Partial<T>) => Promise<void>,
    interval_ms: number = UPDATE_INTERVAL_MS,
): {
    value: T | undefined,
    pending: boolean,
    update: (value: Partial<T>) => void,
} {
    const [value, setValue] = useState<T | undefined>(undefined);
    const [pending, setPending] = useState<Partial<T>>({});
    useEffect(() => {
        setValue(undefined);
        initial(id).then(setValue);
        const interval = setInterval(() => setPending((current) => {
            if (Object.keys(current).length !== 0) {
                sync(id, current).then();
            }
            return {};
        }), interval_ms);
        return () => clearInterval(interval);
    }, [id]);
    useBeforeUnload(
        useCallback((e) => {
            if (Object.keys(pending).length !== 0) {
                e.preventDefault();
                return true;
            }
            return false;
        }, [pending])
    );
    return {
        value,
        pending: Object.keys(pending).length !== 0,
        update: (value: Partial<T>) => {
            setPending((current) => ({...current, ...value}));
            if (value === undefined) {
                console.warn("Attempted to set value before initial value was loaded");
                return;
            }
            setValue((current) => ({...current!, ...value}));
        }
    };
}

export function useSyncMany<I, T>(
    initial: () => Promise<(T & {id: I})[]>,
    sync: (id: I, value: Partial<T>) => Promise<void>,
    add: (value: T) => Promise<T & {id: I}>,
    interval_ms: number = UPDATE_INTERVAL_MS,
): {
        value: Map<I, T & {id: I}> | undefined,
        pending: boolean,
        update: (id: I, value: Partial<T>) => void,
        add: (value: T) => void,
} {
    const [value, setValue] = useState<Map<I, T & {id: I}> | undefined>(undefined);
    const [pending, setPending] = useState<Map<I, Partial<T>>>(new Map());
    useEffect(() => {
        initial().then((value) => setValue(new Map(value.map((value) => [value.id, value]))));
        const interval = setInterval(() => setPending((current) => {
            if (current.size !== 0) {
                for (const [id, value] of current.entries()) {
                    sync(id, value).then();
                }
            }
            return new Map();
        }), interval_ms);
        return () => clearInterval(interval);
    }, []);
    return {
        value,
        pending: pending.size !== 0,
        update: (id: I, value: Partial<T>) => {
            setPending((current) => (new Map(current)).set(id, {...current.get(id), ...value}));
            if (value === undefined) {
                console.warn("Attempted to set value before initial value was loaded");
                return;
            }
            setValue((current) => (new Map(current)).set(id, {...current!.get(id)!, ...value}));
        },
        add: (value: T) => {
            add(value).then((value) => setValue((current) => (new Map(current)).set(value.id, value)));
        },
    };
}
