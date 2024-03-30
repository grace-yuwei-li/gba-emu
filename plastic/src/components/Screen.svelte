<script lang="ts">
	import { gbaStore } from "$lib/gbaStore";
	import { onMount } from "svelte";

    let screen_canvas: HTMLCanvasElement | undefined;
    let screen_array = new Uint8ClampedArray(240 * 160 * 4);

    $: gba = $gbaStore;
    $: {
        if (gba) {
            console.log("setting screen array");
            gba.set_screen_array(screen_array)
        }
    }

    onMount(() => {
        const ctx = screen_canvas?.getContext('2d');

        let rid = requestAnimationFrame(function update() {
            if (gba && ctx) {
                console.log(screen_array);
                let imageData = new ImageData(screen_array, 240);
                ctx.putImageData(imageData, 0, 0);
                gba.request_screen_draw();
                gba.request_cpu_debug_info();
            }
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
