mod bus;
mod cpu;
mod ppu;
mod utils;

pub use utils::logging;
pub use utils::js::*;

use ppu::PpuDetails;
use wasm_bindgen::prelude::*;

use bus::Bus;
use cpu::Cpu;

use crate::cpu::CpuDetails;

#[wasm_bindgen]
pub struct GbaCore {
    cpu: Cpu,
    bus: Bus,
}

impl Default for GbaCore {
    fn default() -> Self {
        Self {
            cpu: Cpu::default(),
            bus: Bus::default(),
        }
    }
}

#[wasm_bindgen]
impl GbaCore {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self::default()
    }

    pub fn inspect_cpu(&self) -> CpuDetails {
        self.cpu.inspect()
    }

    pub fn inspect_ppu(&self) -> PpuDetails {
        self.bus.ppu.inspect()
    }

    pub fn inspect_memory(&self) -> bus::MemoryDetails {
        self.bus.inspect()
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.bus)
    }

    pub fn load_panda(&mut self) {
        let bytes = include_bytes!("../tests/roms/panda.gba");
        self.load_rom(bytes);
    }

    pub fn load_rom(&mut self, bytes: &[u8]) {
        self.bus.load_rom(bytes)
    }

    pub fn skip_bios(&mut self) {
        self.cpu.skip_bios(&self.bus);
    }
}

impl GbaCore {
    pub fn regs(&mut self) -> Vec<u32> {
        self.cpu.get_all_regs()
    }
}
