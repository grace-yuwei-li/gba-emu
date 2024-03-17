<script lang="ts">
	import { gba } from "$lib/gbaStore";
	import Tilemap from "./Tilemap.svelte";

    //$: tiles = $gba?.gba.tiles(bg);
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
            <input type="number" min=0 max=15 value=0>
        </label>
        <label>
            256 Colours
            <input type="checkbox">
        </label>
    </div>
</div>

