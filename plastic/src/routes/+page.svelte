<script lang="ts">
	import { handleKeyDown, handleKeyUp } from "$lib/keys";
	import Debugger from "../components/Debugger/Debugger.svelte";
	import EmuInfo from "../components/EmuInfo.svelte";
	import GbaTicker from "../components/GbaTicker.svelte";
	import Screen from "../components/Screen.svelte";
	import Toolbar from "../components/Toolbar.svelte";

    let clockSpeed: number;

</script>

<GbaTicker clockSpeed={clockSpeed} />
<Toolbar bind:clockSpeed={clockSpeed} />
<div id="main">
    <div>
        <Debugger />
    </div>
    <div class="column" >
        <div>
            <EmuInfo />
        </div>
        <div id="screen-wrapper" on:keydown={handleKeyDown} on:keyup={handleKeyUp} tabindex={0}>
            <Screen />
        </div>
    </div>
</div>

<style>
    :global(body) {
        background-color: #1e1f43;
        margin: 0;
        height: 100vh;
        display: flex;
        flex-direction: column;
        font-family: monospace;
    }

    #main {
        flex-grow: 1;
        display: flex;
        position: relative;
        flex-direction: row;
        align-items: stretch;
        /* Prevents overflow */
        min-height: 0;
    }

    #screen-wrapper {
        flex-grow: 1;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .column {
        display: flex;
        flex-direction: column;
        width: 100%;
    }
</style>
