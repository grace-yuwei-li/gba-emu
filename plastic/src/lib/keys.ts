import { Gba, Key } from '$lib/pkg/debug/gba_web';

export const handleKey = (gba: Gba | undefined, event: KeyboardEvent, pressed: boolean) => {
	const key = getKey(event);
	if (key && gba) {
		gba.set_key(key, pressed);
	}
};

const getKey = (event: KeyboardEvent): Key | undefined => {
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
};
