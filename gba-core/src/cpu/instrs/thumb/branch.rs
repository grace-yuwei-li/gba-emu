use super::ThumbInstruction;
use crate::utils::AddressableBits;
use crate::Bus;
use crate::Cpu;

pub struct ConditionalBranch;
pub struct Branch;
pub struct LongBranchWithLinkFirst;
pub struct LongBranchWithLinkSecond;

impl ThumbInstruction for ConditionalBranch {
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, instruction: u16) {
        let cond = instruction.bits(8, 11);
        let signed_imm = instruction.bits(0, 7) as i8;

        if cpu.check_cond(cond as u32) {
            let offset = i32::from(signed_imm) << 1;
            cpu.set_reg(15, cpu.get_reg(15).wrapping_add_signed(offset));
            cpu.flush_pipeline();
        }
    }

    fn disassembly(&self, instruction: u16) -> String {
        let cond = Cpu::disassemble_cond(instruction.bits(8, 11).into());
        let signed_imm = instruction.bits(0, 7) as i8;
        let offset = i32::from(signed_imm) << 1;
        let sign = if offset >= 0 { "+" } else { "-" };
        format!("B{}, PC{}{:x}", cond, sign, offset.abs())
    }
}

impl ThumbInstruction for Branch {
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, instruction: u16) {
        let signed_imm = sign_extend_i11(instruction.bits(0, 10));
        cpu.set_reg(15, cpu.get_reg(15).wrapping_add_signed(signed_imm << 1));
        cpu.flush_pipeline();
    }

    fn disassembly(&self, instruction: u16) -> String {
        let signed_imm = sign_extend_i11(instruction.bits(0, 10));
        let offset = signed_imm << 1;
        let sign = if offset >= 0 { "+" } else { "-" };
        format!("B, PC{}{:x}", sign, offset.abs())
    }
}

pub fn decode_long_branch_with_link(instruction: u16) -> Box<dyn ThumbInstruction> {
    let h = instruction.bits(11, 12);
    match h {
        0b10 => Box::new(LongBranchWithLinkFirst),
        0b11 => Box::new(LongBranchWithLinkSecond),
        // H == 01 form only exists from ARMv5
        _ => unreachable!(),
    }
}

impl ThumbInstruction for LongBranchWithLinkFirst {
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, instruction: u16) {
        let offset = sign_extend_i11(instruction.bits(0, 10));
        cpu.set_reg(14, cpu.get_reg(15).wrapping_add_signed(offset << 12));
    }

    fn disassembly(&self, instruction: u16) -> String {
        let offset = sign_extend_i11(instruction.bits(0, 10));
        format!("BL(1) {:x}", offset << 12)
    }
}

impl ThumbInstruction for LongBranchWithLinkSecond {
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, instruction: u16) {
        let offset = instruction.bits(0, 10);
        let lr = cpu.get_reg(14);
        let next_instr = cpu.get_reg(15) - 2;
        cpu.set_reg(15, lr + u32::from(offset << 1));
        cpu.set_reg(14, next_instr | 1);
        cpu.flush_pipeline();
    }

    fn disassembly(&self, instruction: u16) -> String {
        let offset = instruction.bits(0, 10);
        format!("BL(2) {:x}", offset << 1)
    }
}

fn sign_extend_i11(num: u16) -> i32 {
    i32::from((num as i16) << 5) >> 5
}
