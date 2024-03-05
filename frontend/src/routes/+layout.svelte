<script lang="ts">
    import {injectSpeedInsights} from "@vercel/speed-insights/sveltekit";
    import {inject} from "@vercel/analytics";

    injectSpeedInsights();
    inject();
</script>

<div class="container">
    <slot />
</div>

<style lang="sass">
    @import "$lib/vars"

    \:global(*, *::before, *::after)
        margin: 0
        padding: 0

    \:global(::-webkit-scrollbar)
        width: 0.75em
        background: none
        border-radius: 1vmax
        border: 0.2em solid transparent
        background-clip: padding-box

        &:hover
            background-color: $greyer

    \:global(::-webkit-scrollbar-button)
        display: none

    \:global(::-webkit-scrollbar-thumb)
        background: $greyer
        border-radius: 1vmax
        border: 0.2em solid transparent
        background-clip: padding-box

        &:hover
            background-color: $greyer2

        &:active
            background-color: $greyer3

    .container
        font-family: $font-normal
        scrollbar-width: thin
        scrollbar-color: $greyer none
        scroll-behavior: smooth
</style>
