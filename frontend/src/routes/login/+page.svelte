<script lang="ts">
    import Form from "$lib/components/Form.svelte";
</script>

<div class="bg">
    <div class="form">
        <Form server method="POST">
            <h1>Login</h1>
            <label>
                Username: <input autofocus name="login" type="text" />
            </label>
            <br />
            <label>
                Password: <input name="password" type="password" />
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
        background: url("/mikolaj-DCzpr09cTXY-unsplash.jpg") center bottom
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
