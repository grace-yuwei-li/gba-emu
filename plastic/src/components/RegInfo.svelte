<script lang="ts">
	import { gba } from '$lib/gbaStore';
	import Psr from './Psr.svelte';

	enum Mode {
		User = 'User',
		System = 'System',
		Supervisor = 'Supervisor',
		Abort = 'Abort',
		Undefined = 'Undefined',
		IRQ = 'IRQ',
		FIQ = 'FIQ'
	}

	$: cpuDetails = $gba?.cpu;

	$: getReg = (index: number, mode: Mode) => {
		return cpuDetails?.reg(index, { type: mode });
	};

	const showReg = (index: number, mode: Mode) => {
		switch (mode) {
			case Mode.User:
				return true;
			case Mode.System:
				return false;
			case Mode.Supervisor:
			case Mode.Abort:
			case Mode.Undefined:
			case Mode.IRQ:
				return index == 13 || index == 14;
			case Mode.FIQ:
				return 8 <= index && index <= 14;
		}
	};

    $: ie_reg = $gba?.gba.ie_reg();
    $: if_reg = $gba?.gba.if_reg();
    $: ime_reg = $gba?.gba.read_halfword(0x4000208);
</script>

<table>
	<thead>
		<tr>
			<th />
			{#each Object.values(Mode) as mode, i (i)}
				<th class={mode === cpuDetails?.mode().type ? 'strong' : ''}>
					{mode}
				</th>
			{/each}
		</tr>
	</thead>
	<tbody>
		{#each { length: 16 } as _, index (index)}
			<tr>
				<th>
					{#if index === 15}
						PC
					{:else if index === 14}
						LR
                    {:else if index === 13}
                        SP
					{:else}
						r{index}
					{/if}
				</th>
				{#each Object.values(Mode) as mode, i (i)}
					<td>
						{#if showReg(index, mode)}
							{getReg(index, mode)?.toString(16)}
						{:else}
							-
						{/if}
					</td>
				{/each}
			</tr>
		{/each}
		<tr>
			<th>CPSR</th>
			{#each Object.values(Mode) as mode, i (i)}
				<td>
					{#if mode === Mode.User && cpuDetails !== undefined }
                        {cpuDetails.cpsr().toString(16)}
					{:else}
						-
					{/if}
				</td>
			{/each}
		</tr>
		<tr>
			<th>SPSR</th>
			{#each Object.values(Mode) as mode, i (i)}
				<td>
                    {#if mode !== Mode.System && mode !== Mode.User && cpuDetails !== undefined}
                        {(cpuDetails.spsr({type: mode}) ?? 0).toString(16)}
                    {:else}
                        -
                    {/if}
				</td>
			{/each}
		</tr>
	</tbody>
</table>
<table>
    <tr>
        <th>IE</th>
        <td>{ie_reg}</td>
    </tr>
    <tr>
        <th>IF</th>
        <td>{if_reg}</td>
    </tr>
    <tr>
        <th>IME</th>
        <td>{ime_reg}</td>
    </tr>
</table>

<style>
	.strong {
		background-color: white;
		color: black;
	}

    td {
        padding: 0 1em;
        text-align: end;
    }
</style>
