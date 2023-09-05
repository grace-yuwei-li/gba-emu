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
use crate::cpu::State;

#[wasm_bindgen]
pub struct GbaCore {
    cpu: Cpu,
    bus: Bus,

    pub stopped: bool,
    debugger_enabled: bool,
    arm_breakpoints: HashSet<u32>,
    thumb_breakpoints: HashSet<u32>,
}

impl Default for GbaCore {
    fn default() -> Self {
        Self {
            cpu: Cpu::default(),
            bus: Bus::default(),

            stopped: false,
            debugger_enabled: true,
            arm_breakpoints: HashSet::new(),
            thumb_breakpoints: HashSet::new(),
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

    pub fn thumb_state(&self) -> bool {
        self.cpu.get_state() == State::Thumb
    }

    pub fn tick(&mut self) {
        if self.debugger_enabled && self.should_break(&self.cpu.get_executing_instruction_pc()) {
            self.stopped = true;
        }

        if !self.stopped {
            self.cpu.tick(&mut self.bus);
            self.bus.ppu.tick();
        }
    }

    fn should_break(&self, address: &u32) -> bool {
        match self.cpu.get_state() {
            State::ARM => self.arm_breakpoints.contains(address),
            State::Thumb => self.thumb_breakpoints.contains(address),
        }
    }

    pub fn load_test_rom(&mut self) {
        let bytes = include_bytes!("../tests/roms/thumb.gba");
        //let bytes = include_bytes!("../tests/roms/panda.gba");
        self.load_rom(bytes);
    }

    pub fn load_rom(&mut self, bytes: &[u8]) {
        self.bus.load_rom(bytes)
    }

    pub fn skip_bios(&mut self) {
        self.cpu.skip_bios();
    }

    pub fn reset(self) -> Self {
        Self {
            stopped: self.stopped,
            arm_breakpoints: self.arm_breakpoints,
            thumb_breakpoints: self.thumb_breakpoints,
            ..Self::default()
        }
    }

    pub fn enable_debugger(&mut self, enabled: bool) {
        self.debugger_enabled = enabled;
    }

    pub fn set_stopped(&mut self, value: bool) {
        self.stopped = value;
    }

    pub fn arm_breakpoints(&self) -> Vec<u32> {
        self.arm_breakpoints.iter().copied().collect()
    }

    pub fn thumb_breakpoints(&self) -> Vec<u32> {
        self.thumb_breakpoints.iter().copied().collect()
    }

    pub fn add_arm_breakpoint(&mut self, breakpoint: u32) {
        self.arm_breakpoints.insert(breakpoint);
    }

    pub fn add_thumb_breakpoint(&mut self, breakpoint: u32) {
        self.thumb_breakpoints.insert(breakpoint);
    }

    pub fn remove_arm_breakpoint(&mut self, breakpoint: u32) {
        self.arm_breakpoints.remove(&breakpoint);
    }

    pub fn remove_thumb_breakpoint(&mut self, breakpoint: u32) {
        self.thumb_breakpoints.remove(&breakpoint);
    }

    pub fn read_address(&self, address: u32) -> u32 {
        self.bus.read(address, &self.cpu)
    }
}
