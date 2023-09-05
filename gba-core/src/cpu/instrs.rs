use wasm_bindgen::prelude::wasm_bindgen;

use crate::{bus::Bus, utils::AddressableBits};

use super::{Cpu, State};

pub mod arm;
pub mod thumb;

impl Cpu {
    pub fn execute(&mut self, bus: &mut Bus, instruction: u32) {
        match self.get_state() {
            State::ARM => {
                if !self.check_cond(instruction.bits(28, 31)) {
                    return;
                }

                let instr_type = Self::decode_arm(instruction);
                instr_type.execute(self, bus, instruction);
            }
            State::Thumb => {
                let thumb_instruction = Self::decode_thumb(instruction as u16);
                thumb_instruction.execute(self, bus, instruction as u16);
            }
        }
    }
}

#[wasm_bindgen]
pub fn disassemble_arm(instruction: u32) -> String {
    Cpu::decode_arm(instruction).disassembly(instruction)
}

#[wasm_bindgen]
pub fn disassemble_thumb(instruction: u16) -> String {
    Cpu::decode_thumb(instruction).disassembly(instruction)
}
