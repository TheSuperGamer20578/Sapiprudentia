import {setSession} from "$houdini";
import {currentUser} from "$lib/auth";

export async function handle({event, resolve}) {
    const token = event.cookies.get("session");
    if (token !== undefined) {
        if ((await currentUser(token)) === null) {
            event.cookies.delete("session", {path: "/"});
        } else {
            setSession(event, {token});
        }
    }
    return await resolve(event);
}
