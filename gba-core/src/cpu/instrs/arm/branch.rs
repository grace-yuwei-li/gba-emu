use crate::logging::Targets;
use crate::Bus;
use crate::Cpu;
use tracing::trace;

use super::ArmInstruction;

pub struct Branch;
impl ArmInstruction for Branch {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus, instruction: u32) {
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
        format!("B{} PC+{:x}", if link { "L" } else { "" }, offset)
    }
}
