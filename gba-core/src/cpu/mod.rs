mod instrs;
mod regs;

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::bus::Bus;
use crate::utils::AddressableBits;

pub use self::instrs::arm::ArmInstruction;
pub use self::instrs::thumb::ThumbInstruction;
use self::regs::Regs;

type ArmLut = [Box<dyn ArmInstruction>; 0x1000];
type ThumbLut = [Box<dyn ThumbInstruction>; 0x1000];

#[derive(PartialEq, Eq)]
pub enum State {
    ARM,
    Thumb,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Mode {
    User,
    System,
    IRQ,
    FIQ,
    Supervisor,
    Undefined,
    Abort,
}

enum CPSR {
    T,
    V,
    C,
    Z,
    N,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct CpuDetails {
    regs: Regs,
    mode: Mode,
    pub executing_pc: Option<u32>,
}

#[wasm_bindgen]
impl CpuDetails {
    pub fn reg(&self, index: u32, mode: JsValue) -> Option<u32> {
        let mode: Mode = serde_wasm_bindgen::from_value(mode).ok()?;
        Some(self.regs.get(index, &mode))
    }

    pub fn cpsr(&self) -> u32 {
        self.regs.cpsr
    }

    pub fn spsr(&self, mode: JsValue) -> Option<u32> {
        let mode: Mode = serde_wasm_bindgen::from_value(mode).ok()?;
        Some(self.regs.spsr(&mode))
    }

    pub fn mode(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.mode).ok().into()
    }

    pub fn pc(&self) -> u32 {
        self.regs.get(15, &Mode::User)
    }
}

pub struct Cpu {
    regs: Regs,

    instr_pipeline: [u32; 2],
    instr_pipeline_size: usize,
    cycle: u128,
    old_interrupt: bool,

    pc_history: VecDeque<u32>,
}

impl Default for Cpu {
    fn default() -> Self {
        let mut cpu = Self {
            regs: Regs::default(),

            instr_pipeline: [0, 0],
            instr_pipeline_size: 0,

            cycle: 0,

            old_interrupt: false,

            pc_history: VecDeque::new(),
        };
        cpu.set_mode(Mode::User);
        cpu
    }
}

impl Cpu {
    pub fn pc_history(&self) -> Vec<u32> {
        self.pc_history.iter().copied().collect()
    }

    fn handle_interrupt(&mut self) {
        self.set_reg_with_mode(14, Mode::IRQ, self.get_executing_instruction_pc() + 4);
        self.regs.spsr_irq = self.regs.cpsr;

        self.set_mode(Mode::IRQ);
        self.set_state(State::ARM);

        // Disable normal interrupts
        self.regs.cpsr.mut_bit(7, true);

        self.set_reg(15, 0x18);
        self.flush_pipeline();
    }

    pub fn get_state(&self) -> State {
        if self.get_cpsr_bit(CPSR::T) == 0 {
            State::ARM
        } else {
            State::Thumb
        }
    }

    fn set_state(&mut self, state: State) {
        match state {
            State::ARM => self.set_flag(CPSR::T, false),
            State::Thumb => self.set_flag(CPSR::T, true),
        }
    }

    fn get_mode(&self) -> Mode {
        // highest bit is always set to 1 - TODO: verify this claim
        match self.regs.cpsr.bits(0, 4) | 0x10 {
            0b10000 => Mode::User,
            0b10001 => Mode::FIQ,
            0b10010 => Mode::IRQ,
            0b10011 => Mode::Supervisor,
            0b10111 => Mode::Abort,
            0b11011 => Mode::Undefined,
            0b11111 => Mode::System,
            _ => panic!(
                "undefined behaviour, mode {:05b} PC:{:x}",
                self.regs.cpsr.bits(0, 4),
                self.regs.pc(),
            ),
        }
    }

    fn set_mode(&mut self, mode: Mode) {
        let val = match mode {
            Mode::User => 0b10000,
            Mode::FIQ => 0b10001,
            Mode::IRQ => 0b10010,
            Mode::Supervisor => 0b10011,
            Mode::Abort => 0b10111,
            Mode::Undefined => 0b11011,
            Mode::System => 0b11111,
        };
        self.regs.cpsr = self.regs.cpsr & !(0x1f) | val;
    }

