use crate::{
    cpu::{Cpu, CPSR},
    utils::{AddressableBits, sub_overflows, add_overflows},
};

use super::ThumbInstruction;

struct AddImm;
struct AddReg;
struct SubImm;
struct SubReg;

pub fn decode(instruction: u16) -> Box<dyn ThumbInstruction> {
    let i = instruction.bit(10) == 1;
    let op = instruction.bit(9) == 1;

    match (i, op) {
        (false, false) => Box::new(AddReg),
        (false, true) => Box::new(SubReg),
        (true, false) => Box::new(AddImm),
        (true, true) => Box::new(SubImm),
    }
}

impl ThumbInstruction for AddImm {
    fn execute(&self, cpu: &mut Cpu, _: &mut crate::bus::Bus, instruction: u16) {
        let term = get_imm_term(instruction);
        let rd = instruction.bits(0, 2);
        let rs = instruction.bits(3, 5);

        execute_add(term, rd, rs, cpu);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rd = instruction.bits(0, 2);
        let rs = instruction.bits(3, 5);
        let imm = instruction.bits(6, 8);
        format!("ADD r{}, r{}, {:x}", rd, rs, imm)
    }
}

impl ThumbInstruction for AddReg {
    fn execute(&self, cpu: &mut Cpu, _: &mut crate::bus::Bus, instruction: u16) {
        let term = get_reg_term(instruction, cpu);
        let rd = instruction.bits(0, 2);
        let rs = instruction.bits(3, 5);

        execute_add(term, rd, rs, cpu);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rd = instruction.bits(0, 2);
        let rs = instruction.bits(3, 5);
        let rn = instruction.bits(6, 8);
        format!("ADD r{}, r{}, r{}", rd, rs, rn)
    }
}

impl ThumbInstruction for SubImm {
    fn execute(&self, cpu: &mut Cpu, _: &mut crate::bus::Bus, instruction: u16) {
        let term = get_imm_term(instruction);
        let rd = instruction.bits(0, 2);
        let rs = instruction.bits(3, 5);

        execute_sub(term, rd, rs, cpu);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rd = instruction.bits(0, 2);
        let rs = instruction.bits(3, 5);
        let imm = instruction.bits(6, 8);
        format!("SUB r{}, r{}, {:x}", rd, rs, imm)
    }
}

impl ThumbInstruction for SubReg {
    fn execute(&self, cpu: &mut Cpu, _: &mut crate::bus::Bus, instruction: u16) {
        let term = get_reg_term(instruction, cpu);
        let rd = instruction.bits(0, 2);
        let rs = instruction.bits(3, 5);

        execute_sub(term, rd, rs, cpu);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rd = instruction.bits(0, 2);
        let rs = instruction.bits(3, 5);
        let rn = instruction.bits(6, 8);
        format!("SUB r{}, r{}, r{}", rd, rs, rn)
    }
}

#[inline]
fn get_imm_term(instruction: u16) -> u32 {
    instruction.bits(6, 8).into()
}

#[inline]
fn get_reg_term(instruction: u16, cpu: &Cpu) -> u32 {
    let rn = instruction.bits(6, 8);
    cpu.get_reg(rn.into())
}

#[inline]
fn execute_add(term: u32, rd: u16, rs: u16, cpu: &mut Cpu) {
    let rs_val = cpu.get_reg(rs.into());
    let (result, carry) = rs_val.overflowing_add(term);

    cpu.set_reg(rd.into(), result);
    cpu.set_flag(CPSR::N, result.bit(31) == 1);
    cpu.set_flag(CPSR::Z, result == 0);
    cpu.set_flag(CPSR::C, carry);
    cpu.set_flag(CPSR::V, add_overflows(rs_val, term, result));
}

#[inline]
fn execute_sub(term: u32, rd: u16, rs: u16, cpu: &mut Cpu) {
    let rs_val = cpu.get_reg(rs.into());
    let (result, borrow) = rs_val.overflowing_sub(term);

    cpu.set_reg(rd.into(), result);
    cpu.set_flag(CPSR::N, result.bit(31) == 1);
    cpu.set_flag(CPSR::Z, result == 0);
    cpu.set_flag(CPSR::C, !borrow);
    cpu.set_flag(CPSR::V, sub_overflows(rs_val, term, result));
}
