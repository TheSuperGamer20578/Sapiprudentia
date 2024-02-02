<script lang="ts">
    import Modal from "$lib/components/Modal.svelte";
    import Form from "$lib/components/Form.svelte";
    import {graphql} from "$houdini";

    let modal: Modal = null!;
    let name = "";
    let class_ = "";

    const create = graphql(`
        mutation CreateSubject($name: String!, $class: String!) {
            createSubject(name: $name, class: $class) {
                ...subjects_insert
            }
        }
    `);

    export function open() {
        name = "";
        class_ = "";
        modal.open();
    }
</script>

<Modal bind:this={modal}>
    <Form
        on:submit={() => {
            modal.close();
            create.mutate({
                name,
                class: class_,
            });
        }}
    >
        <h1>Create Subject</h1>
        <label>
            Name: <input autofocus required type="text" maxlength="255" bind:value={name} />
        </label>
        <label>
            Class: <input required type="text" maxlength="16" bind:value={class_} />
        </label>
        <input type="submit" value="Create" />
    </Form>
</Modal>
