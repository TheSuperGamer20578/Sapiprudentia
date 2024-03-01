<script lang="ts">
    export let subjects: {id: number; name: string}[];
</script>

<nav>
    <ul>
        <li><a href="/todo">Todo</a></li>
        <li><a href="/notes">Notes</a></li>
        <li><a href="/assessments">Assessments</a></li>
    </ul>
    <hr />
    <ul>
        {#each subjects as { id, name } (id)}
            <li><a href="/subjects/{id}">{name}</a></li>
        {/each}
        <li><a href="/subjects/">All Subjects</a></li>
    </ul>
</nav>

<style lang="sass">
    @import "$lib/vars"

    ul
        list-style: none
        margin-inline: $padding
        margin-bottom: calc($padding / 4)

        li
            margin-top: calc($padding / 4)

        a
            text-decoration: none
            color: inherit
            padding: calc($padding / 4) calc($padding / 2)
            border-radius: $corners
            display: block

            &:hover
                background: $bg3

    hr
        border-style: none
        border-bottom: $div-border
        outline: none
</style>