    fn get_reg(&self, idx: u32) -> u32 {
        if idx == 15 {
            match self.get_state() {
                State::ARM => self.get_reg_internal(15) - 4,
                State::Thumb => self.get_reg_internal(15) - 2,
            }
        } else {
            self.get_reg_internal(idx)
        }
    }

    fn get_reg_internal(&self, idx: u32) -> u32 {
        self.regs.get(idx, &self.get_mode())
    }

    /// Only correct outside of .tick() calls
    pub fn get_executing_instruction_pc(&self) -> u32 {
        match self.get_state() {
            State::ARM => self.get_reg_internal(15) - 8,
            State::Thumb => self.get_reg_internal(15) - 4,
        }
    }

    fn set_reg(&mut self, idx: u32, val: u32) {
        let mode = &self.get_mode();
        *self.regs.get_mut(idx, mode) = val;
    }

    pub fn set_reg_with_mode(&mut self, idx: u32, mode: Mode, val: u32) {
        *self.regs.get_mut(idx, &mode) = val;
    }

    fn get_cpsr_bit(&self, field: CPSR) -> u32 {
        match field {
            CPSR::T => (self.regs.cpsr >> 5) & 1,
            CPSR::V => (self.regs.cpsr >> 28) & 1,
            CPSR::C => (self.regs.cpsr >> 29) & 1,
            CPSR::Z => (self.regs.cpsr >> 30) & 1,
            CPSR::N => (self.regs.cpsr >> 31) & 1,
        }
    }

    fn set_flag(&mut self, flag: CPSR, value: bool) {
        match flag {
            CPSR::T => self.regs.cpsr.mut_bit(5, value),
            CPSR::V => self.regs.cpsr.mut_bit(28, value),
            CPSR::C => self.regs.cpsr.mut_bit(29, value),
            CPSR::Z => self.regs.cpsr.mut_bit(30, value),
            CPSR::N => self.regs.cpsr.mut_bit(31, value),
        }
    }

    fn flush_pipeline(&mut self) {
        self.instr_pipeline_size = 0;
    }

    pub fn skip_bios(&mut self) {
        //self.regs.visible[0] = 0xca5;
        *self.regs.get_mut(13, &Mode::User) = 0x3007f00;
        *self.regs.get_mut(13, &Mode::IRQ) = 0x3007fa0;
        *self.regs.get_mut(13, &Mode::Supervisor) = 0x3007fe0;
        *self.regs.get_mut(15, &Mode::User) = 0x8000000;
        self.regs.cpsr = 0xdf;
        //self.mode = Mode::System;
    }

    pub fn tick(&mut self, bus: &mut Bus, arm_lut: &ArmLut, thumb_lut: &ThumbLut) {
        if self.instr_pipeline_size == 2 {
            self.pc_history
                .push_front(self.get_executing_instruction_pc());
            if self.pc_history.len() > 100 {
                self.pc_history.pop_back();
            }
        }

        let ime_flag = bus.read_byte(0x4000208, self);
        let ie_flag = bus.read_half(0x4000200, self);
        let if_flag = bus.read_half(0x4000202, self);

        let new_interrupt = ime_flag.bit(0) == 1 && ie_flag & if_flag != 0;

        if !self.old_interrupt && new_interrupt {
            self.handle_interrupt();
            console::log_1(&format!("handling interrupt {:b}", ie_flag & if_flag).into());
        }
        self.old_interrupt = new_interrupt;

        let instruction = self.instr_pipeline[0];

        self.instr_pipeline[0] = self.instr_pipeline[1];
        self.instr_pipeline[1] = bus.read(self.regs.pc(), self);

        match self.get_state() {
            State::ARM => *self.regs.pc_mut() += 4,
            State::Thumb => *self.regs.pc_mut() += 2,
        }

        if self.instr_pipeline_size == 2 {
            self.execute(bus, instruction, arm_lut, thumb_lut);
        } else {
            self.instr_pipeline_size += 1;
        }

        self.cycle += 1;
    }

    pub fn in_privileged_mode(&self) -> bool {
        match self.get_mode() {
            Mode::User => false,
            _ => true,
        }
    }

    pub fn mode_has_spsr(&self) -> bool {
        match self.get_mode() {
            Mode::User | Mode::System => false,
            _ => true,
        }
    }

