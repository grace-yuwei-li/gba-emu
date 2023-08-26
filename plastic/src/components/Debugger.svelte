<script lang="ts">
    interface Line {
        address: string,
        content: string,
    }

	const lineHeight = 20;
	let debuggerHeight: number = 100;

	const getLine = (index: number) => {
        if (index >= totalLineCount) {
            console.error(index);
        }
        return {
            address: `${(index * 4).toString(16).toUpperCase()}`,
            content: 'Hello',
        }
    };

	const totalLineCount = 4294967296; // 2 ^ 32
	//const totalLineCount = 16*16;

	$: visibleLineCount = Math.ceil(debuggerHeight / lineHeight) + 1;


	const gutterWidth = 100;
	const scrollbarWidth = 20;

	const totalHeight = totalLineCount * lineHeight;
	const scrollbarHeight = Math.min(totalHeight, 1000000);

	let contentTop = 0;
    $: firstLine = Math.floor(contentTop / lineHeight);

    let lines: Line[];
	$: lines = new Array(visibleLineCount).fill(0).map((_, index) => getLine(firstLine + index));

	let scrollbar: HTMLDivElement;

	const handleScrollScrollbar = (event: UIEvent) => {
		const target = event.target as HTMLElement;

		const scrollRatioTop = target.scrollTop / scrollbarHeight;
        const scrollRatioBot = (target.scrollTop + target.clientHeight) / scrollbarHeight;

        if (1 - scrollRatioTop > scrollRatioBot) {
            contentTop = scrollRatioTop * totalHeight;
        } else {
            contentTop = scrollRatioBot * totalHeight - target.clientHeight;
        }
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
</script>

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
				{address}
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
			<div class="cell content-cell" style="--lineHeight:{lineHeight}; top: {lineHeight * index}px">
				{content}
			</div>
		{/each}
	</div>
	<div
		class="scrollbar"
		style="width: {scrollbarWidth}px"
        bind:this={scrollbar}
		on:scroll={handleScrollScrollbar}
        on:wheel|stopPropagation
	>
		<div class="scrollbar-inner" style="height: {scrollbarHeight}px" />
	</div>
</div>

<style>
	#debugger {
		position: relative;
		width: 500px;
        height: 100%;
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
