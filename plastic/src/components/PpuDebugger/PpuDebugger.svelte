<script lang="ts">
	import Tilemap from "../Tilemap.svelte";
	import PalettesCanvas from "./PalettesCanvas.svelte";
	import TilesCanvas from "./TilesCanvas.svelte";

    let palette: number = 0;
    let use_256_colors: boolean = false;

</script>

<div id="ppu-debugger">
    <div>
        Tilemaps?? What are these
        <div>
            <Tilemap bg={0} />
        </div>
        <div>
            <Tilemap bg={1} />
        </div>
        <div>
            <Tilemap bg={2} />
        </div>
        <div>
            <Tilemap bg={3} />
        </div>
    </div>
    <div>
        <h2>
            Tiles
        </h2>
        <label>
            Palette
            <input type="number" min=0 max=15 bind:value={palette}>
        </label>
        <label>
            256 Colours
            <input type="checkbox" bind:checked={use_256_colors}>
        </label>
        <TilesCanvas palette={palette} use_256_colors={use_256_colors} />
    </div>
    <div>
        <PalettesCanvas />
    </div>
</div>

