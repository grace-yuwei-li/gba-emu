use wasm_bindgen::prelude::*;

use crate::{GbaCore, cpu::CpuDetails, ppu::PpuDetails, bus::MemoryDetails};

#[wasm_bindgen]
impl GbaCore {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self::default()
    }

    pub fn pc_history(&self) -> Vec<u32> {
        self.cpu.pc_history()
    }

    pub fn inspect_cpu(&self) -> CpuDetails {
        self.cpu.inspect()
    }

    pub fn inspect_ppu(&self) -> PpuDetails {
        self.bus.ppu.inspect()
    }

    pub fn inspect_memory(&self) -> MemoryDetails {
        self.bus.inspect()
    }

    pub fn tilemap(&self, bg: usize) -> js_sys::Uint8ClampedArray {
        self.bus.ppu.tilemap(bg)
    }

    pub fn ie_reg(&self) -> u32 {
        self.bus.read_half(0x4000200, &self.cpu)
    }

    pub fn if_reg(&self) -> u32 {
        self.bus.read_half(0x4000202, &self.cpu)
    }
}
