<script lang="ts">
    import ConfettiContainer from "$lib/components/ConfettiContainer.svelte";
    import {Confetti} from "svelte-confetti";
    import {Howl} from "howler";
    import {createEventDispatcher} from "svelte";

    const sound = new Howl({
        src: ["/complete.wav"],
    });
    const dispatch = createEventDispatcher();

    export let checked = false;
    let clicked = false;
</script>

<ConfettiContainer active={clicked && checked}>
    <input
        slot="label"
        type="checkbox"
        bind:checked
        on:change={(e) => {
            clicked = true;
            if (checked) {
                sound.play();
            }
            dispatch("change", e);
        }}
    />
    <Confetti x={[-0.5, 0.5]} />
    <Confetti amount={10} x={[-0.75, -0.3]} y={[0.15, 0.75]} />
    <Confetti amount={10} x={[0.3, 0.75]} y={[0.15, 0.75]} />
</ConfettiContainer>

<style lang="sass">
    input
        width: min-content
        height: min-content
        margin: 0.2em
</style>
