<script lang="ts">
    import type {PageData} from "./$houdini";
    import NoteList from "$lib/components/notes/NoteList.svelte";
    import Spinner from "$lib/components/Spinner.svelte";

    export let data: PageData;
    $: ({Notes} = data);
    $: notes = $Notes.data!.notes!;
</script>

{#if $Notes.fetching}
    <Spinner />
{:else}
    <NoteList {notes} />
{/if}
