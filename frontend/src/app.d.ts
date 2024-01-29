import "unplugin-icons/types/svelte";

declare global {
    namespace App {
        interface Error {}

        interface Locals {}

        interface PageData {
            title?: string;
        }

        interface PageState {}

        interface Platform {}

        interface Session {
            token: string;
        }
    }
}

export {};
