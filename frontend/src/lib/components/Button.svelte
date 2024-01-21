<script lang="ts">
    export let title: string | undefined = undefined;
    export let colour: "normal" | "active" | "icon" = "icon";
    export let small: boolean = false;
    export let rem: boolean = false;
</script>

<button class={colour} class:rem class:small on:click {title}>
    <slot />
</button>

<style lang="sass">
    @import "$lib/vars"

    button
        border-radius: $corners
        padding: calc($padding / 4) $padding
        border: none

        &.normal
            @include clickable($bg2)

        &.active
            @include clickable($bg-accent)

        &.icon
            @include clickable($bg2, transparent)

        &.small
            padding: 0

        &.rem
            font-size: 1rem
</style>
