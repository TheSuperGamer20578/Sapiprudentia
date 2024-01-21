<script lang="ts">
    import Button from "$lib/components/Button.svelte";
    import Corner from "$lib/components/Corner.svelte";
    import IconClose from "~icons/ic/close";

    let modal: HTMLDialogElement;

    export function open() {
        modal.showModal();
    }

    export function close() {
        modal.close();
    }
</script>

<dialog bind:this={modal} on:close>
    <Corner>
        <Button colour="icon" on:click={close} rem small title="Close">
            <IconClose />
        </Button>
    </Corner>
    <slot />
</dialog>

<style lang="sass">
    @import "$lib/vars"

    dialog
        margin: auto
        padding: 1em
        border: none
        box-shadow: $box-shadow
        min-width: 50ch
</style>
