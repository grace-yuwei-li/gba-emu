<script lang="ts">
	import { init, tick } from '$lib/gbaStore';
    import { addFrameTime } from '$lib/frameTimeStore';
	import { onMount } from 'svelte';

    export let clockSpeed: number;

    let start: number;
    let tickDebt: number = 0;

    const tickGba: FrameRequestCallback = (timestamp) => {
        if (start === undefined) {
            start = timestamp; 
        }
        const elapsedMillis = timestamp - start;

        tickDebt += clockSpeed * elapsedMillis / 1000;

        const numTicks = Math.floor(tickDebt);

        // Record how long this frame takes
        const frameStart = performance.now();
        tick(numTicks);
        const frameEnd = performance.now();
        addFrameTime(frameEnd - frameStart);

        tickDebt -= numTicks

        start = timestamp;

        requestAnimationFrame(tickGba);
    }

	onMount(async () => {
        await init();

        requestAnimationFrame(tickGba);
	});
</script>
