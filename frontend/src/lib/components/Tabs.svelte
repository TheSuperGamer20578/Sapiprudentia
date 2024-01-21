<script generics="T" lang="ts">
    import Button from "$lib/components/Button.svelte";

    let current = 0;
    export let tabs: {title: string; value: T; count?: string}[];
    let content: HTMLDivElement;
    $: content?.children.item(current)?.classList.add("active");
</script>

<div class="tabs">
    {#each tabs as tab, i}
        <Button colour={i === current ? "active" : "normal"} on:click={() => (current = i)}>
            {tab.title}
            {#if tab.count !== undefined}
                <span class="count">
                    {tab.count}
                </span>
            {/if}
        </Button>
    {/each}
    {#if $$slots.end}
        <div class="end">
            <slot name="end" />
        </div>
    {/if}
</div>

<slot tab={tabs[current].value} />

<style lang="sass">
    @import "$lib/vars"

    .tabs
        display: flex
        flex-direction: row
        flex-wrap: wrap
        gap: 1em
        border-bottom: $div-border
        padding-bottom: 0.5em
        margin: 0.5em

    .count
        background: $greyer
        border-radius: $corners
        padding: calc($padding / 8) calc($padding / 4)
        margin-right: -0.75em
        margin-left: 0.5em
        display: inline-block

    .end
        margin-left: auto
</style>
