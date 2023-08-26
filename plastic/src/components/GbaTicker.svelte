<script lang="ts">
	import { gba, init } from '$lib/gbaStore';
	import { onDestroy, onMount } from 'svelte';

    export let clockSpeed: number;

    let start: number;

    const tickGba: FrameRequestCallback = (timestamp) => {
        if (start === undefined) {
            start = timestamp; 
        }
        const elapsedMillis = timestamp - start;

        const numTicks = Math.ceil(clockSpeed * elapsedMillis / 1000);

        for (let i = 0; i < numTicks; i++) {
            $gba?.tick();
        }
        $gba = $gba;

        start = timestamp;

        requestAnimationFrame(tickGba);
    }


	onMount(async () => {
        await init();

        requestAnimationFrame(tickGba);

        console.log('mounted');
	});

    onDestroy(() => {
        $gba?.free();
        $gba = undefined;
        
        console.log('destroyed');
    });
</script>
