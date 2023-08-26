<script lang="ts">
    import { gba, reset } from '$lib/gbaStore';

    export let clockSpeed: number = 1000000;

    const tick = () => {
        $gba?.tick();
        $gba = $gba;
        console.log($gba?.inspect_cpu().pc.toString(16));
    }
</script>

<div id="toolbar" >
    <button on:click={reset}>Reset</button>
    <button on:click={tick}>Tick</button>
    <label>
        Clock speed (hz):
        <input type="number" bind:value={clockSpeed} />
    </label>
    <span>PC: 0x{$gba?.inspect_cpu().pc.toString(16)}</span>
</div>

<style>
    #toolbar {
        width: 100%;
        height: 100px;
        background-color: purple;
        color: white;
    }
</style>
