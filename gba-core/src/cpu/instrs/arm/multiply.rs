use crate::{cpu::CPSR, utils::AddressableBits};

use super::ArmInstruction;

pub struct Mla;
pub struct Mul;
pub struct Umlal;
pub struct Smlal;
pub struct Umull;
pub struct Smull;

pub fn decode_multiply(instruction: u32) -> Box<dyn ArmInstruction> {
    if instruction.bit(21) == 1 {
        Box::new(Mla)
    } else {
        Box::new(Mul)
    }
}

pub fn decode_multiply_long(instruction: u32) -> Box<dyn ArmInstruction> {
    // bits are UA
    match instruction.bits(21, 22) {
        0b00 => Box::new(Umull),
        0b01 => Box::new(Umlal),
        0b10 => Box::new(Smull),
        0b11 => Box::new(Smlal),
        _ => unreachable!(),
    }
}

impl ArmInstruction for Mla {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, _: &mut crate::bus::Bus, instruction: u32) {
        let s = instruction.bit(20);
        let rd = instruction.bits(16, 19);
        let rn = cpu.get_reg(instruction.bits(12, 15));
        let rs = cpu.get_reg(instruction.bits(8, 11));
        let rm = cpu.get_reg(instruction.bits(0, 3));

        cpu.set_reg(rd, rm * rs + rn);
        if s == 1 {
            todo!("mla set flags")
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        let s = instruction.bit(20);
        let rd = instruction.bits(16, 19);
        let rn = instruction.bits(12, 15);
        let rs = instruction.bits(8, 11);
        let rm = instruction.bits(0, 3);
        format!(
            "MLA{} r{}, r{}, r{}, r{}",
            if s == 1 { "S" } else { "" },
            rd,
            rm,
            rs,
            rn
        )
    }
}

impl ArmInstruction for Mul {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, _: &mut crate::bus::Bus, instruction: u32) {
        let s = instruction.bit(20);
        let rd = instruction.bits(16, 19);
        let rs = cpu.get_reg(instruction.bits(8, 11));
        let rm = cpu.get_reg(instruction.bits(0, 3));

        let result = rm.wrapping_mul(rs);
        cpu.set_reg(rd, result);

        if s == 1 {
            cpu.set_flag(CPSR::N, result.bit(31) == 1);
            cpu.set_flag(CPSR::Z, result == 0);
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        let s = instruction.bit(20);
        let rd = instruction.bits(16, 19);
        let rs = instruction.bits(8, 11);
        let rm = instruction.bits(0, 3);
        format!(
            "MUL{} r{}, r{}, r{}",
            if s == 1 { "S" } else { "" },
            rd,
            rm,
            rs
        )
    }
}

impl ArmInstruction for Umlal {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, _: &mut crate::bus::Bus, instruction: u32) {
        let s = instruction.bit(20);
        let rd_hi = instruction.bits(16, 19);
        let rd_lo = instruction.bits(12, 15);
        let rs = cpu.get_reg(instruction.bits(8, 11));
        let rm = cpu.get_reg(instruction.bits(0, 3));

        let wide_result: u64 = (rs as u64) * (rm as u64);
        let wide_result_lo: u32 = wide_result.bits(0, 31).try_into().unwrap();
        let wide_result_hi: u32 = wide_result.bits(32, 63).try_into().unwrap();

        let (low, carry) = cpu.get_reg(rd_lo).overflowing_add(wide_result_lo);
        let high = cpu
            .get_reg(rd_hi)
            .wrapping_add(wide_result_hi)
            .wrapping_add(if carry { 1 } else { 0 });

        cpu.set_reg(rd_lo, low);
        cpu.set_reg(rd_hi, high);

        if s == 1 {
            cpu.set_flag(CPSR::N, high.bit(31) == 1);
            cpu.set_flag(CPSR::Z, low == 0 && high == 0);
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        let s = instruction.bit(20);
        let rd_hi = instruction.bits(16, 19);
        let rd_lo = instruction.bits(12, 15);
        let rs = instruction.bits(8, 11);
        let rm = instruction.bits(0, 3);

        format!(
            "UMLAL{} r{}, r{}, r{}, r{}",
            if s == 1 { "S" } else { "" },
            rd_lo,
            rd_hi,
            rm,
            rs
        )
    }
}

