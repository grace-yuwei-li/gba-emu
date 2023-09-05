use crate::{
    bus::Bus,
    cpu::{Cpu, CPSR},
    utils::{add_overflows, sub_overflows, AddressableBits},
};

use super::ThumbInstruction;

struct Mov;
struct Cmp;
struct Add;
struct Sub;

pub fn decode(instruction: u16) -> Box<dyn ThumbInstruction> {
    let op = instruction.bits(11, 12);
    match op {
        0b00 => Box::new(Mov),
        0b01 => Box::new(Cmp),
        0b10 => Box::new(Add),
        0b11 => Box::new(Sub),
        _ => unreachable!(),
    }
}

impl ThumbInstruction for Mov {
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, instruction: u16) {
        let rd = instruction.bits(8, 10);
        let imm = instruction.bits(0, 7);

        cpu.set_reg(rd.into(), imm as u32);

        cpu.set_flag(CPSR::N, false);
        cpu.set_flag(CPSR::Z, imm == 0);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rd = instruction.bits(8, 10);
        let offset = instruction.bits(0, 7);
        format!("MOV r{}, {:x}", rd, offset)
    }
}

impl ThumbInstruction for Cmp {
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, instruction: u16) {
        let rn = instruction.bits(8, 10);
        let rn_val = cpu.get_reg(rn.into());
        let imm = instruction.bits(0, 7) as u32;

        let (result, borrow) = rn_val.overflowing_sub(imm);
        let overflow = sub_overflows(rn_val, imm, result);

        cpu.set_flag(CPSR::N, result.bit(31) == 1);
        cpu.set_flag(CPSR::Z, result == 0);
        cpu.set_flag(CPSR::C, !borrow);
        cpu.set_flag(CPSR::V, overflow);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rn = instruction.bits(8, 10);
        let offset = instruction.bits(0, 7);
        format!("CMP r{}, {:x}", rn, offset)
    }
}

impl ThumbInstruction for Add {
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, instruction: u16) {
        let rn = instruction.bits(8, 10);
        let rn_val = cpu.get_reg(rn.into());
        let imm = instruction.bits(0, 7) as u32;

        let (result, carry) = rn_val.overflowing_add(imm);
        let overflow = add_overflows(rn_val, imm, result);

        cpu.set_reg(rn.into(), result);
        cpu.set_flag(CPSR::N, result.bit(31) == 1);
        cpu.set_flag(CPSR::Z, result == 0);
        cpu.set_flag(CPSR::C, carry);
        cpu.set_flag(CPSR::V, overflow);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rn = instruction.bits(8, 10);
        let offset = instruction.bits(0, 7);
        format!("ADD r{}, {:x}", rn, offset)
    }
}

impl ThumbInstruction for Sub {
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, instruction: u16) {
        let rn = instruction.bits(8, 10);
        let rn_val = cpu.get_reg(rn.into());
        let imm = instruction.bits(0, 7) as u32;

        let (result, borrow) = rn_val.overflowing_sub(imm);
        let overflow = sub_overflows(rn_val, imm, result);

        cpu.set_reg(rn.into(), result);
        cpu.set_flag(CPSR::N, result.bit(31) == 1);
        cpu.set_flag(CPSR::Z, result == 0);
        cpu.set_flag(CPSR::C, !borrow);
        cpu.set_flag(CPSR::V, overflow);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rn = instruction.bits(8, 10);
        let offset = instruction.bits(0, 7);
        format!("SUB r{}, {:x}", rn, offset)
    }
}
