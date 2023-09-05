use crate::utils::AddressableBits;

use super::ThumbInstruction;

struct Add;

pub fn decode() -> Box<dyn ThumbInstruction> {
    Box::new(Add)
}

impl ThumbInstruction for Add {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, _: &mut crate::bus::Bus, instruction: u16) {
        let sign = instruction.bit(7);
        let imm: u32 = instruction.bits(0, 6).into();
        if sign == 0 {
            cpu.set_reg(13, cpu.get_reg(13).wrapping_add(imm * 4));
        } else {
            cpu.set_reg(13, cpu.get_reg(13).wrapping_sub(imm * 4));
        }
    }

    fn disassembly(&self, instruction: u16) -> String {
        let sign = instruction.bit(7);
        let imm = instruction.bits(0, 6);
        if sign == 0 {
            format!("ADD SP, {:x}", imm * 4)
        } else {
            format!("SUB SP, {:x}", imm * 4)
        }
    }
}