    pub fn inspect(&self) -> CpuDetails {
        CpuDetails {
            regs: self.regs.clone(),
            mode: self.get_mode(),
            executing_pc: if self.instr_pipeline_size == 2 {
                Some(self.get_executing_instruction_pc())
            } else {
                None
            },
        }
    }

    pub fn prefetched_instruction(&self) -> u32 {
        match self.get_state() {
            State::ARM => self.instr_pipeline[0],
            State::Thumb => 0, // TODO: implement this properly
        }
    }
}

pub fn generate_luts() -> (ArmLut, ThumbLut) {
    let arm_lut = (0u32..0x1000)
        .into_iter()
        .map(|val| {
            let low = val.bits(0, 3);
            let high = val.bits(4, 11);
            let instruction = (high << 20) | (low << 4);
            Cpu::decode_arm(instruction)
        })
        .collect::<Vec<Box<dyn ArmInstruction>>>()
        .try_into()
        .unwrap();

    let thumb_lut = (0u16..0x1000)
        .into_iter()
        .map(|val| {
            let instruction = val << 4;
            Cpu::decode_thumb(instruction)
        })
        .collect::<Vec<Box<dyn ThumbInstruction>>>()
        .try_into()
        .unwrap();

    (arm_lut, thumb_lut)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_div(r0: u32, r1: u32) {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        let (arm_lut, thumb_lut) = generate_luts();
        bus.set_bios(include_bytes!("../../og-bios.bin"));

        cpu.skip_bios();
        cpu.set_reg_with_mode(0, Mode::System, r0);
        cpu.set_reg_with_mode(1, Mode::System, r1);
        // Goto DIV
        cpu.set_reg(15, 0x3b4);
        cpu.flush_pipeline();

        while cpu.get_reg(15) - 4 != 0x400 {
            cpu.tick(&mut bus, &arm_lut, &thumb_lut);
        }

        let expected0 = (r0 as i32) / (r1 as i32);
        let expected1 = (r0 as i32) % (r1 as i32);
        let expected3: u32 = expected0.abs().try_into().unwrap();

        let result0 = cpu.get_reg(0) as i32;
        let result1 = cpu.get_reg(1) as i32;
        let result3 = cpu.get_reg(3);

        assert_eq!(result0, expected0, "og div {} by {}", r0, r1);
        assert_eq!(result1, expected1, "og mod {} by {}", r0, r1);
        assert_eq!(result3, expected3, "og abs div {} by {}", r0, r1);
    }

    fn test_div_cultofgba(r0: u32, r1: u32) {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        let (arm_lut, thumb_lut) = generate_luts();
        bus.set_bios(include_bytes!("../../bios.bin"));

        cpu.skip_bios();
        cpu.set_reg_with_mode(0, Mode::System, r0);
        cpu.set_reg_with_mode(1, Mode::System, r1);
        // Goto DIV
        cpu.set_reg(15, 0x734);
        cpu.flush_pipeline();

        while cpu.get_reg(15) - 4 != 0x790 {
            cpu.tick(&mut bus, &arm_lut, &thumb_lut);
        }

        let expected0 = (r0 as i32) / (r1 as i32);
        let expected1 = (r0 as i32) % (r1 as i32);
        let expected3: u32 = expected0.abs().try_into().unwrap();

        let result0 = cpu.get_reg(0) as i32;
        let result1 = cpu.get_reg(1) as i32;
        let result3 = cpu.get_reg(3);

        assert_eq!(result0, expected0, "cult div {} by {}", r0, r1);
        assert_eq!(result1, expected1, "cult mod {} by {}", r0, r1);
        assert_eq!(result3, expected3, "cult abs div {} by {}", r0, r1);
    }

    #[test]
    fn test_div_1_by_1() {
        test_div_cultofgba(1, 1);
        test_div(1, 1);
    }

    #[test]
    fn test_div_2_by_1() {
        test_div_cultofgba(2, 1);
        test_div(2, 1);
    }

    #[test]
    fn test_div_many_by_1() {
        for i in 0..0x100 {
            test_div_cultofgba(i, 1);
            test_div(i, 1);
        }
    }

    #[test]
    fn test_div_by_10() {
        test_div_cultofgba(123, 10);
        test_div(123, 10);
    }

    #[test]
    fn test_div_by_16() {
        test_div_cultofgba(0xa000000, 0x10);
        test_div(0xa000000, 0x10);
    }
}