impl ArmInstruction for Smlal {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, _: &mut crate::bus::Bus, instruction: u32) {
        let s = instruction.bit(20);
        let rd_hi = instruction.bits(16, 19);
        let rd_lo = instruction.bits(12, 15);
        let rs = cpu.get_reg(instruction.bits(8, 11)) as i32;
        let rm = cpu.get_reg(instruction.bits(0, 3)) as i32;

        let wide_result: u64 = (i64::from(rs) * i64::from(rm)) as u64;
        let wide_result_lo: u32 = wide_result.bits(0, 31).try_into().unwrap();
        let wide_result_hi: u32 = wide_result.bits(32, 63).try_into().unwrap();

        let (low, carry) = cpu.get_reg(rd_lo).overflowing_add(wide_result_lo);
        let high = cpu
            .get_reg(rd_hi)
            .wrapping_add(wide_result_hi)
            .wrapping_add(if carry { 1 } else { 0 });

        cpu.set_reg(rd_lo, low);
        cpu.set_reg(rd_hi, high);

        if s == 1 {
            cpu.set_flag(CPSR::N, high.bit(31) == 1);
            cpu.set_flag(CPSR::Z, low == 0 && high == 0);
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        let s = instruction.bit(20);
        let rd_hi = instruction.bits(16, 19);
        let rd_lo = instruction.bits(12, 15);
        let rs = instruction.bits(8, 11);
        let rm = instruction.bits(0, 3);

        format!(
            "SMLAL{} r{}, r{}, r{}, r{}",
            if s == 1 { "S" } else { "" },
            rd_lo,
            rd_hi,
            rm,
            rs
        )
    }
}

impl ArmInstruction for Umull {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, _: &mut crate::bus::Bus, instruction: u32) {
        let s = instruction.bit(20);
        let rd_hi = instruction.bits(16, 19);
        let rd_lo = instruction.bits(12, 15);
        let rs = cpu.get_reg(instruction.bits(8, 11));
        let rm = cpu.get_reg(instruction.bits(0, 3));

        let wide_result: u64 = (rs as u64) * (rm as u64);
        let low: u32 = wide_result.bits(0, 31).try_into().unwrap();
        let high: u32 = wide_result.bits(32, 63).try_into().unwrap();

        cpu.set_reg(rd_lo, low);
        cpu.set_reg(rd_hi, high);

        if s == 1 {
            cpu.set_flag(CPSR::N, high.bit(31) == 1);
            cpu.set_flag(CPSR::Z, low == 0 && high == 0);
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        let s = instruction.bit(20);
        let rd_hi = instruction.bits(16, 19);
        let rd_lo = instruction.bits(12, 15);
        let rs = instruction.bits(8, 11);
        let rm = instruction.bits(0, 3);

        format!(
            "UMULL{} r{}, r{}, r{}, r{}",
            if s == 1 { "S" } else { "" },
            rd_lo,
            rd_hi,
            rm,
            rs
        )
    }
}

impl ArmInstruction for Smull {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, _: &mut crate::bus::Bus, instruction: u32) {
        let s = instruction.bit(20);
        let rd_hi = instruction.bits(16, 19);
        let rd_lo = instruction.bits(12, 15);
        let rs = cpu.get_reg(instruction.bits(8, 11)) as i32;
        let rm = cpu.get_reg(instruction.bits(0, 3)) as i32;

        let wide_result: u64 = (i64::from(rs) * i64::from(rm)) as u64;
        let low: u32 = wide_result.bits(0, 31).try_into().unwrap();
        let high: u32 = wide_result.bits(32, 63).try_into().unwrap();

        cpu.set_reg(rd_lo, low);
        cpu.set_reg(rd_hi, high);

        if s == 1 {
            cpu.set_flag(CPSR::N, high.bit(31) == 1);
            cpu.set_flag(CPSR::Z, low == 0 && high == 0);
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        let s = instruction.bit(20);
        let rd_hi = instruction.bits(16, 19);
        let rd_lo = instruction.bits(12, 15);
        let rs = instruction.bits(8, 11);
        let rm = instruction.bits(0, 3);

        format!(
            "SMULL{} r{}, r{}, r{}, r{}",
            if s == 1 { "S" } else { "" },
            rd_lo,
            rd_hi,
            rm,
            rs
        )
    }
}
