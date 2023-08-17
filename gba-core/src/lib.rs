mod bus;
mod cpu;
mod ppu;
mod utils;

pub use utils::logging;
use wasm_bindgen::prelude::*;

use bus::Bus;
use cpu::Cpu;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

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
        Self::default()
    }

    pub fn greet(&self, name: &str) {
        alert(&format!("Hello, {}!", name));
    }
}

impl GbaCore {
    pub fn load_rom(&mut self, bytes: &[u8]) {
        self.bus.load_rom(bytes)
    }

    pub fn skip_bios(&mut self) {
        self.cpu.skip_bios(&self.bus);
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.bus)
    }

    pub fn regs(&mut self) -> Vec<u32> {
        self.cpu.get_all_regs()
    }
}
