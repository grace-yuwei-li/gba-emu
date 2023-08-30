mod regs;
mod instrs;

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

use crate::bus::Bus;
use crate::utils::AddressableBits;

use self::regs::Regs;

enum State {
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
    M,
    T,
    F,
    I,
    V,
    C,
    Z,
    N,
}

#[wasm_bindgen]
pub struct CpuDetails {
    regs: Regs,
    mode: Mode,
    pub executing_pc: Option<u32>,
}

#[wasm_bindgen]
impl CpuDetails {
    pub fn reg(&self, index: usize, mode: JsValue) -> Option<u32> {
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
}

impl Default for Cpu {
    fn default() -> Self {
        let mut cpu = Self {
            regs: Regs::default(),

            instr_pipeline: [0, 0],
            instr_pipeline_size: 0,

            cycle: 0,
        };
        cpu.set_mode(Mode::User);
        cpu
    }
}

impl Cpu {
    fn get_state(&self) -> State {
        if self.get_cpsr_bits(CPSR::T) == 0 {
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
        match self.regs.cpsr.bits(0, 4) {
            0b10000 => Mode::User,
            0b10001 => Mode::FIQ,
            0b10010 => Mode::IRQ,
            0b10011 => Mode::Supervisor,
            0b10111 => Mode::Abort,
            0b11011 => Mode::Undefined,
            0b11111 => Mode::System,
            _ => panic!("undefined behaviour, mode {:05b}", self.regs.cpsr.bits(0, 4)),
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
        self.regs.cpsr = val;
    }

    fn get_reg(&self, idx: usize) -> u32 {
        if idx == 15 {
            match self.get_state() {
                State::ARM => self.get_reg_internal(15) - 4,
                State::Thumb => self.get_reg_internal(15) - 2,
            }
        } else {
            self.get_reg_internal(idx)
        }
    }

    fn get_reg_internal(&self, idx: usize) -> u32 {
        self.regs.get(idx, &self.get_mode())
    }

    /// Should be ran before calling .tick()
    pub fn get_executing_instruction_pc(&self) -> u32 {
        self.get_reg_internal(15) - 8
    }

    fn set_reg(&mut self, idx: usize, val: u32) {
        let mode = &self.get_mode();
        *self.regs.get_mut(idx, mode) = val;
    }

    pub fn set_reg_with_mode(&mut self, idx: usize, mode: Mode, val: u32) {
        *self.regs.get_mut(idx, &mode) = val;
    }

    fn get_cpsr_bits(&self, field: CPSR) -> u32 {
        match field {
            CPSR::M => self.regs.cpsr & 0b11111,
            CPSR::T => (self.regs.cpsr >> 5) & 1,
            CPSR::F => (self.regs.cpsr >> 6) & 1,
            CPSR::I => (self.regs.cpsr >> 7) & 1,
            CPSR::V => (self.regs.cpsr >> 28) & 1,
            CPSR::C => (self.regs.cpsr >> 29) & 1,
            CPSR::Z => (self.regs.cpsr >> 30) & 1,
            CPSR::N => (self.regs.cpsr >> 31) & 1,
        }
    }

    fn set_flag(&mut self, flag: CPSR, value: bool) {
        match flag {
            CPSR::M => unimplemented!(),
            CPSR::T => self.regs.cpsr.mut_bit(5, value),
            CPSR::F => self.regs.cpsr.mut_bit(6, value),
            CPSR::I => self.regs.cpsr.mut_bit(7, value),
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

    pub fn tick(&mut self, bus: &mut Bus) {
        let instruction = self.instr_pipeline[0];

        self.instr_pipeline[0] = self.instr_pipeline[1];
        log::trace!(
            "Cycle {} PC {:x} read value {:x}",
            self.cycle,
            self.regs.pc(),
            bus.read(self.regs.pc(), self)
        );
        self.instr_pipeline[1] = bus.read(self.regs.pc(), self);

        match self.get_state() {
            State::ARM => *self.regs.pc_mut() += 4,
            State::Thumb => *self.regs.pc_mut() += 2,
        }

        if self.instr_pipeline_size == 2 {
            self.execute(bus, instruction);
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
