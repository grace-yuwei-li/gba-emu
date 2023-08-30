use super::ArmInstruction;
use crate::cpu::{Cpu, Mode};
use crate::bus::Bus;
use crate::utils::AddressableBits;

pub struct Swi;

impl ArmInstruction for Swi {
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, _: u32) {
        cpu.set_reg_with_mode(14, Mode::Supervisor, cpu.get_reg(15) - 4);
        *cpu.regs.spsr_mut(&Mode::Supervisor) = cpu.regs.cpsr;
        cpu.regs.cpsr = cpu.regs.cpsr.bits(6, 31) | 0b010011;
        cpu.regs.cpsr.mut_bit(7, true);
        cpu.set_reg(15, 0x8);
        cpu.flush_pipeline();
    }

    fn disassembly(&self, instruction: u32) -> String {
        let imm = instruction.bits(0, 23);
        format!("SWI {:x}", imm)
    }
}
