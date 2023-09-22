<script lang="ts">
	import { gba } from "$lib/gbaStore";

    export let bg: number;

    let tilemap_canvas: HTMLCanvasElement;

    $: tilemap = $gba?.gba.tilemap(bg);

    $: {
        if (tilemap) {
            const imageData = new ImageData(tilemap, 8, 16 * 8);
            createImageBitmap(imageData, {
                resizeWidth: 8,
                resizeHeight: 16 * 8,
                resizeQuality: 'pixelated',
            }).then((bitmap) => {
                const ctx = tilemap_canvas?.getContext('2d');
                ctx?.drawImage(bitmap, 0, 0);
            });
        }
    }
</script>

<canvas
    class="tilemap-canvas"
    bind:this={tilemap_canvas}
    style="image-rendering: pixelated"
    width={8}
    height={16*8}
/>

<style>
    .tilemap-canvas {
        width: calc(2 * 8px);
        height: calc(2 * 16 * 8px);
        padding: 0.5em;
    }
</style>
