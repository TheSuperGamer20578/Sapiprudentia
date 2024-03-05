<script lang="ts">
    import {DateTime} from "luxon";

    export let assessments: {
        id: number;
        subject: {name: string};
        title: string;
        exam: boolean;
        status: "NOT_ISSUED" | "NOT_STARTED" | "IN_PROGRESS" | "FINISHED" | "RESULTS_RECEIVED";
        weight: number;
        due: DateTime | null;
        duePeriod: "BS" | "RC" | "ONE" | "TWO" | "THREE" | "FOUR" | "AS" | "MIDNIGHT" | null;
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
    {#each assessments.sort((a, b) => (a.due?.toUnixInteger() ?? Infinity) - (b.due?.toUnixInteger() ?? Infinity)) as assessment (assessment.id)}
        {@const days = Math.round(
            assessment.due?.diff(DateTime.now().startOf("day")).as("days") ?? Infinity,
        )}
        <li
            class:exam={assessment.exam}
            class:fortnight={days <= 14}
            class:week={days <= 7}
            class:soon={days <= 2}
            class:late={days <= 0}
        >
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
                <div class="issued">
                    Issued:
                    {assessment.issued.weekdayLong}
                    {assessment.issued.toLocaleString()}
                </div>
            {:else if assessment.due}
                {@const issue_by = assessment.due.minus({weeks: 2})}
                <div class="issue-by">
                    Must be issued by:
                    {issue_by.weekdayLong}
                    {issue_by.toLocaleString()}
                </div>
            {/if}
            <div class="due">
                {#if assessment.due}
                    Due:
                    {assessment.due.weekdayLong}
                    {assessment.due.toLocaleString()}
                    {#if assessment.duePeriod}
                        {duePeriodTranslation[assessment.duePeriod]}
                    {/if}
                {:else}
                    Examination Block
                {/if}
            </div>
            <span class="status" data-status={assessment.status}>
                {#if assessment.exam && assessment.status === "NOT_STARTED"}
                    Exam
                {:else}
                    {statusTranslation[assessment.status]}
                {/if}
            </span>
        </li>
    {/each}
</ul>

<style lang="sass">
    @import "$lib/vars"

    ul
        list-style-type: none

    li
        margin-top: 1em
        border-left: 3px solid $fg-accent
        padding: 0.5em 1em
        &.exam
            border-color: $warning
        &:has(.status[data-status="NOT_ISSUED"])
            color: $fg-placeholder
        &.fortnight .issue-by
            color: $danger
            font-weight: bolder
        &:not(:has(.status[data-status="FINISHED"], .status[data-status="RESULTS_RECIEVED"]))
            &.fortnight:not(.exam) .due
                font-weight: bold
            &.week:not(.exam) .due
                font-weight: bolder
                color: $fg-accent
            &.soon:not(.exam) .due
                color: $warning
            &.soon.exam .due
                color: $fg-accent
            &.late .due
                color: $danger

    .subject
        padding-right: 0.5em
        margin-right: 0.5em
        border-right: 1px solid

    .issued, .issue-by
        margin-top: 0.25em

    .issued
        color: $fg-placeholder

    .weight
        margin-inline: 0.5em 1em

    .status
        display: inline-block
        padding: 0.25em 0.5em
        border-radius: 1em
        margin-top: 0.25em
        &[data-status="NOT_ISSUED"]
            color: $fg-placeholder
            background: $bg-placeholder
        &[data-status="NOT_STARTED"]
            color: $fg
            background: $bg-placeholder
        &[data-status="IN_PROGRESS"]
            color: $fg
            background: $bg-accent
        &[data-status="FINISHED"],
        &[data-status="RESULTS_RECEIVED"]
            color: $fg
            background: $success
</style>
