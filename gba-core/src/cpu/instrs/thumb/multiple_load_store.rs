use crate::utils::{reg_list, AddressableBits};

use super::ThumbInstruction;

struct Stmia;
struct Ldmia;

pub fn decode(instruction: u16) -> Box<dyn ThumbInstruction> {
    if instruction.bit(11) == 0 {
        Box::new(Stmia)
    } else {
        Box::new(Ldmia)
    }
}

impl ThumbInstruction for Stmia {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, bus: &mut crate::bus::Bus, instruction: u16) {
        let rn = instruction.bits(8, 10);
        let reg_list = instruction.bits(0, 7);
        let start_address = cpu.get_reg(rn.into());

        if reg_list != 0 {
            let mut address = start_address;
            let mut first_reg = true;

            for i in 0..=7 {
                if reg_list.bit(i) == 1 {
                    bus.write(address, cpu.get_reg(i.try_into().unwrap()));
                    address += 4;

                    if first_reg {
                        first_reg = false;
                        let write_back = cpu.get_reg(rn.into()) + reg_list.count_ones() * 4;
                        cpu.set_reg(rn.into(), write_back);
                    }
                }
            }
        } else {
            bus.write(start_address, cpu.get_reg(15) + 2);
            cpu.set_reg(rn.into(), cpu.get_reg(rn.into()) + 0x40);
        }
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rn = instruction.bits(8, 10);
        let regs = instruction.bits(0, 7);
        format!("STMIA r{}!, {{{}}}", rn, reg_list(regs.into(), 8))
    }
}

impl ThumbInstruction for Ldmia {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, bus: &mut crate::bus::Bus, instruction: u16) {
        let rn = instruction.bits(8, 10);
        let reg_list = instruction.bits(0, 7);
        let start_address = cpu.get_reg(rn.into());

        if reg_list != 0 {
            let mut address = start_address;
            for i in 0..=7 {
                if reg_list.bit(i) == 1 {
                    cpu.set_reg(i.try_into().unwrap(), bus.read(address, cpu));
                    address += 4;
                }
            }
            cpu.set_reg(
                rn.into(),
                cpu.get_reg(rn.into()) + reg_list.count_ones() * 4,
            );
        } else {
            // Empty register list loads value from memory into PC
            let value = bus.read(start_address, cpu);
            cpu.set_reg(15, value & 0xffff_fffc);
            cpu.flush_pipeline();
            cpu.set_reg(rn.into(), cpu.get_reg(rn.into()) + 0x40);
        }
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rn = instruction.bits(8, 10);
        let regs = instruction.bits(0, 7);
        format!("LDMIA r{}!, {{{}}}", rn, reg_list(regs.into(), 8))
    }
}
