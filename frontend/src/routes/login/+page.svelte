<script lang="ts">
    import {login} from "$lib/auth";
    import {goto} from "$app/navigation";
    import {page} from "$app/stores";
    import Form from "$lib/components/Form.svelte";

    let username = "";
    let password = "";
</script>

<div class="bg">
    <div class="form">
        <Form
            on:submit={async () => {
                await login(username, password);
                await goto(decodeURIComponent($page.url.searchParams.get("next") ?? "/"));
            }}
        >
            <h1>Login</h1>
            <label>
                Username: <input autofocus bind:value={username} type="text" />
            </label>
            <br />
            <label>
                Password: <input bind:value={password} type="password" />
            </label>
            <br />
            <input type="submit" />
        </Form>
    </div>
</div>

<style lang="sass">
    @import "$lib/vars"

    .bg
        height: 100vh
        width: 100vw
        background: url("/static/mikolaj-DCzpr09cTXY-unsplash.jpg") center bottom
        background-size: cover
        display: flex
        align-items: center
        justify-content: center

    .form
        background: $bg
        max-width: 50ch
        padding: 1em
        box-shadow: $box-shadow
</style>
