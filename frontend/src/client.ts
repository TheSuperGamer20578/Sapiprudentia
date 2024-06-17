import {HoudiniClient} from "$houdini";
import {env} from "$env/dynamic/public";

export default new HoudiniClient({
    url: `${env.PUBLIC_API_URL}/graphql`,
    fetchParams: ({session}) => {
        if (session) {
            return {
                credentials: "include",
                headers: {
                    Authorization: `Bearer ${session.token}`,
                },
            };
        }
        return {};
    },
});
