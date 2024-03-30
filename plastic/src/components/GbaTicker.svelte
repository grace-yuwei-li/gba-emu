<script lang="ts">
	import { init, gbaStore } from '$lib/gbaStore';
    import { addFrameTime } from '$lib/frameTimeStore';
	import { onMount } from 'svelte';

    export let clockSpeed: number;
    $: gba = $gbaStore;

    let start: number;
    let tickDebt: number = 0;
    let rid: number;

    const tickGba: FrameRequestCallback = (timestamp) => {
        if (gba) {
            gba.process_responses();
        } else {

        }
        rid = requestAnimationFrame(tickGba);
    }

	onMount(() => {
        async function foo() {
            await init();
        }
        foo();

        rid = requestAnimationFrame(tickGba);

        // Maybe we should free the old GBA as well?
        return () => cancelAnimationFrame(rid);
	});
</script>
