mod bus;
mod cpu;
mod ppu;
mod utils;

use std::collections::HashSet;

pub use utils::js::*;
pub use utils::logging;

use ppu::PpuDetails;
use wasm_bindgen::prelude::*;

use bus::Bus;
use cpu::Cpu;

use crate::cpu::CpuDetails;

#[wasm_bindgen]
pub struct GbaCore {
    cpu: Cpu,
    bus: Bus,

    pub stopped: bool,
    debugger_enabled: bool,
    breakpoints: HashSet<u32>,
}

impl Default for GbaCore {
    fn default() -> Self {
        Self {
            cpu: Cpu::default(),
            bus: Bus::default(),

            stopped: false,
            debugger_enabled: true,
            breakpoints: HashSet::new(),
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
        if self.debugger_enabled && self.breakpoints.contains(&self.cpu.get_executing_instruction_pc()) {
            self.stopped = true;
        }

        if !self.stopped {
            self.cpu.tick(&mut self.bus)
        }
    }

    pub fn load_test_rom(&mut self) {
        let bytes = include_bytes!("../tests/roms/panda.gba");
        self.load_rom(bytes);
    }

    pub fn load_rom(&mut self, bytes: &[u8]) {
        self.bus.load_rom(bytes)
    }

    pub fn skip_bios(&mut self) {
        self.cpu.skip_bios(&self.bus);
    }

    pub fn reset(self) -> Self {
        Self {
            stopped: self.stopped,
            breakpoints: self.breakpoints,
            ..Self::default()
        }
    }

    pub fn breakpoints(&self) -> Vec<u32> {
        self.breakpoints.iter().copied().collect()
    }

    pub fn add_breakpoint(&mut self, breakpoint: u32) {
        self.breakpoints.insert(breakpoint);
    }

    pub fn remove_breakpoint(&mut self, breakpoint: u32) {
        self.breakpoints.remove(&breakpoint);
    }

    pub fn read_address(&self, address: u32) -> u32 {
        self.bus.read(address)
    }
}

impl GbaCore {
    pub fn regs(&mut self) -> Vec<u32> {
        self.cpu.get_all_regs()
    }
}
