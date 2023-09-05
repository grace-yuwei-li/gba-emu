use crate::utils::AddressableBits;

use super::ThumbInstruction;

pub(super) struct LdrPc;

impl ThumbInstruction for LdrPc {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, bus: &mut crate::bus::Bus, instruction: u16) {
        let rd = instruction.bits(8, 10);
        let imm = instruction.bits(0, 7);

        let address = (cpu.get_reg(15) & 0xffff_fffc) + imm as u32 * 4;
        let value = bus.read(address, cpu);
        cpu.set_reg(rd.into(), value);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rd = instruction.bits(8, 10);
        let imm = instruction.bits(0, 7);
        format!("LDR r{}, [PC, {:x}]", rd, imm)
    }
}
