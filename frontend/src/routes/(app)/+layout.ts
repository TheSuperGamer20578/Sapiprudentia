import type {AfterLoadEvent} from "./$houdini";
import {redirect} from "@sveltejs/kit";

export function _houdini_afterLoad({event, data}: AfterLoadEvent) {
    if (!data.Layout?.currentUser) {
        throw redirect(302, `/login?next=${encodeURIComponent(event.url.pathname)}`);
    }
}
