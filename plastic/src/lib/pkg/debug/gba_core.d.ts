/* tslint:disable */
/* eslint-disable */
/**
* @param {number} instruction
* @returns {string}
*/
export function disassemble_arm(instruction: number): string;
/**
* @param {number} instruction
* @returns {string}
*/
export function disassemble_thumb(instruction: number): string;
/**
*/
export enum Key {
  A = 0,
  B = 1,
  Select = 2,
  Start = 3,
  Right = 4,
  Left = 5,
  Up = 6,
  Down = 7,
  R = 8,
  L = 9,
}
/**
*/
export class CpuDetails {
  free(): void;
/**
* @param {number} index
* @param {any} mode
* @returns {number | undefined}
*/
  reg(index: number, mode: any): number | undefined;
/**
* @returns {number}
*/
  cpsr(): number;
/**
* @param {any} mode
* @returns {number | undefined}
*/
  spsr(mode: any): number | undefined;
/**
* @returns {any}
*/
  mode(): any;
/**
* @returns {number}
*/
  pc(): number;
/**
*/
  executing_pc?: number;
}
/**
*/
export class GbaCore {
  free(): void;
/**
*/
  constructor();
/**
* @returns {CpuDetails}
*/
  inspect_cpu(): CpuDetails;
/**
* @returns {PpuDetails}
*/
  inspect_ppu(): PpuDetails;
/**
* @returns {MemoryDetails}
*/
  inspect_memory(): MemoryDetails;
/**
* @returns {boolean}
*/
  thumb_state(): boolean;
/**
*/
  tick(): void;
/**
* @param {number} num_ticks
*/
  tick_multiple(num_ticks: number): void;
/**
*/
  load_test_rom(): void;
/**
* @param {Uint8Array} bytes
*/
  load_rom(bytes: Uint8Array): void;
/**
*/
  skip_bios(): void;
/**
* @returns {GbaCore}
*/
  reset(): GbaCore;
/**
* @param {boolean} enabled
*/
  enable_debugger(enabled: boolean): void;
/**
* @param {boolean} value
*/
  set_stopped(value: boolean): void;
/**
* @returns {Uint32Array}
*/
  arm_breakpoints(): Uint32Array;
/**
* @returns {Uint32Array}
*/
  thumb_breakpoints(): Uint32Array;
/**
* @param {number} breakpoint
*/
  add_arm_breakpoint(breakpoint: number): void;
/**
* @param {number} breakpoint
*/
  add_thumb_breakpoint(breakpoint: number): void;
/**
* @param {number} breakpoint
*/
  remove_arm_breakpoint(breakpoint: number): void;
/**
* @param {number} breakpoint
*/
  remove_thumb_breakpoint(breakpoint: number): void;
/**
* @param {number} address
* @returns {number}
*/
  read_address(address: number): number;
/**
* @param {number} key
* @param {boolean} pressed
*/
  set_key(key: number, pressed: boolean): void;
/**
*/
  stopped: boolean;
}
/**
*/
export class MemoryDetails {
  free(): void;
/**
*/
  readonly vram: Uint8Array;
}
/**
*/
export class PpuDetails {
  free(): void;
/**
* @returns {Uint8ClampedArray}
*/
  screen(): Uint8ClampedArray;
/**
*/
  bg_mode: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly disassemble_arm: (a: number, b: number) => void;
  readonly disassemble_thumb: (a: number, b: number) => void;
  readonly __wbg_ppudetails_free: (a: number) => void;
  readonly __wbg_get_ppudetails_bg_mode: (a: number) => number;
  readonly __wbg_set_ppudetails_bg_mode: (a: number, b: number) => void;
  readonly ppudetails_screen: (a: number) => number;
  readonly __wbg_gbacore_free: (a: number) => void;
  readonly __wbg_get_gbacore_stopped: (a: number) => number;
  readonly __wbg_set_gbacore_stopped: (a: number, b: number) => void;
  readonly gbacore_new: () => number;
  readonly gbacore_inspect_cpu: (a: number) => number;
  readonly gbacore_inspect_ppu: (a: number) => number;
  readonly gbacore_inspect_memory: (a: number) => number;
  readonly gbacore_thumb_state: (a: number) => number;
  readonly gbacore_tick: (a: number) => void;
  readonly gbacore_tick_multiple: (a: number, b: number) => void;
  readonly gbacore_load_test_rom: (a: number) => void;
  readonly gbacore_load_rom: (a: number, b: number, c: number) => void;
  readonly gbacore_skip_bios: (a: number) => void;
  readonly gbacore_reset: (a: number) => number;
  readonly gbacore_enable_debugger: (a: number, b: number) => void;
  readonly gbacore_arm_breakpoints: (a: number, b: number) => void;
  readonly gbacore_thumb_breakpoints: (a: number, b: number) => void;
  readonly gbacore_add_arm_breakpoint: (a: number, b: number) => void;
  readonly gbacore_add_thumb_breakpoint: (a: number, b: number) => void;
  readonly gbacore_remove_arm_breakpoint: (a: number, b: number) => void;
  readonly gbacore_remove_thumb_breakpoint: (a: number, b: number) => void;
  readonly gbacore_read_address: (a: number, b: number) => number;
  readonly gbacore_set_key: (a: number, b: number, c: number) => void;
  readonly gbacore_set_stopped: (a: number, b: number) => void;
  readonly __wbg_memorydetails_free: (a: number) => void;
  readonly memorydetails_vram: (a: number) => number;
  readonly __wbg_cpudetails_free: (a: number) => void;
  readonly __wbg_get_cpudetails_executing_pc: (a: number, b: number) => void;
  readonly __wbg_set_cpudetails_executing_pc: (a: number, b: number, c: number) => void;
  readonly cpudetails_reg: (a: number, b: number, c: number, d: number) => void;
  readonly cpudetails_cpsr: (a: number) => number;
  readonly cpudetails_spsr: (a: number, b: number, c: number) => void;
  readonly cpudetails_mode: (a: number) => number;
  readonly cpudetails_pc: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
