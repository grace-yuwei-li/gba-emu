import { writable } from 'svelte/store';
import { gbaStore } from './gbaStore';
import type { Gba } from './pkg/debug/gba_web';

interface DebuggerData {
	screen_array: Uint8ClampedArray;
	bg_palette_array: Uint8ClampedArray;
	bg_tile_array: Uint8ClampedArray;
}

const initialData: DebuggerData = {
	screen_array: new Uint8ClampedArray(240 * 160 * 4),
	bg_palette_array: new Uint8ClampedArray(16 * 16 * 4),
	bg_tile_array: new Uint8ClampedArray(1024 * 8 * 8 * 4)
};

export const debuggerStore = writable<DebuggerData>(initialData);

gbaStore.subscribe((gba) => {
	if (gba) {
		console.log('Setting screen array');
		gba.set_screen_array(initialData.screen_array);
	}
});

export function updateDebuggerData(gba: Gba) {
	gba.process_responses();
	// Assign to store

	// Request updates
	gba.request_screen_draw();
	gba.request_cpu_debug_info();
}
