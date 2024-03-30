use wasm_bindgen::prelude::wasm_bindgen;

use crate::{bus::Bus, utils::AddressableBits};

use super::{ArmLut, Cpu, State, ThumbLut};

pub mod arm;
pub mod thumb;

impl Cpu {
    pub fn execute(
        &mut self,
        bus: &mut Bus,
        instruction: u32,
        arm_lut: &ArmLut,
        thumb_lut: &ThumbLut,
    ) {
        match self.get_state() {
            State::ARM => {
                if !self.check_cond(instruction.bits(28, 31)) {
                    return;
                }
                let index = instruction.bits(20, 27) << 4 | instruction.bits(4, 7);
                let arm_instruction = &arm_lut[usize::try_from(index).unwrap()];
                arm_instruction.execute(self, bus, instruction);
            }
            State::Thumb => {
                let index = instruction.bits(4, 15);
                let thumb_instruction = &thumb_lut[usize::try_from(index).unwrap()];
                thumb_instruction.execute(self, bus, instruction as u16);
            }
        }
    }
}

#[cfg_attr(feature="debugger", wasm_bindgen)]
pub fn disassemble_arm(instruction: u32) -> String {
    Cpu::decode_arm(instruction).disassembly(instruction)
}

#[cfg_attr(feature="debugger", wasm_bindgen)]
pub fn disassemble_thumb(instruction: u16) -> String {
    Cpu::decode_thumb(instruction).disassembly(instruction)
}
