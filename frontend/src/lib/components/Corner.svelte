<div class="outer">
    <div class="inner">
        <slot />
    </div>
</div>

<style lang="sass">
    .outer
        position: relative
        height: 0

    .inner
        position: absolute
        top: 0
        right: 0
</style>
