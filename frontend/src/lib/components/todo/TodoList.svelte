<script lang="ts">
    import Tabs from "$lib/components/Tabs.svelte";
    import TodoListInner from "./_TodoListInner.svelte";
    import Button from "$lib/components/Button.svelte";
    import IconPlus from "~icons/ic/round-plus";
    import CreateTodoModal from "./_CreateTodoModal.svelte";

    let addModal: CreateTodoModal;
    export let todos: {
        id: number;
        title: string;
        completed: boolean;
        due?: Date;
        subject?: {id: number; name: string};
        parent?: number;
        archived: boolean;
    }[];
    export let subject: number | null = null;
    export let subjects: {id: number; name: string}[];

    $: rootTodos = todos.filter(
        (todo) => todo.parent === undefined && (subject === null || todo.subject?.id === subject),
    );
    $: todo = rootTodos.filter(({archived}) => !archived);
    $: archived = rootTodos.filter(({archived}) => archived);
</script>

<CreateTodoModal bind:this={addModal} {subjects} />

<Tabs
    let:tab
    tabs={[
        {
            title: "Todo",
            value: false,
            count: todo.length.toString(),
        },
        {
            title: "Archived",
            value: true,
            count: archived.length.toString(),
        },
    ]}
>
    <Button
        colour="icon"
        on:click={() => addModal.createRoot(subject)}
        rem={true}
        slot="end"
        small={true}
    >
        <IconPlus />
    </Button>
    {#if (tab ? archived : todo).length === 0}
        Empty <!-- TODO -->
    {:else}
        <TodoListInner
            {todos}
            rootTodos={tab ? archived : todo}
            showSubject={subject === null}
            addChild={(parent) => addModal.createSubtask(parent)}
            edit={(todo) => addModal.edit(todo)}
        />
    {/if}
</Tabs>
