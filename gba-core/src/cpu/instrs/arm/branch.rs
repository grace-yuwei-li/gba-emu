use crate::Bus;
use crate::Cpu;

use super::ArmInstruction;

pub struct Branch;
impl ArmInstruction for Branch {
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, instruction: u32) {
        let link = (instruction >> 24) & 1 != 0;
        let offset = instruction & 0xffffff;
        let offset = ((offset << 8) as i32) >> 6;

        if link {
            cpu.set_reg(14, cpu.get_reg(15) - 4);
        }

        let dest = cpu.get_reg(15).wrapping_add_signed(offset);

        cpu.set_reg(15, dest);
        cpu.flush_pipeline();
    }

    fn disassembly(&self, instruction: u32) -> String {
        let link = (instruction >> 24) & 1 != 0;
        let offset = instruction & 0xffffff;
        let offset = ((offset << 8) as i32) >> 6;

        let cond = Cpu::disassemble_cond(instruction);
        format!("B{}{} PC+{:x}", if link { "L" } else { "" }, cond, offset)
    }
}
