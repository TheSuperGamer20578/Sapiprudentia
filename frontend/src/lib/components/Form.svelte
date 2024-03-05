<script lang="ts">
    export let server: boolean = false;
</script>

{#if server}
    <form on:submit {...$$restProps}>
        <slot />
    </form>
{:else}
    <form on:submit|preventDefault {...$$restProps}>
        <slot />
    </form>
{/if}

<style lang="sass">
    @import "$lib/vars"

    form
        :global(label)
            display: block
            margin-bottom: 1em

        :global(input:not([type="checkbox"]))
            border: none
            outline: none
            border-bottom: $fg 1px solid
            padding: 0.25em
            width: calc(100% - 0.5em)

            &:hover
                border-color: mix($fg-accent, $fg)

            &:focus
                border-color: $fg-accent

        :global(input[type="submit"])
            @include clickable($bg-accent)
            display: block
            float: right
            border: none
            outline: none
            padding: 0.25em 1em
            width: min-content

        :global(h1)
            color: $fg-accent
            font-family: $font-slab
            margin-bottom: 0.75em
</style>
