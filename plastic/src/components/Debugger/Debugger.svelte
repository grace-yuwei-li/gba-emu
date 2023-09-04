<script lang="ts">
	import { gba } from "$lib/gbaStore";
    import { disassemble_arm } from '$lib/pkg/debug/gba_core';
    import { type LineData, InstructionMode } from '$lib/debugger';
	import Line from "./Line.svelte";

	let debuggerHeight: number = 0;

	const lineHeight = 20;

    // Canonical position
    let topAddress = 0x8000000;
    let toolbarAddress: string;
    let instructionMode = InstructionMode.Auto;

    // Determines gap between instruction addresses
    let instructionSize: 2 | 4; 
    $: if (instructionMode === InstructionMode.Auto) {
        if ($gba?.gba.thumb_state()) {
            instructionSize = 2;
        } else {
            instructionSize = 4;
        }
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
    let breakpoints: Record<number, boolean>;
    $: breakpoints = Object.fromEntries(Array.from($gba?.gba.breakpoints() ?? [])
                        .map((address:number) => [address, true]));

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
            memValue = $gba?.gba.read_address(address);
        } catch {
            // Do nothing, disassembly is already undefined
        }

        if (memValue !== undefined) {
            if (instructionSize == 2) {
                memValue &= 0xffff;
            }

            if (instructionSize === 2) {
                disassembly = "No THUMB disassembly yet";
            } else if (instructionSize === 4) {
                disassembly = disassemble_arm(memValue);
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

    const toggleBreakpoint = (address: number) => {
        if (breakpoints[address]) {
            $gba?.gba.remove_breakpoint(address);
        } else {
            $gba?.gba.add_breakpoint(address);
        }
        $gba = $gba;
    };
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
                    isExecuting={$gba?.cpu.executing_pc === line.address}
                    isPc={$gba?.cpu.pc() === line.address}
                    isBreakpoint={Boolean(breakpoints[line.address])}
                    toggleBreakpoint={toggleBreakpoint} />
            {/each}
        </table>
    </div>
</div>

<style>
    #debugger-wrapper {
        display: flex;
        flex-direction: column;
        height: 100%;
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
		width: 700px;
		overflow: hidden;
        background-color: white;
        height: 100%;
	}

	.gutter {
		position: absolute;
		top: 0;
		left: 0;
        text-align: end;
	}

	.content {
		position: absolute;
		top: 0;
	}

    .instruction-mode-label {
        display: flex;
        flex-direction: column;
        margin: 0.5em;
        color: white;
    }
</style>
