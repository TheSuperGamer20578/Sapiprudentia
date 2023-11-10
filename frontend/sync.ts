import {useEffect, useState} from "react";

const UPDATE_INTERVAL_MS = 2500;

export function useSync<I, T>(id: I, initial: (id: I) => Promise<T>, sync: (id: I, value: Partial<T>) => Promise<void>, interval_ms: number = UPDATE_INTERVAL_MS): [T | undefined, boolean, (value: Partial<T>) => void] {
    const [value, setValue] = useState<T | undefined>(undefined);
    const [pending, setPending] = useState<Partial<T>>({});
    useEffect(() => {
        initial(id).then(setValue);
        const interval = setInterval(() => setPending((current) => {
            if (Object.keys(current).length !== 0) {
                sync(id, current).then();
            }
            return {};
        }), interval_ms);
        return () => clearInterval(interval);
    }, []);
    return [value, Object.keys(pending).length !== 0, (value: Partial<T>) => {
        // console.log(value);
        console.log(pending);
        setPending((current) => ({...current, ...value}));
        if (value === undefined) {
            console.warn("Attempted to set value before initial value was loaded");
            return;
        }
        setValue((current) => ({...current!, ...value}));
    }];
}
