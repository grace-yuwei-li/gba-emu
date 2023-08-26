import { writable } from 'svelte/store';
import type { GbaCore } from '$lib/pkg/debug/gba_core';

export const gba = writable<GbaCore | undefined>(undefined);
