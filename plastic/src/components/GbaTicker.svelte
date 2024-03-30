<script lang="ts">
	import { init, gbaStore } from '$lib/gbaStore';
    import { updateDebuggerData } from '$lib/debuggerStore';
	import { onMount } from 'svelte';

    $: gba = $gbaStore;

    let rid: number;
    function tickGba() {
        if (gba) {
            updateDebuggerData(gba);
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
