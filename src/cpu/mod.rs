mod instrs;

use crate::bus::Bus;


enum State {
    ARM,
    Thumb,
}

enum Mode {
    User,
    System,
    IRQ,
    FIQ,
    Undef,
    Abt,
}

#[derive(Debug, Default)]
struct Regs {
    visible: [u32; 16],
    banked: [u32; 20],
}

enum CPSR {
    M,
    T,
    F,
    I,
    V,
    C,
    Z,
    N
}

pub struct Cpu {
    state: State,
    mode: Mode,
    regs: Regs,
    cpsr: u32,

    instr_pipeline: [u32; 2],
    instr_pipeline_size: usize,
    cycle: u128,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            state: State::ARM,
            mode: Mode::User,
            regs: Regs::default(),
            cpsr: 0,

            instr_pipeline: [0, 0],
            instr_pipeline_size: 0,

            cycle: 0,
        }
    }
}

impl Cpu {
    fn get_reg(&self, idx: usize) -> u32 {
        if idx == 15 {
            self.get_reg_internal(15) - 4
        } else {
            self.get_reg_internal(idx)
        }
    }

    fn get_reg_internal(&self, idx: usize) -> u32 {
        self.regs.visible[idx]
    }

    fn set_reg(&mut self, idx: usize, val: u32) {
        self.regs.visible[idx] = val;
    }

    pub fn get_all_regs(&self) -> Vec<u32> {
        let mut out = self.regs.visible.to_vec();
        out.push(self.cpsr);
        // SPSR is same as CPSR
        out.push(self.cpsr);
        out
    }

    fn get_cpsr_bits(&self, field: CPSR) -> u32 {
        match field {
            CPSR::M => self.cpsr & 0b11111,
            CPSR::T => (self.cpsr >> 5) & 1,
            CPSR::F => (self.cpsr >> 6) & 1,
            CPSR::I => (self.cpsr >> 7) & 1,
            CPSR::V => (self.cpsr >> 28) & 1,
            CPSR::C => (self.cpsr >> 29) & 1,
            CPSR::Z => (self.cpsr >> 30) & 1,
            CPSR::N => (self.cpsr >> 31) & 1,
        }
    }

    fn flush_pipeline(&mut self) {
        self.instr_pipeline_size = 0;
    }

    pub fn skip_bios(&mut self, bus: &Bus) {
        //self.regs.visible[0] = 0xca5;
        self.regs.visible[13] = 0x3007f00;
        self.regs.visible[15] = 0x8000000;
        self.cpsr = 0xdf;
        self.mode = Mode::System;

        // sp_usr/sys = 0x3007f00
        // sp_irq = 0x3007fa0
        // sp_supervisor = 0x3007fe0
    }

    pub fn tick(&mut self, bus: &mut Bus) {
        let instruction = self.instr_pipeline[0];

        self.instr_pipeline[0] = self.instr_pipeline[1];
        log::trace!("Cycle {} PC {:x} read value {:x}", self.cycle, self.regs.visible[15], bus.get(self.regs.visible[15]));
        self.instr_pipeline[1] = bus.get(self.regs.visible[15]);
        self.regs.visible[15] += 4;

        if self.instr_pipeline_size == 2 {
            log::trace!("Cycle {} PC {:x} execute instruction {:x}", self.cycle, self.regs.visible[15], instruction);
            self.execute(bus, instruction);
        } else {
            self.instr_pipeline_size += 1;
        }

        self.cycle += 1;
    }
}
