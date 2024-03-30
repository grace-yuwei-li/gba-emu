import { gbaStore } from './gbaStore';
import { Key } from './pkg/debug/gba_web';

export const handleKeyDown = (event: KeyboardEvent) => {
	/*
	const key = getKey(event);
	if (key === undefined) return;

	gba.update((details) => {
		if (!details) return details;

		details.gba.set_key(key, true);

		return details;
	});
    */
};

export const handleKeyUp = (event: KeyboardEvent) => {
	/*
	const key = getKey(event);
	if (key === undefined) return;

	gba.update((details) => {
		if (!details) return details;

		details.gba.set_key(key, false);

		return details;
	});
        */
};

const getKey = (event: KeyboardEvent): Key | undefined => {
	/*
	const keyMap: Record<string, Key> = {
		x: Key.A,
		z: Key.B,
		Backspace: Key.Select,
		Enter: Key.Start,
		ArrowUp: Key.Up,
		ArrowDown: Key.Down,
		ArrowLeft: Key.Left,
		ArrowRight: Key.Right,
		a: Key.L,
		s: Key.R
	};
	return keyMap[event.key];
        */
};
