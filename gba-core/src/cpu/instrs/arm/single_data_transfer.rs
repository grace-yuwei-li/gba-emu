use crate::logging::Targets;
use crate::utils::AddressableBits;
use crate::Bus;
use crate::Cpu;
use tracing::trace;

use super::ArmInstruction;
use super::MetaInstr;

struct STR;
struct LDR;

impl ArmInstruction for STR {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32) {
        let i = instruction.bit(25);
        let p = instruction.bit(24);
        let u = instruction.bit(23);
        let b = instruction.bit(22);
        let w = instruction.bit(21);

        let rn = instruction.bits(16, 19);
        let rd = instruction.bits(12, 15);

        if b == 1 {
            todo!();
        }
        println!(
            "I {} P {} U {} B {} W {}",
            i == 1,
            p == 1,
            u == 1,
            b == 1,
            w == 1
        );

        match (i == 1, p == 1, u == 1, b == 1, w == 1) {
            (false, false, u, false, false) => {
                let address = cpu.get_reg(rn as usize);
                bus.write(address, cpu.get_reg(rd as usize));
            }
            (false, true, u, false, false) => {
                let address = cpu.get_reg(rn as usize) + instruction.bits(0, 11);
                bus.write(address, cpu.get_reg(rd as usize));
            }
            _ => todo!(),
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        let b = instruction.bit(22);
        format!("STR{}", if b == 1 { "B" } else { "" })
    }
}

impl ArmInstruction for LDR {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32) {
        let i = instruction.bit(25);
        let p = instruction.bit(24);
        let u = instruction.bit(23);
        let b = instruction.bit(22);
        let w = instruction.bit(21);

        let rn = instruction.bits(16, 19);
        let rd = instruction.bits(12, 15);
        let offset = instruction.bits(0, 12);

        println!(
            "I {} P {} U {} B {} W {}",
            i == 1,
            p == 1,
            u == 1,
            b == 1,
            w == 1
        );

        if b == 1 {
            todo!();
        }

        let address;
        match (i == 1, p == 1, u == 1, b == 1, w == 1) {
            (false, true, u, false, false) => {
                address = if u {
                    cpu.get_reg(rn as usize) + offset
                } else {
                    cpu.get_reg(rn as usize) - offset
                };
            }
            _ => todo!(),
        }

        cpu.set_reg(rd as usize, bus.read(address));

        if rd == 15 {
            cpu.flush_pipeline();
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        let b = instruction.bit(22);
        format!("LDR{}", if b == 1 { "B" } else { "" })
    }
}

impl MetaInstr {
    pub(super) fn decode_single_data_transfer(instruction: u32) -> Box<dyn ArmInstruction> {
        let l = instruction.bit(20);

        if l == 1 {
            Box::new(LDR)
        } else {
            Box::new(STR)
        }
    }
}
