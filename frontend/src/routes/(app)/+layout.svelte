<script lang="ts">
    import UserBox from "./UserBox.svelte";
    import Nav from "./Nav.svelte";
    import {page} from "$app/stores";
    import type {LayoutData} from "./$houdini";
    import {redirect} from "@sveltejs/kit";
    import {goto} from "$app/navigation";

    export let data: LayoutData;
    $: ({Layout} = data);
    $: layoutData = $Layout.data!;
    $: currentUser = layoutData.currentUser!;
</script>

<svelte:head>
    {#if $page.data.title !== undefined}
        <title>{$page.data.title} | Sapiprudentia</title>
    {:else}
        <title>Sapiprudentia</title>
    {/if}
</svelte:head>

<div class="container">
    <div class="sidebar">
        <UserBox email={currentUser.email} name={currentUser.name} />
        <Nav
            subjects={layoutData.subjects
                ?.filter(({active}) => active)
                .map(({id, name}) => ({
                    id,
                    name,
                }))}
        />
    </div>
    <div class="content">
        <slot />
    </div>
</div>

<style lang="sass">
    @import "$lib/vars"


    .container
        width: 100vw
        height: 100vh
        display: flex
        flex-direction: row

    .content
        flex: 1
        overflow: auto
        max-width: 100ch
        margin: 2em auto 0 auto
        padding-inline: 1em

    .sidebar
        width: 25ch
        border-right: $div-border
        background: $bg2
        display: flex
        flex-direction: column
</style>
