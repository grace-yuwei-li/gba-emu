use crate::{cpu::CPSR, utils::AddressableBits};

use super::ThumbInstruction;

struct LSL;
struct LSR;
struct ASR;

pub fn decode(instruction: u16) -> Box<dyn ThumbInstruction> {
    let opcode = instruction.bits(11, 12);
    match opcode {
        0b00 => Box::new(LSL),
        0b01 => Box::new(LSR),
        0b10 => Box::new(ASR),
        0b11 => unreachable!(),
        _ => unreachable!(),
    }
}

impl ThumbInstruction for LSL {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, _: &mut crate::bus::Bus, instruction: u16) {
        let imm = instruction.bits(6, 10);
        let rm = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);
        if imm == 0 {
            cpu.set_reg(rd.into(), cpu.get_reg(rm.into()));
        } else {
            cpu.set_flag(CPSR::C, cpu.get_reg(rm.into()).bit(32 - imm as usize) == 1);
            cpu.set_reg(rd.into(), cpu.get_reg(rm.into()) << imm);
        }
        cpu.set_flag(CPSR::N, cpu.get_reg(rd.into()).bit(31) == 1);
        cpu.set_flag(CPSR::Z, cpu.get_reg(rd.into()) == 0);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let imm = instruction.bits(6, 10);
        let rm = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        format!("LSL r{}, r{}, {:x}", rd, rm, imm)
    }
}

impl ThumbInstruction for LSR {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, _: &mut crate::bus::Bus, instruction: u16) {
        let imm = instruction.bits(6, 10);
        let rm = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);
        if imm == 0 {
            cpu.set_flag(CPSR::C, cpu.get_reg(rm.into()).bit(31) == 1);
            cpu.set_reg(rd.into(), 0);
        } else {
            cpu.set_flag(CPSR::C, cpu.get_reg(rm.into()).bit(imm as usize - 1) == 1);
            cpu.set_reg(rd.into(), cpu.get_reg(rm.into()) >> imm);
        }
        cpu.set_flag(CPSR::N, cpu.get_reg(rd.into()).bit(31) == 1);
        cpu.set_flag(CPSR::Z, cpu.get_reg(rd.into()) == 0);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let imm = instruction.bits(6, 10);
        let rm = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        format!("LSR r{}, r{}, {:x}", rd, rm, imm)
    }
}

impl ThumbInstruction for ASR {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, _: &mut crate::bus::Bus, instruction: u16) {
        let imm = instruction.bits(6, 10);
        let rm = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);
        if imm == 0 {
            cpu.set_flag(CPSR::C, cpu.get_reg(rm.into()).bit(31) == 1);
            if cpu.get_reg(rm.into()).bit(31) == 0 {
                cpu.set_reg(rd.into(), 0);
            } else {
                cpu.set_reg(rd.into(), 0xffff_ffff);
            }
        } else {
            cpu.set_flag(CPSR::C, cpu.get_reg(rm.into()).bit(imm as usize - 1) == 1);
            cpu.set_reg(rd.into(), ((cpu.get_reg(rm.into()) as i32) >> imm) as u32);
        }
        cpu.set_flag(CPSR::N, cpu.get_reg(rd.into()).bit(31) == 1);
        cpu.set_flag(CPSR::Z, cpu.get_reg(rd.into()) == 0);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let imm = instruction.bits(6, 10);
        let rm = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        format!("ASR r{}, r{}, {:x}", rd, rm, imm)
    }
}
