<script lang="ts">
	import { gba } from "$lib/gbaStore";

    let canvas: HTMLCanvasElement;

    $: ctx = canvas?.getContext('2d');

    const width = 16;
    const height = 16;

    $: {
        if (ctx && $gba) {
            $gba.gba.draw_palettes(ctx)
        }
    }
</script>

<canvas
    class="palette-canvas"
    bind:this={canvas}
    style="image-rendering: pixelated; 
        --width: {width};
        --height: {height};
        "
    width={width}
    height={height}
/>

<style>
    .palette-canvas {
        width: calc(8px * var(--width));
        height: calc(8px * var(--height));
        padding: 0.5em;
    }
</style>
