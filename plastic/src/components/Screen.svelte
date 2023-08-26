<script lang="ts">
	import { gba } from "$lib/gbaStore";
	import { onMount } from "svelte";

    let canvas: HTMLCanvasElement;
    const screenScale = 3;

    const tick = () => {
        const details = $gba?.inspect_ppu();

        const canvas_data = details?.screen();
        if (canvas_data) {
            const imageData = new ImageData(canvas_data, 240);
            createImageBitmap(imageData, {
                resizeWidth: 240 * 3,
                resizeHeight: 160 * 3,
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
