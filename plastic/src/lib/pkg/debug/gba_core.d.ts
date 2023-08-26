/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} input
* @returns {Uint8ClampedArray}
*/
export function to_canvas_binary_data(input: Uint8Array): Uint8ClampedArray;
/**
*/
export class CpuDetails {
  free(): void;
/**
*/
  pc: number;
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
*/
  tick(): void;
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
* @returns {Uint32Array}
*/
  breakpoints(): Uint32Array;
/**
* @param {number} breakpoint
*/
  add_breakpoint(breakpoint: number): void;
/**
* @param {number} breakpoint
*/
  remove_breakpoint(breakpoint: number): void;
/**
* @param {number} address
* @returns {number}
*/
  read_address(address: number): number;
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
  readonly __wbg_cpudetails_free: (a: number) => void;
  readonly __wbg_get_cpudetails_pc: (a: number) => number;
  readonly __wbg_set_cpudetails_pc: (a: number, b: number) => void;
  readonly __wbg_gbacore_free: (a: number) => void;
  readonly __wbg_get_gbacore_stopped: (a: number) => number;
  readonly __wbg_set_gbacore_stopped: (a: number, b: number) => void;
  readonly gbacore_new: () => number;
  readonly gbacore_inspect_cpu: (a: number) => number;
  readonly gbacore_inspect_ppu: (a: number) => number;
  readonly gbacore_inspect_memory: (a: number) => number;
  readonly gbacore_tick: (a: number) => void;
  readonly gbacore_load_test_rom: (a: number) => void;
  readonly gbacore_load_rom: (a: number, b: number, c: number) => void;
  readonly gbacore_skip_bios: (a: number) => void;
  readonly gbacore_reset: (a: number) => number;
  readonly gbacore_breakpoints: (a: number, b: number) => void;
  readonly gbacore_add_breakpoint: (a: number, b: number) => void;
  readonly gbacore_remove_breakpoint: (a: number, b: number) => void;
  readonly gbacore_read_address: (a: number, b: number) => number;
  readonly to_canvas_binary_data: (a: number, b: number) => number;
  readonly __wbg_memorydetails_free: (a: number) => void;
  readonly memorydetails_vram: (a: number) => number;
  readonly __wbg_ppudetails_free: (a: number) => void;
  readonly __wbg_get_ppudetails_bg_mode: (a: number) => number;
  readonly __wbg_set_ppudetails_bg_mode: (a: number, b: number) => void;
  readonly ppudetails_screen: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
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
