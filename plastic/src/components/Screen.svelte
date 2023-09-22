<script lang="ts">
	import { gba } from "$lib/gbaStore";

    let screen_canvas: HTMLCanvasElement;

    $: details = $gba?.ppu;

    $: {
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
