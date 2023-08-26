import { writable } from 'svelte/store';
import initWasm, { GbaCore } from '$lib/pkg/debug/gba_core';

export const gba = writable<GbaCore | undefined>(undefined);

export const reset = () => {
    gba.update((old) => {
        if (old) {
            old.free();
        }

        const emu = new GbaCore();
        emu.load_test_rom();
        emu.skip_bios();

        return emu;
    });
}

export const init = async () => {
    await initWasm();

    reset();
}
