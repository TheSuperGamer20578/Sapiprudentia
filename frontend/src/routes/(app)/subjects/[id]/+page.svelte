<script lang="ts">
    import {graphql, type PageData} from "$houdini";
    import Spinner from "$lib/components/Spinner.svelte";
    import TodoList from "$lib/components/todo/TodoList.svelte";
    import Corner from "$lib/components/Corner.svelte";
    import Button from "$lib/components/Button.svelte";
    import IconStarFull from "~icons/ic/star";
    import IconStarEmpty from "~icons/ic/star-border";

    export let data: PageData;
    $: ({Subject} = data);
    $: subject = $Subject.data?.subject;
    $: todos =
        $Subject.data?.todos.map((todo) => ({
            id: todo.id,
            title: todo.title,
            completed: todo.completed,
            due: todo.due ?? undefined,
            subject: (todo.subject && {...todo.subject}) ?? undefined,
            parent: todo.parent?.id,
            archived: todo.archived,
        })) ?? [];

    const setActive = graphql(`
        mutation SetSubjectActive($id: Int!, $active: Boolean!) {
            subject(id: $id) {
                active(active: $active) {
                    id
                    active
                }
            }
        }
    `);
</script>

{#if $Subject.fetching}
    <Spinner />
{:else}
    <div class="info-block">
        <Corner>
            <Button
                title={subject?.active ? "Remove from sidebar" : "Add to sidebar"}
                colour="icon"
                small
                rem
                on:click={async () => {
                    await setActive.mutate({id: subject.id, active: !subject.active});
                    await Subject.fetch(); // FIXME
                }}
            >
                {#if subject?.active}
                    <IconStarFull />
                {:else}
                    <IconStarEmpty />
                {/if}
            </Button>
        </Corner>
        <h1>{subject?.name}</h1>
        <p class="class">{subject?.class}</p>
    </div>
    <h2>Todo</h2>
    <TodoList subject={subject?.id} {todos} subjects={[]} />
{/if}

<style lang="sass">
    @import "$lib/vars"

    .info-block
        background: $bg2
        padding: 1em
        border-radius: $corners
        box-shadow: $box-shadow
        margin-bottom: 1em
</style>
