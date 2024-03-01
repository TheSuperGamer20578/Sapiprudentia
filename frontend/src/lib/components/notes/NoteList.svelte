<script lang="ts">
    import {DateTime} from "luxon";

    export let notes: {
        id: number;
        title: string;
        date: DateTime;
        subject?: {name: string} | null;
    }[];
</script>

<h1>Notes</h1>
<ul class="dates">
    {#each Object.entries(Object.groupBy( notes, ({date}) => date.toString(), )).sort(([a], [b]) => DateTime.fromISO(b).toUnixInteger() - DateTime.fromISO(a).toUnixInteger()) as [date, notes] (date)}
        <li>
            <h2>
                {DateTime.fromISO(date).toLocaleString()}
            </h2>
            <ul class="notes">
                {#each notes as note (note.id)}
                    <li>
                        <a href="/notes/{note.id}">
                            {#if note.subject}
                                <span class="subject">
                                    {note.subject.name}
                                </span>
                            {/if}
                            {note.title}
                        </a>
                    </li>
                {/each}
            </ul>
        </li>
    {/each}
</ul>

<style lang="sass">
    @import "$lib/vars"

    ul
        list-style-type: none

    .dates > li
        margin-top: 1em
        display: flex
        gap: 1em

        h2
            width: 10ch
            text-align: right
            font-family: $font-mono
            font-size: 1.25em

        ul
            flex: 1

    .notes > li a
        @include clickable($bg)
        color: inherit
        text-decoration: inherit
        display: block
        padding: 0.25em 1em
        border-radius: $corners

    .subject
        font-weight: bold
        padding-right: 0.5em
        margin-right: 0.75em
        border-right: $fg-placeholder solid 1px
</style>
