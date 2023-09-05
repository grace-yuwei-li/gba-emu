use crate::bus::Bus;
use crate::cpu::{Cpu, CPSR};
use crate::utils::{sub_overflows, AddressableBits};

use super::ThumbInstruction;

struct ADD;
struct CMP;
struct MOV;
struct BX;

pub fn decode(instruction: u16) -> Box<dyn ThumbInstruction> {
    match instruction.bits(8, 9) {
        0b00 => Box::new(ADD),
        0b01 => Box::new(CMP),
        0b10 => Box::new(MOV),
        0b11 => Box::new(BX),
        _ => unreachable!(),
    }
}

impl ThumbInstruction for ADD {
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, instruction: u16) {
        let (rd, rs) = get_fields(instruction);
        let result = cpu.get_reg(rd).wrapping_add(cpu.get_reg(rs));
        if rd == 15 {
            cpu.set_reg(15, result & 0xfffffffe);
            cpu.flush_pipeline();
        } else {
            cpu.set_reg(rd, result);
        }
    }

    fn disassembly(&self, instruction: u16) -> String {
        let (rd, rs) = get_fields(instruction);
        format!("ADD r{}, r{}", rd, rs)
    }
}

impl ThumbInstruction for CMP {
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, instruction: u16) {
        let (rd, rs) = get_fields(instruction);
        let rd_val = cpu.get_reg(rd);
        let rs_val = cpu.get_reg(rs);
        let (result, borrow) = rd_val.overflowing_sub(rs_val);
        cpu.set_flag(CPSR::N, result.bit(31) == 1);
        cpu.set_flag(CPSR::Z, result == 0);
        cpu.set_flag(CPSR::C, !borrow);
        cpu.set_flag(CPSR::V, sub_overflows(rd_val, rs_val, result));
    }

    fn disassembly(&self, instruction: u16) -> String {
        let (rd, rs) = get_fields(instruction);
        format!("CMP r{}, r{}", rd, rs)
    }
}

impl ThumbInstruction for MOV {
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, instruction: u16) {
        let (rd, rs) = get_fields(instruction);
        if rd == 15 {
            cpu.set_reg(15, cpu.get_reg(rs) & 0xfffffffe);
            cpu.flush_pipeline();
        } else {
            cpu.set_reg(rd, cpu.get_reg(rs));
        }
    }

    fn disassembly(&self, instruction: u16) -> String {
        let (rd, rs) = get_fields(instruction);
        format!("MOV r{}, r{}", rd, rs)
    }
}

impl ThumbInstruction for BX {
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, instruction: u16) {
        let rm = instruction.bits(3, 6);
        let rm_val = cpu.get_reg(rm.into());

        cpu.set_flag(CPSR::T, rm_val.bit(0) == 1);
        cpu.set_reg(15, (rm_val.bits(1, 31) as u32) << 1);
        cpu.flush_pipeline();
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rm = instruction.bits(3, 6);
        format!("BX r{}", rm)
    }
}

fn get_fields(instruction: u16) -> (u32, u32) {
    let rd = instruction.bit(7) << 3 | instruction.bits(0, 2);
    let rs = instruction.bits(3, 6);
    (rd.into(), rs.into())
}
