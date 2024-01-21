<script lang="ts">
    import Avatar from "@svelte-put/avatar/Avatar.svelte";

    export let name: string;
    export let email: string;
</script>

<div class="container">
    <Avatar
        gravatar={email}
        style="width: 3rem; height: 3rem; margin-right: 1em;"
        uiAvatar={name.replace(" ", "+")}
    />
    <p class="name">{name}</p>
</div>

<style lang="sass">
    @import "$lib/vars"

    .container
        border-bottom: $div-border
        height: 3rem
        display: flex
        align-items: center
</style>
