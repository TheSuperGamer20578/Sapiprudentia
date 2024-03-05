<script lang="ts">
    import TodoList from "$lib/components/todo/TodoList.svelte";
    import type {PageData} from "./$houdini";
    import Spinner from "$lib/components/Spinner.svelte";

    export let data: PageData;
    $: ({Todo} = data);
    $: todos =
        $Todo.data?.todos.map((todo) => ({
            id: todo.id,
            title: todo.title,
            completed: todo.completed,
            due: todo.due ?? undefined,
            subject: (todo.subject && {...todo.subject}) ?? undefined,
            parent: todo.parent?.id,
            archived: todo.archived,
            standing: todo.standing,
        })) ?? [];
    $: subjects = $Todo.data?.subjects ?? [];
</script>

<h1>Todo</h1>
{#if $Todo.fetching}
    <Spinner />
{:else}
    <TodoList {todos} {subjects} />
{/if}
