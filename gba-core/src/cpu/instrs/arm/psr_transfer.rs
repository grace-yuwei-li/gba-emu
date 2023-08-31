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
                cpu.get_reg(rm)
            };
        } else {
            let rm = instruction.bits(0, 3);
            operand = cpu.get_reg(rm);
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
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, instruction: u32) {
        let r = instruction.bit(22);
        let rd = instruction.bits(12, 15);
        if r == 1 {
            cpu.set_reg(rd, cpu.regs.spsr(&cpu.get_mode()));
        } else {
            cpu.set_reg(rd, cpu.regs.cpsr);
        }
    }
    fn disassembly(&self, instruction: u32) -> String {
        let r = instruction.bit(22);
        let rd = instruction.bits(12, 15);
        format!("MRS r{}, {}", rd, if r == 1 { "SPSR" } else { "CPSR" })
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
            //todo!("unpredictable")
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
            println!("{:?}", cpu.get_mode());
            if cpu.in_privileged_mode() {
                if (fields.operand & state_mask) != 0 {
                    todo!("unpredictable");
                } else {
                    mask = byte_mask & (user_mask | priv_mask);
                }
            } else {
                mask = byte_mask & user_mask;
            }
            cpu.regs.cpsr = (cpu.regs.cpsr & !mask) | (fields.operand & mask);
        } else {
            if cpu.mode_has_spsr() {
                let mask = byte_mask & (user_mask | priv_mask | state_mask);
                *cpu.regs.spsr_mut(&cpu.get_mode()) = (cpu.regs.spsr(&cpu.get_mode()) & !mask) | (fields.operand & mask);
            } else {
                todo!("unpredictable")
            }
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        let r = instruction.bit(22);
        let field_mask = instruction.bits(16, 19);
        let fields = ["c", "x", "s", "f"]
            .into_iter()
            .enumerate()
            .filter_map(|(i, f)| if field_mask.bit(i) == 1 {
                Some(f.to_string())
            } else {
                None
            })
            .collect::<Vec<String>>()
            .join("");

        let operand = if instruction.bit(25) == 1 {
            let rotate_imm = instruction.bits(8, 11);
            let imm = instruction.bits(0, 7);
            format!("#{}", imm.rotate_right(2 * rotate_imm))
        } else {
            let rm = instruction.bits(0, 3);
            format!("r{}", rm)
        };
        format!("MSR {}_{}, {}", if r == 1 { "SPSR" } else { "CPSR" }, fields, operand)
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
