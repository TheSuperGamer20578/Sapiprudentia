<script lang="ts">
    export let subjects: {id: number; name: string}[];
    export let required: boolean = false;
    export let value = "null";
</script>

<select bind:value on:change {required}>
    <option disabled={required} value="null">{required ? "Pick a subject" : "None"}</option>
    {#each subjects as { id, name } (id)}
        <option value={id}>{name}</option>
    {/each}
</select>
