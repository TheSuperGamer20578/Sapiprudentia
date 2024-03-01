<script lang="ts">
    import {type DateTime, Duration} from "luxon";

    export let assessments: {
        id: number;
        subject: {name: string};
        title: string;
        exam: boolean;
        status: "NOT_ISSUED" | "NOT_STARTED" | "IN_PROGRESS" | "FINISHED" | "RESULTS_RECEIVED";
        weight: number;
        due: DateTime;
        duePeriod: "BS" | "RC" | "ONE" | "TWO" | "THREE" | "FOUR" | "AS" | "MIDNIGHT";
        issued: DateTime | null;
        markOutOf: number | null;
        mark: number | null;
        notification: string | null;
        submission: string | null;
        reference: string | null;
    }[];

    const duePeriodTranslation = {
        BS: "7:30am",
        RC: "8:45am",
        ONE: "Period 1",
        TWO: "Period 2",
        THREE: "Period 3",
        FOUR: "Period 4",
        AS: "3:15pm",
        MIDNIGHT: "11:59pm",
    };
    const statusTranslation = {
        NOT_ISSUED: "Not Issued",
        NOT_STARTED: "Not Started",
        IN_PROGRESS: "In Progress",
        FINISHED: "Finished",
        RESULTS_RECEIVED: "Results Received",
    };
</script>

<ul>
    {#each assessments.sort((a, b) => a.due.toUnixInteger() - b.due.toUnixInteger()) as assessment (assessment.id)}
        <li class:exam={assessment.exam}>
            <span class="subject">{assessment.subject.name}</span>
            <span class="title">{assessment.title}</span>
            <span class="weight">{assessment.weight}%</span>
            {#if assessment.mark && assessment.markOutOf}
                <span class="mark mark--full">
                    {assessment.mark}/{assessment.markOutOf}
                    ({Math.round((assessment.mark / assessment.markOutOf) * 1000) / 10}%)
                </span>
            {:else if assessment.markOutOf}
                <span class="mark mark--partial">
                    /{assessment.markOutOf}
                </span>
            {/if}
            {#if assessment.issued}
                <div class="issued">Issued: {assessment.issued.toLocaleString()}</div>
            {:else}
                <div class="issue-by">
                    Must be issued by:
                    {assessment.due.minus({weeks: 2}).toLocaleString()}
                </div>
            {/if}
            <div class="due">
                Due:
                {assessment.due.toLocaleString()}
                {duePeriodTranslation[assessment.duePeriod]}
            </div>
            <span class="status" data-status={assessment.status}>
                {statusTranslation[assessment.status]}
            </span>
        </li>
    {/each}
</ul>

<style lang="sass">
    ul
        list-style-type: none

    li
        margin-top: 1em

    .subject
        padding-right: 0.5em
        margin-right: 0.5em
        border-right: 1px solid black

    .issued, .issue-by
        margin-top: 0.25em

    .weight
        margin-inline: 0.5em 1em
</style>
