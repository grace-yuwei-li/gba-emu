<script lang="ts">
	import { gba } from "$lib/gbaStore";
	import { onMount } from "svelte";

    let canvas: HTMLCanvasElement;
    const screenScale = 2;

    const tick = () => {
        const details = $gba?.inspect_ppu();

        const canvas_data = details?.vram();
        if (canvas_data) {
            const imageData = new ImageData(canvas_data, 240);
            createImageBitmap(imageData, {
                resizeWidth: 240 * 2,
                resizeHeight: 160 * 2,
                resizeQuality: 'pixelated',
            }).then((bitmap) => {
                const ctx = canvas.getContext('2d');
                ctx?.drawImage(bitmap, 0, 0);
            });
        }

        requestAnimationFrame(tick) 
    };

    onMount(() => {
        requestAnimationFrame(tick) 
    });
</script>

<canvas
    bind:this={canvas}
    style="image-rendering: pixelated"
    width={240 * screenScale}
    height={160 * screenScale}
/>
