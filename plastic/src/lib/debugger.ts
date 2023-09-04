export interface LineData {
	address: number;
	value: number | undefined;
	content: string | undefined;
}

export enum InstructionMode {
	Arm = 'arm',
	Thumb = 'thumb',
	Auto = 'auto'
}
