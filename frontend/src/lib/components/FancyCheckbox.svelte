<script lang="ts">
    export let checked = false;
</script>

<input bind:checked on:change type="checkbox" />

<style lang="sass">
    input
        width: min-content
        height: min-content
        margin: 0.2em
</style>
