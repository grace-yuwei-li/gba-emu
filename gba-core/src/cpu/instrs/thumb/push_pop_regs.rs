use crate::utils::{AddressableBits, reg_list};

use super::ThumbInstruction;

struct Push;
struct Pop;

pub fn decode(instruction: u16) -> Box<dyn ThumbInstruction> {
    let l = instruction.bit(11);

    if l == 0 {
        Box::new(Push)
    } else {
        Box::new(Pop)
    }
}

impl ThumbInstruction for Push {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, bus: &mut crate::bus::Bus, instruction: u16) {
        let regs = instruction.bits(0, 8);
        let start_address = cpu.get_reg(13) - 4 * regs.count_ones();
        let mut address = start_address;

        for i in 0..=7 {
            if regs.bit(i) == 1 {
                bus.write(address, cpu.get_reg(i.try_into().unwrap()));
                address += 4;
            }
        }
        if regs.bit(8) == 1 {
            bus.write(address, cpu.get_reg(14));
        }

        cpu.set_reg(13, start_address);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let regs = reg_list(instruction.into(), 8);
        let lr = if instruction.bit(8) == 1 { ",LR" } else { "" };
        format!("PUSH {{{}{}}}", regs, lr)
    }
}

impl ThumbInstruction for Pop {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, bus: &mut crate::bus::Bus, instruction: u16) {
        let regs = instruction.bits(0, 8);
        let start_address = cpu.get_reg(13);
        let mut address = start_address;

        for i in 0..=7 {
            if regs.bit(i) == 1 {
                cpu.set_reg(i.try_into().unwrap(), bus.read(address, cpu));
                address += 4;
            }
        }
        if regs.bit(8) == 1 {
            let value = bus.read(address, cpu);
            cpu.set_reg(15, value & 0xfffffffe);
            cpu.flush_pipeline();
            address += 4;
        }

        cpu.set_reg(13, address);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let regs = reg_list(instruction.into(), 8);
        let pc = if instruction.bit(8) == 1 { ",PC" } else { "" };
        format!("POP {{{}{}}}", regs, pc)
    }
}
