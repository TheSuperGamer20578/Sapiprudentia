<script lang="ts">
    export let active: boolean = false;
</script>

<span class="container">
    <slot name="label" />
    {#if active}
        <div class="confetti">
            <slot />
        </div>
    {/if}
</span>

<style lang="sass">
    .container
        position: relative

    .confetti
        position: absolute
        top: 50%
        left: 50%
</style>
