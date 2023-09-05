import { writable } from 'svelte/store';

const BUFFER_CAPACITY = 60;

interface FrameTimes {
	buffer: number[];
}

export const frameTimes = writable<FrameTimes>({
	buffer: []
});

export const addFrameTime = (frameTime: number) => {
	frameTimes.update(({ buffer }) => {
		if (buffer.length < BUFFER_CAPACITY) {
			return {
				buffer: [...buffer, frameTime]
			};
		} else {
			const copy = buffer.slice(1);
			copy.push(frameTime);
			return {
				buffer: copy
			};
		}
	});
};
