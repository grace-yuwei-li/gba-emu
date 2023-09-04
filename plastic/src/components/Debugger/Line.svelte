<script lang="ts">
	import type { LineData } from "$lib/debugger";

    export let line: LineData;
    export let lineHeight: number;
    export let instructionSize: number;
    export let isExecuting: boolean;
    export let isPc: boolean;
    export let isBreakpoint: boolean;
    export let toggleBreakpoint: (address: number) => void;
</script>

<tr style='height: {lineHeight}px'>
    <th 
        on:click={() => toggleBreakpoint(line.address)}
        class="
            {isBreakpoint ? 'breakpoint' : ''}
        "
    >
        {line.address.toString(16)}
    </th>
    <td class="disassembly
            {isExecuting ? 'executing' : ''}
            {isPc ? 'pc' : ''}
    ">{line.content}</td>
    <td>{line.value?.toString(16).padStart(instructionSize * 2, '0')}</td>
</tr>

<style>
    .disassembly {
        min-width: 20em;
    }

    .breakpoint {
        background-color: red;
        color: white;
    }

    .pc {
        background-color: gray;
        color: white;
    }

    .executing {
        background-color: red;
        color: white;
    }
</style>
