<script lang="ts">
    import { reset } from '$lib/gbaStore';

    export let clockSpeed: number = 10000000;
</script>

<div id="toolbar" >
    <button on:click={reset}>Reset</button>
    <label>
        Clock speed (hz):
        <input type="number" bind:value={clockSpeed} />
    </label>
</div>

<style>
    #toolbar {
        width: 100%;
        height: 100px;
        background-color: purple;
        color: white;
    }
</style>
