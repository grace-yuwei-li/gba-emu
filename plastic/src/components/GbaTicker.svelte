<script>
	import { gba } from '$lib/gbaStore';
	import { onDestroy, onMount } from 'svelte';
    import initWasm, { GbaCore } from '$lib/pkg/debug/gba_core';

    const tickGba = () => {
        const start = performance.now();
        for (let i = 0; i < 10; i++) {
            $gba?.tick();
        }
        const elapsed = performance.now() - start;

        //console.log(`${elapsed} ms elapsed`)
		//console.log($gba?.inspect_cpu().pc);

        requestAnimationFrame(tickGba);
    }


	onMount(async () => {
        await initWasm();
        $gba = new GbaCore();
        $gba.skip_bios();
        $gba.load_test_rom();

        requestAnimationFrame(tickGba);

        console.log('mounted');
	});

    onDestroy(() => {
        $gba?.free();
        $gba = undefined;
        
        console.log('destroyed');
    });
</script>
