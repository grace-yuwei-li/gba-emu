mod wasm;
mod debug;

use std::collections::HashSet;

use crate::Ppu;
use crate::bus::{self, Bus};
use crate::cpu::generate_luts;
use crate::cpu::State;
use crate::cpu::{ArmInstruction, Cpu, ThumbInstruction};

use wasm_bindgen::prelude::*;

pub use crate::utils::js::*;

#[wasm_bindgen]
pub struct GbaCore {
    cpu: Cpu,
    pub(crate) bus: Bus,

    arm_lut: [Box<dyn ArmInstruction>; 0x1000],
    thumb_lut: [Box<dyn ThumbInstruction>; 0x1000],

    pub stopped: bool,
    debugger_enabled: bool,
    arm_breakpoints: HashSet<u32>,
    thumb_breakpoints: HashSet<u32>,
}

impl Default for GbaCore {
    fn default() -> Self {
        let (arm_lut, thumb_lut) = generate_luts();

        Self {
            cpu: Cpu::default(),
            bus: Bus::default(),

            arm_lut,
            thumb_lut,

            stopped: false,
            debugger_enabled: true,
            arm_breakpoints: HashSet::new(),
            thumb_breakpoints: HashSet::new(),
        }
    }
}

#[wasm_bindgen]
impl GbaCore {
    pub fn read_halfword(&self, address: u32) -> u32 {
        self.bus.read_half(address, &self.cpu)
    }

    pub fn thumb_state(&self) -> bool {
        self.cpu.get_state() == State::Thumb
    }

    pub fn tick(&mut self) {
        if self.debugger_enabled && self.should_break(&self.cpu.get_executing_instruction_pc()) {
            self.stopped = true;
        }

        if !self.stopped {
            self.cpu.tick(&mut self.bus, &self.arm_lut, &self.thumb_lut);
            self.bus.ppu.tick(&mut self.bus.io_map);
        }
    }

    pub fn tick_multiple(&mut self, num_ticks: u32) {
        for _ in 0..num_ticks {
            self.tick();
        }
    }

    fn should_break(&self, address: &u32) -> bool {
        match self.cpu.get_state() {
            State::ARM => self.arm_breakpoints.contains(address),
            State::Thumb => self.thumb_breakpoints.contains(address),
        }
    }

    pub fn load_test_rom(&mut self) {
        let bytes = include_bytes!("../tests/roms/armwrestler-gba-fixed.gba");
        //let bytes = include_bytes!("../tests/roms/panda.gba");
        self.load_rom(bytes);
    }

    pub fn load_rom(&mut self, bytes: &[u8]) {
        self.bus.load_rom(bytes);
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

    pub fn set_key(&mut self, key: bus::Key, pressed: bool) {
        self.bus.set_key(key, pressed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_to_if_clears_bit() {
        let mut gba = GbaCore::new();
        gba.bus.io_map.set_interrupt(bus::Interrupt::VBlank, true);

        assert_eq!(gba.bus.read_half(0x4000202, &gba.cpu), 1);

        gba.bus.write_half(0x4000202, 1);

        assert_eq!(gba.bus.read_half(0x4000202, &gba.cpu), 0);
    }
}
