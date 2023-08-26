<script lang="ts">
	import { gba } from "$lib/gbaStore";
    import { disassemble_arm } from '$lib/pkg/debug/gba_core';

    interface Line {
        address: number,
        content: string,
    }

	let debuggerHeight: number = 0;
	let scrollbar: HTMLDivElement;

	const lineHeight = 20;
	const totalLineCount = 4294967296; // 2 ^ 32

	const gutterWidth = 100;
	const scrollbarWidth = 20;

	const totalHeight = totalLineCount * lineHeight;
    // Cap scrollbar height
	const scrollbarHeight = Math.min(totalHeight, 100000);

	$: visibleLineCount = Math.ceil(debuggerHeight / lineHeight) + 1;

	let contentTop = 0x8000000 / 4 * lineHeight;
    $: firstLine = Math.floor(contentTop / lineHeight);

    let lines: Line[];
	$: lines = new Array(visibleLineCount).fill(0).map((_, index) => getLine(firstLine + index));

	const handleScrollScrollbar = (event: UIEvent) => {
		const target = event.target as HTMLElement;

		const scrollRatioTop = target.scrollTop / scrollbarHeight;
        const scrollRatioBot = (target.scrollTop + target.clientHeight) / scrollbarHeight;

        contentTop = scrollRatioTop * totalHeight;
        /*
        if (1 - scrollRatioTop > scrollRatioBot) {
            contentTop = scrollRatioTop * totalHeight;
        } else {
            contentTop = scrollRatioBot * totalHeight - target.clientHeight;
        }
        */
	};

	const handleWheelContent = (event: WheelEvent) => {
        const scale = 0.2;

        event.deltaY * scale;
        const upperLimit = totalHeight - debuggerHeight;
        contentTop = Math.min(upperLimit, Math.max(0, contentTop + event.deltaY * scale));

        if (scrollbar) {
            const scrollPercentage = contentTop / totalHeight;
            scrollbar.scrollTo(0, scrollPercentage * scrollbarHeight);
        }
    };

	$: getLine = (index: number): Line => {
        if (index >= totalLineCount) {
            console.error(index);
        }

        let disassembly;
        try {
            const memValue = $gba?.read_address(index * 4);
            if (memValue !== undefined) {
                disassembly = disassemble_arm(memValue);
            } else {
                disassembly = undefined;
            }
        } catch {
            disassembly = undefined;
        }
        return {
            //address: `${(index * 4).toString(16).toUpperCase()}`,
            address: index * 4,
            content: disassembly ?? 'undefined'
        }
    };

    let toolbarAddress: string;

    const handleAddressChange = () => {
        let address = parseInt(toolbarAddress, 16);

        if (!isNaN(address)) {
            const targetLine = Math.floor(address / 4);
            const upperLimit = totalHeight - debuggerHeight;
            contentTop = Math.min(upperLimit, Math.max(0, targetLine * lineHeight));

            if (scrollbar) {
                const scrollPercentage = contentTop / totalHeight;
                scrollbar.scrollTo(0, scrollPercentage * scrollbarHeight);
            }
        }
    }
</script>

<div id="debugger-wrapper">
    <div id="debugger-toolbar">
        <form on:submit|preventDefault={handleAddressChange}>
            <input type="text" bind:value={toolbarAddress} />
            <input type="submit" value="Go To Address" />
        </form>
    </div>
    <div 
        id="debugger" 
        on:wheel={handleWheelContent}
        bind:clientHeight={debuggerHeight}
    >
        <div
            class="gutter"
            style="
                width: {gutterWidth}px;
                top: {-contentTop % lineHeight}px;
            "
        >
            {#each lines as { address }, index (address)}
                <div class="cell gutter-cell" style="--lineHeight:{lineHeight}; top: {lineHeight * index}px">
                    {`${address.toString(16).toUpperCase()}`}
                </div>
            {/each}
        </div>
        <div
            class="content"
            style="
                left: {gutterWidth}px; 
                right: 100px;
                top: {-contentTop % lineHeight}px;
            "
        >
            {#each lines as { address, content }, index (address)}
                <div 
                    class="cell content-cell {$gba?.inspect_cpu().pc === address ? 'selected' : 'not-selected'}" 
                    style="--lineHeight:{lineHeight}; top: {lineHeight * index}px"
                >
                    {content}
                </div>
            {/each}
        </div>
        <div
            class="scrollbar"
            style="width: {scrollbarWidth}px"
            bind:this={scrollbar}
            on:scroll|preventDefault
            on:wheel|preventDefault|stopPropagation
        >
            <div class="scrollbar-inner" style="height: {scrollbarHeight}px" />
        </div>
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
    }

	#debugger {
        flex-grow: 1;
		position: relative;
		width: 500px;
		overflow: hidden;
        font-family: monospace;
        background-color: white;
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

    .selected {
        background-color: gray;
        color: white;
    }

	.cell {
        position: absolute;
        left: 5px;
        right: 5px;
		height: calc(var(--lineHeight) * 1px);
	}

	.scrollbar {
		position: absolute;
		top: 0;
		right: 0;
		overflow-y: scroll;
        height: 100%;
	}
</style>
