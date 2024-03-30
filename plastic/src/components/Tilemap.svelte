<script lang="ts">
	import { gbaStore } from "$lib/gbaStore";

    export let bg: number;

    let gba = $gbaStore;
    let tilemap_canvas: HTMLCanvasElement;

    const width = 16 * 8;
    const height = 8;

    //$: tilemap = gba?.gba.tilemap(bg);

    $: {
        /*
        if (tilemap) {
            const arr = new Uint8ClampedArray(tilemap.buffer);
            const imageData = new ImageData(arr, width, height);
            createImageBitmap(imageData, {
                resizeWidth: width,
                resizeHeight: height,
                resizeQuality: 'pixelated',
            }).then((bitmap) => {
                const ctx = tilemap_canvas?.getContext('2d');
                ctx?.drawImage(bitmap, 0, 0);
            });
        }
        */
    }
</script>

<canvas
    class="tilemap-canvas"
    bind:this={tilemap_canvas}
    style="image-rendering: pixelated; 
        --width: {width};
        --height: {height};
        "
    width={width}
    height={height}
/>

<style>
    .tilemap-canvas {
        width: calc(3px * var(--width));
        height: calc(3px * var(--height));
        padding: 0.5em;
    }
</style>
