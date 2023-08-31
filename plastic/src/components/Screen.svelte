<script lang="ts">
	import { gba } from "$lib/gbaStore";

    let canvas: HTMLCanvasElement;

    $: details = $gba?.inspect_ppu();

    $: {
        const canvas_data = details?.screen();
        if (canvas_data) {
            const imageData = new ImageData(canvas_data, 240, 160);
            createImageBitmap(imageData, {
                resizeWidth: 240,
                resizeHeight: 160,
                resizeQuality: 'pixelated',
            }).then((bitmap) => {
                const ctx = canvas.getContext('2d');
                ctx?.drawImage(bitmap, 0, 0);
            });
        }
    }
</script>

<canvas
    bind:this={canvas}
    style="image-rendering: pixelated"
    width={240}
    height={160}
/>

<style>
    canvas {
        width: calc(3 * 240px);
        height: calc(3 * 160px);
        padding: 1em;
    }
</style>
