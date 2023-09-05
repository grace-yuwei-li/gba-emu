use crate::{bus::Bus, cpu::Cpu, utils::AddressableBits};

use super::ThumbInstruction;

struct Ldrh;
struct Strh;

pub fn decode(instruction: u16) -> Box<dyn ThumbInstruction> {
    if instruction.bit(11) == 0 {
        Box::new(Strh)
    } else {
        Box::new(Ldrh)
    }
}

impl ThumbInstruction for Strh {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u16) {
        let offset = instruction.bits(6, 10);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        let address = cpu.get_reg(rn.into()) + 2 * offset as u32;

        // How does misalignment work here?
        bus.write_half(address, cpu.get_reg(rd.into()) as u16);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let offset = instruction.bits(6, 10);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        format!("STRH r{}, [r{}, {:x}]", rd, rn, offset * 2)
    }
}

impl ThumbInstruction for Ldrh {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u16) {
        let offset = instruction.bits(6, 10);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        let address = cpu.get_reg(rn.into()) + 2 * offset as u32;

        // How does misalignment work here?
        let data = bus.read_half(address, cpu);

        cpu.set_reg(rd.into(), data);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let offset = instruction.bits(6, 10);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        format!("LDRH r{}, [r{}, {:x}]", rd, rn, offset * 2)
    }
}
