import { get, writable } from 'svelte/store';
import initWasm, { CpuDetails, GbaCore, PpuDetails } from '$lib/pkg/debug/gba_core';

interface GbaDetails {
	gba: GbaCore;
	cpu: CpuDetails;
	ppu: PpuDetails;
}

export const gba = writable<GbaDetails | undefined>(undefined);
export const rom = writable<Uint8Array | undefined>(undefined);

export const reset = () => {
	gba.update((old) => {
		if (old) {
			old.gba.free();
			old.cpu.free();
		}

		const emu = new GbaCore();

		let rom_data = get(rom);
		if (!rom_data) {
			emu.load_test_rom();
		} else {
			emu.load_rom(rom_data);
		}
		emu.skip_bios();

		return {
			gba: emu,
			cpu: emu.inspect_cpu(),
			ppu: emu.inspect_ppu()
		};
	});
};

export const tick = (numTicks: number) => {
	gba.update((details) => {
		if (!details) {
			return details;
		}

		for (let i = 0; i < numTicks; i++) {
			details.gba.tick();
		}

		details.cpu.free();
		details.ppu.free();

		return {
			gba: details.gba,
			cpu: details.gba.inspect_cpu(),
			ppu: details.gba.inspect_ppu()
		};
	});
};

export const init = async () => {
	await initWasm();

	reset();
};
