use crate::utils::AddressableBits;

use super::ArmInstruction;

pub struct Mla;
pub struct Mul;

pub fn decode_multiply(instruction: u32) -> Box<dyn ArmInstruction> {
    if instruction.bit(21) == 1 {
        Box::new(Mla)
    } else {
        Box::new(Mul)
    }
}

impl ArmInstruction for Mla {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, bus: &mut crate::bus::Bus, instruction: u32) {
        // Do the wrong thing
        let s = instruction.bit(20);
        let rd = instruction.bits(16, 19);
        let rn = instruction.bits(12, 15);
        let rs = instruction.bits(8, 11);
        let rm = instruction.bits(0, 3);

        let rn = cpu.get_reg(rn);
        let rs = cpu.get_reg(rs);
        let rm = cpu.get_reg(rm);

        cpu.set_reg(rd, rm * rs + rn);
        if s == 1 {
            todo!()
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        let s = instruction.bit(20);
        let rd = instruction.bits(16, 19);
        let rn = instruction.bits(12, 15);
        let rs = instruction.bits(8, 11);
        let rm = instruction.bits(0, 3);
        format!("MLA{} r{}, r{}, r{}, r{}", if s == 1 { "S" } else { "" }, rd, rm, rs, rn)
    }
}

impl ArmInstruction for Mul {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, bus: &mut crate::bus::Bus, instruction: u32) {
        // Do the wrong thing
    }

    fn disassembly(&self, instruction: u32) -> String {
        format!("MUL")
    }
}
