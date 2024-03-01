<script lang="ts">
    import type {PageData} from "./$houdini";
    import AssessmentList from "$lib/components/AssessmentList.svelte";
    import Spinner from "$lib/components/Spinner.svelte";

    export let data: PageData;
    $: ({Assessments} = data);
    $: assessments = $Assessments.data!.assessments!;
</script>

<h1>Assessments</h1>
{#if $Assessments.fetching}
    <Spinner />
{:else}
    <AssessmentList {assessments} />
{/if}
