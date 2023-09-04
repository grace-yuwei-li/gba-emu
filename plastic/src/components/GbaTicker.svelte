<script lang="ts">
	import { gba, init, tick } from '$lib/gbaStore';
	import { onMount } from 'svelte';

    export let clockSpeed: number;

    let start: number;

    const tickGba: FrameRequestCallback = (timestamp) => {
        if (start === undefined) {
            start = timestamp; 
        }
        const elapsedMillis = timestamp - start;

        const numTicks = Math.ceil(clockSpeed * elapsedMillis / 1000);

        tick(numTicks);

        start = timestamp;

        requestAnimationFrame(tickGba);
    }

	onMount(async () => {
        await init();

        requestAnimationFrame(tickGba);

        console.log('mounted');
        console.log($gba);
	});
</script>
