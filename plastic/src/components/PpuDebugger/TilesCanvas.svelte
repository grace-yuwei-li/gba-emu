<script lang="ts">
	import { gba } from "$lib/gbaStore";

    export let palette: number;
    export let use_256_colors: boolean;

    let tiles_canvas: HTMLCanvasElement;

    $: ctx = tiles_canvas?.getContext('2d');

    const width = 32 * 8;
    const height = 32 * 8;

    $: {
        if (ctx && $gba) {
            const palette16 = use_256_colors ? undefined : palette;
            $gba.gba.draw_tiles(ctx, palette16);
        }
    }
</script>

<canvas
    class="tiles-canvas"
    bind:this={tiles_canvas}
    style="image-rendering: pixelated; 
        --width: {width};
        --height: {height};
        "
    width={width}
    height={height}
/>

<style>
    .tiles-canvas {
        width: calc(2px * var(--width));
        height: calc(2px * var(--height));
        padding: 0.5em;
    }
</style>
