<script lang="ts">
	import { gbaStore } from "$lib/gbaStore";
    //import { disassemble_arm, disassemble_thumb } from '$lib/pkg/debug/gba_core';
    import { type LineData, InstructionMode } from '$lib/debugger';
	import Line from "./Line.svelte";

	let debuggerHeight: number = 0;
    let gba = $gbaStore;

	const lineHeight = 20;

    // Canonical position
    let topAddress = 0x8000000;
    let toolbarAddress: string;
    let instructionMode = InstructionMode.Auto;

    // Determines gap between instruction addresses
    let instructionSize: 2 | 4; 
    $: if (instructionMode === InstructionMode.Auto) {
        instructionSize = 2;
        /*
        if (true || gba?.gba.thumb_state()) {
            instructionSize = 2;
        } else {
            instructionSize = 4;
        }
        */
    } else if (instructionMode === InstructionMode.Arm) {
        instructionSize = 4;
    } else if (instructionMode === InstructionMode.Thumb) {
        instructionSize = 2;
    } else {
        throw new Error(`Invalid instruction mode ${instructionMode}`);
    }

    // Lines to be displayed in table
    let lines: LineData[];
	$: visibleLineCount = Math.min(Math.ceil(debuggerHeight / lineHeight) + 1, 10000);
	$: lines = new Array(visibleLineCount).fill(0).map((_, index) => {
        const flooredTopAddress = Math.floor(topAddress / instructionSize) * instructionSize;
        return getLine( flooredTopAddress + index * instructionSize);
    });

    // Map of breakpoints
    let arm_breakpoints: Record<number, boolean>;
    let thumb_breakpoints: Record<number, boolean>;
    /*
    $: arm_breakpoints = Object.fromEntries(Array.from($gba?.gba.arm_breakpoints() ?? [])
                        .map((address:number) => [address, true]));
    $: thumb_breakpoints = Object.fromEntries(Array.from($gba?.gba.thumb_breakpoints() ?? [])
                        .map((address:number) => [address, true]));
    */
    arm_breakpoints = {};
    thumb_breakpoints = {};

    const instructionModeOptions = [
        { label: "Auto", value: InstructionMode.Auto },
        { label: "Arm", value: InstructionMode.Arm },
        { label: "Thumb", value: InstructionMode.Thumb },
    ];

    // Scroll event handler
	const handleWheel = (event: WheelEvent) => {
        if (event.deltaY > 0) {
            topAddress += instructionSize;
        } else {
            topAddress -= instructionSize;
        }
    };
    // Gets the value in memory and the disassembly at the given address
	$: getLine = (address: number): LineData => {
        let disassembly: string | undefined = undefined;

        let memValue: number | undefined = undefined;
        try {
            //memValue = $gba?.gba.read_address(address);
            memValue = 0
        } catch {
            // Do nothing, disassembly is already undefined
        }

        if (memValue !== undefined) {
            if (instructionSize == 2) {
                memValue &= 0xffff;
            }

            if (instructionSize === 2) {
                //disassembly = disassemble_thumb(memValue);
                disassembly = "Invalid instruction size";
            } else if (instructionSize === 4) {
                //disassembly = disassemble_arm(memValue);
                disassembly = "Invalid instruction size";
            } else {
                disassembly = "Invalid instruction size";
            }
        }

        return {
            address,
            value: memValue,
            content: disassembly,
        }
    };

    // Submitted by the form to jump to an address
    const handleAddressChange = () => {
        let address = parseInt(toolbarAddress, 16);

        if (!isNaN(address)) {
            topAddress = address;
        }
    }

    /*
    const toggleBreakpoint = (address: number) => {
        if (instructionSize === 4) {
            if (arm_breakpoints[address]) {
                $gba?.gba.remove_arm_breakpoint(address);
            } else {
                $gba?.gba.add_arm_breakpoint(address);
            }
        } else if (instructionSize === 2) {
            if (thumb_breakpoints[address]) {
                $gba?.gba.remove_thumb_breakpoint(address);
            } else {
                $gba?.gba.add_thumb_breakpoint(address);
            }
        }
        $gba = $gba;
    };
    */
</script>

<div id="debugger-wrapper">
    <div id="debugger-toolbar">
        <form on:submit|preventDefault={handleAddressChange}>
            <input type="text" bind:value={toolbarAddress} />
            <input type="submit" value="Go To Address" />
        </form>
        <div style="display: flex; flex-direction: row; align-items: center;">
            {#each instructionModeOptions as { label, value }, index (index)}
                <label class="instruction-mode-label">
                    {label}
                    <input type="radio" name="instructionModeSelect" bind:group={instructionMode} value={value} />
                </label>
            {/each}
        </div>
    </div>
    <div 
        id="debugger" 
        on:wheel={handleWheel}
        bind:clientHeight={debuggerHeight}
    >
        <table>
            {#each lines as line (line.address)}
                <Line 
                    line={line} 
                    lineHeight={lineHeight} 
                    instructionSize={instructionSize} 
                    isExecuting={false && $gba?.cpu.executing_pc === line.address}
                    isPc={false && $gba?.cpu.pc() === line.address}
                    isBreakpoint={Boolean(instructionSize === 4 ? arm_breakpoints[line.address] : thumb_breakpoints[line.address])}
                    toggleBreakpoint={() => {}} />
            {/each}
        </table>
    </div>
</div>

<style>
    #debugger-wrapper {
        display: flex;
        flex-direction: column;
        height: 100%;
        color: black;
    }

    #debugger-toolbar {
        height: 20px;
        margin: 1em;
        display: flex;
        flex-direction: row;
    }

	#debugger {
        flex-grow: 1;
		position: relative;
		width: 500px;
		overflow: hidden;
        background-color: white;
        height: 100%;
	}

    .instruction-mode-label {
        display: flex;
        flex-direction: column;
        margin: 0.5em;
        color: white;
    }
</style>
