<script lang="ts">
	import Tilemap from "../Tilemap.svelte";
	import BackgroundsCanvas from "./BackgroundsCanvas.svelte";
	import PalettesCanvas from "./PalettesCanvas.svelte";
	import TilesCanvas from "./TilesCanvas.svelte";

	import { gba } from "$lib/gbaStore";

    let ppu_panel: string = "tilemaps";

    let palette: number = 0;
    let use_256_colors: boolean = false;

    let background: number = 0;

    let bg_mode = $gba?.gba.background_mode();


</script>

<div id="ppu-debugger">
    <label>
        <input type="radio" bind:group={ppu_panel} value={"tilemaps"}>
        Tilemaps??
    </label>
    <label>
        <input type="radio" bind:group={ppu_panel} value={"tiles"}>
        Tiles
    </label>
    <label>
        <input type="radio" bind:group={ppu_panel} value={"palettes"}>
        Palettes
    </label>
    <label>
        <input type="radio" bind:group={ppu_panel} value={"backgrounds"}>
        Backgrounds
    </label>
    {#if ppu_panel === "tilemaps"}
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
    {:else if ppu_panel === "tiles"}
        <div>
            <h2>
                Tiles
            </h2>
            <label>
                Palettes
                <input type="number" min=0 max=15 bind:value={palette}>
            </label>
            <label>
                256 Colours
                <input type="checkbox" bind:checked={use_256_colors}>
            </label>
            <TilesCanvas palette={palette} use_256_colors={use_256_colors} />
        </div>
    {:else if ppu_panel === "palettes"}
        <div>
            <PalettesCanvas />
        </div>
    {:else if ppu_panel === "backgrounds"}
        <div>
            <h2>Background</h2>
            <label>
                Background
                <input type="number" min=0 max=3 bind:value={background}>
            </label>
            <div>
                <span>BG Mode:</span>
                <span>{bg_mode}</span>
            </div>
            <BackgroundsCanvas background={background} />
        </div>
    {/if}
</div>

