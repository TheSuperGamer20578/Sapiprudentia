import {type Actions, fail, redirect} from "@sveltejs/kit";
import {login} from "$lib/auth";
import {DateTime} from "luxon";

export const actions = {
    default: async ({request, cookies, url}) => {
        const data = await request.formData();
        if (!data.has("login") || !data.has("password")) {
            return fail(400);
        }
        let resp;
        try {
            resp = await login(data.get("login")!.toString(), data.get("password")!.toString());
        } catch (e) {
            if (e instanceof Response) {
                if (e.status === 403) {
                    return fail(403, {login});
                }
                throw e;
            }
            throw e;
        }
        cookies.set("session", resp.token, {
            path: "/",
            expires: data.get("remember") ? DateTime.now().plus({days: 365}).toJSDate() : undefined,
        });
        throw redirect(303, url.searchParams.get("next") ?? "/");
    },
} satisfies Actions;
