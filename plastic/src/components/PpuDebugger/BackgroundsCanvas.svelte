<script lang="ts">
	import { gbaStore } from "$lib/gbaStore";

    export let background: number;

    let canvas: HTMLCanvasElement;
    let gba = $gbaStore;

    $: ctx = canvas?.getContext('2d');
    //let background_info = $gba?.gba.background_info(background);
    let background_info = "background_info";

    $: {
        console.log(background_info);
    }

    const width = 16;
    const height = 16;

</script>

<!--
<div>
    Priority: {background_info?.priority}<br>
    Character base block: {background_info?.character_base_block}<br>
    Mosaic: {background_info?.mosaic}<br>
    Use 256 colors: {background_info?.use_256_colors}<br>
    Screen base block: {background_info?.screen_base_block}<br>
    Wraparound: {background_info?.wraparound}<br>
    Screen size: {background_info?.screen_size}
</div>
-->
