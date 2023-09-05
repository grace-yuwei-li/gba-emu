<script lang="ts">
    import { gba, reset, tick } from '$lib/gbaStore';

    export let clockSpeed: number = 0;

    const resume = () => {
        $gba?.gba.enable_debugger(false);
        $gba?.gba.set_stopped(false);
        tick(1);
        $gba?.gba.enable_debugger(true);
    }

    const step = () => {
        $gba?.gba.enable_debugger(false);
        $gba?.gba.set_stopped(false);
        tick(1);
        $gba?.gba.set_stopped(true);
        $gba?.gba.enable_debugger(true);
        $gba = $gba;
    }

    const handleReset = () => {
        let arm_breakpoints = Array.from($gba?.gba.arm_breakpoints() ?? []);
        let thumb_breakpoints = Array.from($gba?.gba.thumb_breakpoints() ?? []);
        reset();
        for (const bp of arm_breakpoints) {
            $gba?.gba.add_arm_breakpoint(bp);
        }
        for (const bp of thumb_breakpoints) {
            $gba?.gba.add_thumb_breakpoint(bp);
        }
    }
</script>

<div id="toolbar" >
    <button on:click={handleReset}>Reset</button>
    <button on:click={resume}>Resume</button>
    <button on:click={step}>Step</button>
    <label>
        Clock speed (hz):
        <input type="number" bind:value={clockSpeed} />
    </label>
    <span>PC: 0x{$gba?.cpu.pc().toString(16)}</span>
    <span>Thumb: {$gba?.gba.thumb_state()}</span>
</div>

<style>
    #toolbar {
        width: 100%;
        height: 100px;
        background-color: purple;
        color: white;
    }
</style>
