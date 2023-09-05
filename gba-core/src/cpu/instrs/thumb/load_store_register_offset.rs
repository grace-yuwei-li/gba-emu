use crate::{bus::Bus, cpu::Cpu, utils::AddressableBits};

use super::ThumbInstruction;

struct Ldr;
struct Ldrb;
struct Str;
struct Strb;

pub fn decode(instruction: u16) -> Box<dyn ThumbInstruction> {
    let l = instruction.bit(11) == 1;
    let b = instruction.bit(10) == 1;
    match (l, b) {
        (false, false) => Box::new(Str),
        (true, false) => Box::new(Ldr),
        (false, true) => Box::new(Strb),
        (true, true) => Box::new(Ldrb),
    }
}

impl ThumbInstruction for Str {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u16) {
        let rm = instruction.bits(6, 8);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        let address = cpu.get_reg(rn.into()).wrapping_add(cpu.get_reg(rm.into()));
        bus.write(address, cpu.get_reg(rd.into()));
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rm = instruction.bits(6, 8);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);
        format!("STR r{}, [r{}, r{}]", rd, rn, rm)
    }
}

impl ThumbInstruction for Strb {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u16) {
        let rm = instruction.bits(6, 8);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        let address = cpu.get_reg(rn.into()).wrapping_add(cpu.get_reg(rm.into()));
        bus.write_byte(address, cpu.get_reg(rd.into()) as u8);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rm = instruction.bits(6, 8);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);
        format!("STRB r{}, [r{}, r{}]", rd, rn, rm)
    }
}

impl ThumbInstruction for Ldr {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u16) {
        let rm = instruction.bits(6, 8);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        let address = cpu.get_reg(rn.into()).wrapping_add(cpu.get_reg(rm.into()));
        let data = bus.read(address, cpu);
        cpu.set_reg(rd.into(), data);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rm = instruction.bits(6, 8);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);
        format!("LDR r{}, [r{}, r{}]", rd, rn, rm)
    }
}

impl ThumbInstruction for Ldrb {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u16) {
        let rm = instruction.bits(6, 8);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        let address = cpu.get_reg(rn.into()).wrapping_add(cpu.get_reg(rm.into()));
        let data = bus.read_byte(address, cpu);
        cpu.set_reg(rd.into(), data.into());
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rm = instruction.bits(6, 8);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);
        format!("LDRB r{}, [r{}, r{}]", rd, rn, rm)
    }
}
