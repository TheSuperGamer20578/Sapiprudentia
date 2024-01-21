<script lang="ts">
    import FancyCheckbox from "$lib/components/FancyCheckbox.svelte";
    import Button from "$lib/components/Button.svelte";
    import IconPlus from "~icons/ic/round-plus";
    import IconEdit from "~icons/ic/edit";
    import IconArchive from "~icons/ic/archive";
    import IconDelete from "~icons/ic/delete";
    import IconInbox from "~icons/ic/inbox";
    import IconCalendar from "~icons/ic/baseline-calendar-today";
    import IconBook from "~icons/ic/book";
    import {graphql} from "$houdini";
    import {DateTime} from "luxon";
    import Chip from "$lib/components/Chip.svelte";

    interface Todo {
        id: number;
        title: string;
        completed: boolean;
        due?: DateTime;
        subject?: {id: number; name: string};
        parent?: number;
        archived: boolean;
    }

    export let todos: Todo[];
    export let rootTodos: Todo[];
    export let showSubject: boolean;
    export let nested: boolean = false;
    export let addChild: (parent: number) => void;
    export let edit: (todo: {
        id: number;
        title: string;
        subject: number | null;
        due: DateTime | null;
        parent: any | null;
    }) => void;

    const archive = graphql(`
        mutation ArchiveTodo($id: Int!) {
            todo(id: $id) {
                archived(archived: true) {
                    id
                    archived
                }
            }
        }
    `);
    const restore = graphql(`
        mutation RestoreTodo($id: Int!) {
            todo(id: $id) {
                archived(archived: false) {
                    id
                    archived
                }
            }
        }
    `);
    const deleteTodo = graphql(`
        mutation DeleteTodo($id: Int!) {
            todo(id: $id) {
                delete {
                    ...todos_remove
                }
            }
        }
    `);
    const check = graphql(`
        mutation CheckTodo($id: Int!, $checked: Boolean!) {
            todo(id: $id) {
                completed(completed: $checked) {
                    id
                    completed
                }
            }
        }
    `);
</script>

<ul class:nested>
    {#each rootTodos as todo (todo.id)}
        <li>
            <div class="todo">
                <div>
                    <FancyCheckbox
                        bind:checked={todo.completed}
                        on:change={async () => {
                            await check.mutate({id: todo.id, checked: todo.completed});
                        }}
                    />
                    <p>{todo.title}</p>
                    <div class="buttons">
                        <Button
                            title="Add Subtask"
                            colour="icon"
                            small
                            rem
                            on:click={() => addChild(todo.id)}
                        >
                            <IconPlus />
                        </Button>
                        <Button
                            title="Edit"
                            colour="icon"
                            small
                            rem
                            on:click={() => {
                                edit({
                                    id: todo.id,
                                    title: todo.title,
                                    subject: todo.subject?.id ?? null,
                                    due: todo.due ?? null,
                                    parent: todo.parent,
                                });
                            }}
                        >
                            <IconEdit />
                        </Button>
                        {#if todo.parent === undefined}
                            {#if todo.archived}
                                <Button
                                    title="Restore"
                                    colour="icon"
                                    small
                                    rem
                                    on:click={async () => await restore.mutate({id: todo.id})}
                                >
                                    <IconInbox />
                                </Button>
                            {:else}
                                <Button
                                    title="Archive"
                                    colour="icon"
                                    small
                                    rem
                                    on:click={async () => await archive.mutate({id: todo.id})}
                                >
                                    <IconArchive />
                                </Button>
                            {/if}
                        {/if}
                        <Button
                            title="Delete"
                            colour="icon"
                            small
                            rem
                            on:click={async () => await deleteTodo.mutate({id: todo.id})}
                        >
                            <IconDelete />
                        </Button>
                    </div>
                </div>
                {#if todo.due || todo.subject}
                    <div class="chips">
                        {#if todo.due}
                            {@const days = Math.round(
                                todo.due.diff(DateTime.now().startOf("day")).as("days"),
                            )}
                            <span
                                title={todo.due.toLocaleString()}
                                class:late={!todo.completed && days < 0}
                                class:upcoming={!todo.completed && days >= 0 && days <= 5}
                            >
                                <Chip>
                                    <IconCalendar />
                                    {#if days === 0}
                                        Today
                                    {:else if days === 1}
                                        Tomorrow
                                    {:else if days === -1}
                                        Yesterday
                                    {:else if days < 0 && (!todo.completed || days >= -10)}
                                        {-days} days ago
                                    {:else if days <= 5 && days > 0 && (todo.due.weekday > DateTime.now().weekday || DateTime.now().weekday >= 6)}
                                        {todo.due.weekdayLong}
                                    {:else}
                                        {todo.due.toLocaleString()}
                                    {/if}
                                </Chip>
                            </span>
                        {/if}
                        {#if todo.subject}
                            <Chip>
                                <IconBook />
                                {todo.subject?.name}
                            </Chip>
                        {/if}
                    </div>
                {/if}
            </div>
            <svelte:self
                {todos}
                {showSubject}
                {addChild}
                {edit}
                nested
                rootTodos={todos.filter(({parent}) => parent === todo.id)}
            />
        </li>
    {/each}
</ul>

<style lang="sass">
    @import "$lib/vars"

    ul
        list-style: none
        margin-left: 1em

        &.nested
            margin-left: 1.5em

    .todo
        margin-right: 1em
        padding: 0.5em 0.25em
        border-radius: $corners

        > div
            display: flex
            flex-direction: row
            gap: 1ch
            min-height: 1.5em

        &:hover
            background: $bg2

            .buttons
                display: block

        p
            flex: 1
            overflow-wrap: anywhere

    .buttons
        display: none

    .chips
        margin-left: 1.5em
        font-size: 0.9em

    .upcoming
        color: $warning
        font-weight: bold

    .late
        color: $danger
        font-weight: bolder
</style>
