import {DateTime} from "luxon";

/** @type {import("houdini").ConfigFile} */
const config = {
    watchSchema: {
        url: "http://localhost:8000/graphql",
    },
    plugins: {
        "houdini-svelte": {},
    },
    scalars: {
        NaiveDate: {
            type: "DateTime",
            unmarshal: (value) => value && DateTime.fromISO(value),
            marshal: (value) => value && value.toISODate(),
        },
        NaiveDateTime: {
            type: "DateTime",
            unmarshal: (value) => value && DateTime.fromISO(value),
            marshal: (value) => value && value.toISO(),
        },
    },
};

export default config;
