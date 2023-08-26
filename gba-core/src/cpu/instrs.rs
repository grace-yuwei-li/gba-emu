use wasm_bindgen::prelude::wasm_bindgen;

use crate::bus::Bus;

use super::{Cpu, State};

pub mod arm;
pub mod thumb;

impl Cpu {
    pub fn execute(&mut self, bus: &mut Bus, instruction: u32) {
        match self.get_state() {
            State::ARM => {
                if !self.check_cond(instruction) {
                    log::trace!("Cond check failed for instruction {:#034b}", instruction);
                    return;
                }

                log::trace!("Executing ARM instruction {:08x}", instruction);
                let instr_type = self.decode_arm(instruction);
                instr_type.execute(self, bus, instruction);
            }
            State::Thumb => {
                log::trace!("Executing THUMB instruction {:04x}", instruction as u16);
                let fp = self.decode_thumb(instruction as u16);
                fp(self, bus, instruction as u16)
            }
        }
    }
}

#[wasm_bindgen]
pub fn disassemble_arm(instruction: u32) -> String {
    arm::MetaInstr::decode_arm(instruction).disassembly(instruction)
}
