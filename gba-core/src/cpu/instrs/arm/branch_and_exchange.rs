use crate::cpu::State;
use crate::utils::AddressableBits;
use crate::Bus;
use crate::Cpu;

use super::ArmInstruction;

pub struct BranchAndExchange;
impl ArmInstruction for BranchAndExchange {
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, instruction: u32) {
        let rn = instruction.bits(0, 3);

        if rn == 15 {
            todo!("undefined behaviour");
        }

        let dest = cpu.get_reg(rn);

        if dest.bit(0) == 1 {
            cpu.set_state(State::Thumb);
        }

        cpu.set_reg(15, dest & 0xffff_fffe);

        cpu.flush_pipeline();
    }

    fn disassembly(&self, instruction: u32) -> String {
        let rn = instruction.bits(0, 3);
        format!("BX r{}", rn)
    }
}
