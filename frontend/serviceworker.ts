import {registerRoute} from "workbox-routing";
import {getOrCreatePrecacheController} from "workbox-precaching/utils/getOrCreatePrecacheController";
import {precacheAndRoute, PrecacheEntry} from "workbox-precaching";
declare const WB_MANIFEST: PrecacheEntry[];

const precache = getOrCreatePrecacheController();

precacheAndRoute(WB_MANIFEST);

registerRoute(
    ({request}) => request.headers.get("accept")?.split(",").includes("text/html"),
    async () => {
        return (await precache.matchPrecache("/static/react.html"))!;
    },
    "GET",
);
