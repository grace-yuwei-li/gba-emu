<script lang="ts">
    import { debuggerStore } from "$lib/debuggerStore";
	import { onMount } from "svelte";

    let screen_canvas: HTMLCanvasElement | undefined;

    $: debuggerData = $debuggerStore;

    onMount(() => {
        const ctx = screen_canvas?.getContext('2d');
        if (!ctx) {
            throw new Error("Failed to get screen canvas context");
        }

        let rid = requestAnimationFrame(function update() {
            let imageData = new ImageData(debuggerData.screen_array, 240);
            ctx.putImageData(imageData, 0, 0);

            rid = requestAnimationFrame(update);
        });

        return () => cancelAnimationFrame(rid);
    });

    $: {
        /*
        const canvas_data = details?.screen();
        if (canvas_data) {
            const imageData = new ImageData(canvas_data, 240, 160);
            createImageBitmap(imageData, {
                resizeWidth: 240,
                resizeHeight: 160,
                resizeQuality: 'pixelated',
            }).then((bitmap) => {
                const ctx = screen_canvas.getContext('2d');
                ctx?.drawImage(bitmap, 0, 0);
            });
        }
        */
    }
</script>

<canvas
    class="screen-canvas"
    bind:this={screen_canvas}
    style="image-rendering: pixelated"
    width={240}
    height={160}
/>

<style>
    .screen-canvas {
        width: calc(2 * 240px);
        height: calc(2 * 160px);
        padding: 1em;
    }
</style>
