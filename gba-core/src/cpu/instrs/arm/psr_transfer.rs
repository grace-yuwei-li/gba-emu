use crate::{bus::Bus, cpu::Cpu, utils::AddressableBits};

use super::{ArmInstruction, MetaInstr};

struct MsrFields {
    r: bool,
    field_mask: u32,
    operand: u32,
}

impl MsrFields {
    fn parse(instruction: u32, cpu: &Cpu) -> Self {
        let operand;

        if instruction.bit(25) == 1 {
            operand = if instruction.bit(25) == 1 {
                let rotate_imm = instruction.bits(8, 11);
                let imm = instruction.bits(0, 7);
                imm.rotate_right(2 * rotate_imm)
            } else {
                let rm = instruction.bits(0, 3);
                cpu.get_reg(rm as usize)
            };
        } else {
            let rm = instruction.bits(0, 3);
            operand = cpu.get_reg(rm as usize);
        }

        Self {
            r: instruction.bit(22) == 1,
            field_mask: instruction.bits(16, 19),
            operand,
        }
    }
}

struct MRS;
struct MSR;

impl ArmInstruction for MRS {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32) {
        todo!()
    }
    fn disassembly(&self, instruction: u32) -> String {
        "MRS".to_string()
    }
}

impl ArmInstruction for MSR {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32) {
        let unalloc_mask = 0x0fffff00;
        let user_mask = 0xf0000000;
        let priv_mask = 0x0000000f;
        let state_mask = 0x00000020;

        let fields = MsrFields::parse(instruction, cpu);
        let field_mask = fields.field_mask;

        if fields.operand & unalloc_mask != 0 {
            todo!("unpredictable")
        }

        let byte_mask: u32 = (if field_mask.bit(0) == 1 { 0xff } else { 0 })
            | (if field_mask.bit(1) == 1 { 0xff00 } else { 0 })
            | (if field_mask.bit(2) == 1 { 0xff0000 } else { 0 })
            | (if field_mask.bit(3) == 1 {
                0xff000000
            } else {
                0
            });

        if !fields.r {
            let mask;
            println!("{:?}", cpu.mode);
            if cpu.in_privileged_mode() {
                todo!();
            } else {
                mask = byte_mask & user_mask;
            }
            cpu.cpsr = (cpu.cpsr & !mask) | (fields.operand & mask);
        } else {
            todo!("true")
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        "MSR".to_string()
    }
}

impl MetaInstr {
    pub(super) fn decode_psr_transfer(instruction: u32) -> Box<dyn ArmInstruction> {
        if instruction.bit(21) == 0 {
            Box::new(MRS)
        } else {
            Box::new(MSR)
        }
    }
}
