<script lang="ts">
    import { gba, reset } from '$lib/gbaStore';

    export let clockSpeed: number = 0;

    const tick = () => {
        $gba?.enable_debugger(false);
        $gba?.set_stopped(false);
        $gba?.tick();
        $gba?.enable_debugger(true);
        $gba = $gba;
        console.log($gba?.inspect_cpu().pc().toString(16));
    }

    const handleReset = () => {
        let breakpoints = Array.from($gba?.breakpoints() ?? []);
        reset();
        for (const bp of breakpoints) {
            $gba?.add_breakpoint(bp);
        }
    }
</script>

<div id="toolbar" >
    <button on:click={handleReset}>Reset</button>
    <button on:click={tick}>Tick</button>
    <label>
        Clock speed (hz):
        <input type="number" bind:value={clockSpeed} />
    </label>
    <span>PC: 0x{$gba?.inspect_cpu().pc().toString(16)}</span>
</div>

<style>
    #toolbar {
        width: 100%;
        height: 100px;
        background-color: purple;
        color: white;
    }
</style>
