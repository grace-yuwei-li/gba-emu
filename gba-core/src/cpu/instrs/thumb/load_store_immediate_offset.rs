use crate::{bus::Bus, cpu::Cpu, utils::AddressableBits};

use super::ThumbInstruction;

struct Ldr;
struct Ldrb;
struct Str;
struct Strb;

pub fn decode(instruction: u16) -> Box<dyn ThumbInstruction> {
    let b = instruction.bit(12) == 1;
    let l = instruction.bit(11) == 1;
    match (l, b) {
        (false, false) => Box::new(Str),
        (true, false) => Box::new(Ldr),
        (false, true) => Box::new(Strb),
        (true, true) => Box::new(Ldrb),
    }
}

impl ThumbInstruction for Str {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u16) {
        let offset = instruction.bits(6, 10);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        let address = cpu.get_reg(rn.into()) + 4 * offset as u32;

        // How does misalignment work here?
        bus.write(address, cpu.get_reg(rd.into()));
    }

    fn disassembly(&self, instruction: u16) -> String {
        let offset = instruction.bits(6, 10);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        format!("STR r{}, [r{}, {:x}]", rd, rn, offset * 4)
    }
}

impl ThumbInstruction for Strb {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u16) {
        let offset = instruction.bits(6, 10);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        let address = cpu.get_reg(rn.into()) + offset as u32;

        bus.write_byte(address, cpu.get_reg(rd.into()) as u8);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let offset = instruction.bits(6, 10);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        format!("STRB r{}, [r{}, {:x}]", rd, rn, offset)
    }
}

impl ThumbInstruction for Ldr {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u16) {
        let offset = instruction.bits(6, 10);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        let address = cpu.get_reg(rn.into()) + 4 * offset as u32;

        // How does misalignment work here?
        let data = bus.read(address, cpu);

        cpu.set_reg(rd.into(), data);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let offset = instruction.bits(6, 10);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        format!("LDR r{}, [r{}, {:x}]", rd, rn, offset * 4)
    }
}

impl ThumbInstruction for Ldrb {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u16) {
        let offset = instruction.bits(6, 10);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        let address = cpu.get_reg(rn.into()) + offset as u32;

        let data = bus.read_byte(address, cpu);

        cpu.set_reg(rd.into(), data.into());
    }

    fn disassembly(&self, instruction: u16) -> String {
        let offset = instruction.bits(6, 10);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        format!("LDRB r{}, [r{}, {:x}]", rd, rn, offset)
    }
}
