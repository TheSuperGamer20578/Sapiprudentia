<script lang="ts">
    import type {PageData} from "$houdini";
    import Spinner from "$lib/components/Spinner.svelte";
    import Corner from "$lib/components/Corner.svelte";
    import Button from "$lib/components/Button.svelte";
    import IconPlus from "~icons/ic/round-plus";
    import CreateSubjectModal from "./CreateSubjectModal.svelte";

    let createModal: CreateSubjectModal = null!;

    export let data: PageData;
    $: ({Subjects} = data);
</script>

<CreateSubjectModal bind:this={createModal} />

<Corner>
    <Button colour="icon" small rem title="Add" on:click={createModal.open}>
        <IconPlus />
    </Button>
</Corner>
<h1>Subjects</h1>
{#if $Subjects.fetching}
    <Spinner />
{:else}
    <ul>
        {#each $Subjects.data?.subjects ?? [] as subject (subject.id)}
            <li>
                <a href="/subjects/{subject.id}" class:active={subject.active}>
                    <p class="name">{subject.name}</p>
                    <p class="class">{subject.class}</p>
                </a>
            </li>
        {/each}
        <li aria-hidden="true" />
        <li aria-hidden="true" />
    </ul>
{/if}

<style lang="sass">
    @import "$lib/vars"

    ul
        list-style: none
        display: flex
        flex-direction: row
        flex-wrap: wrap
        gap: 1em
        margin-top: 1em

    li
        flex-grow: 1
        width: 25ch

    a
        @include clickable($bg2)
        display: block
        padding: 1em
        text-decoration: none
        color: $fg-placeholder
        border-radius: $corners
        box-shadow: $box-shadow
        &.active
            color: $fg

    .name
        font-size: 1.25em

    .class
        font-size: 0.75em
</style>
