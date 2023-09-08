use crate::bus::Bus;
use crate::cpu::{Cpu, Mode};
use crate::utils::AddressableBits;

use super::ThumbInstruction;

pub struct Swi;

impl ThumbInstruction for Swi {
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, _: u16) {
        cpu.set_reg_with_mode(14, Mode::Supervisor, cpu.get_reg(15) - 2);
        *cpu.regs.spsr_mut(&Mode::Supervisor) = cpu.regs.cpsr;
        cpu.regs.cpsr = cpu.regs.cpsr.bits(6, 31) | 0b010011;
        cpu.regs.cpsr.mut_bit(7, true);
        cpu.set_reg(15, 0x8);
        cpu.flush_pipeline();
    }

    fn disassembly(&self, instruction: u16) -> String {
        let imm = instruction.bits(0, 23);
        format!("SWI {:x}", imm)
    }
}
