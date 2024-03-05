<script lang="ts">
    import Tabs from "$lib/components/Tabs.svelte";
    import TodoListInner from "./_TodoListInner.svelte";
    import Button from "$lib/components/Button.svelte";
    import IconPlus from "~icons/ic/round-plus";
    import CreateTodoModal from "./_CreateTodoModal.svelte";
    import type {DateTime} from "luxon";

    let addModal: CreateTodoModal;
    export let todos: {
        id: number;
        title: string;
        completed: boolean;
        due?: DateTime;
        subject?: {id: number; name: string};
        parent?: number;
        archived: boolean;
        standing: boolean;
    }[];
    export let subject: number | null = null;
    export let subjects: {id: number; name: string}[];

    $: rootTodos = todos.filter(
        (todo) => todo.parent === undefined && (subject === null || todo.subject?.id === subject),
    );
    $: filteredTodos = {
        todo: rootTodos.filter(({standing, archived}) => !standing && !archived),
        standing: rootTodos.filter(({standing, archived}) => standing && !archived),
        archived: rootTodos.filter(({archived}) => archived),
    } as {[key: string]: typeof todos};
</script>

<CreateTodoModal bind:this={addModal} {subjects} />

<Tabs
    let:tab
    tabs={[
        {
            title: "Todo",
            value: "todo",
            count: filteredTodos.todo.length.toString(),
        },
        {
            title: "Standing",
            value: "standing",
            count: filteredTodos.standing.length.toString(),
        },
        {
            title: "Archived",
            value: "archived",
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
    {#if filteredTodos[tab].length === 0}
        Empty <!-- TODO -->
    {:else}
        <TodoListInner
            {todos}
            rootTodos={filteredTodos[tab]}
            showSubject={subject === null}
            addChild={(parent) => addModal.createSubtask(parent)}
            edit={(todo) => addModal.edit({...todo, showSubject: subject === null})}
        />
    {/if}
</Tabs>
