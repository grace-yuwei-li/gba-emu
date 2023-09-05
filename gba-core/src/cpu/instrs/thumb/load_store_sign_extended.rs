use crate::{bus::Bus, cpu::Cpu, utils::AddressableBits};

use super::ThumbInstruction;

struct Strh;
struct Ldrh;
struct Ldrsb;
struct Ldrsh;

pub fn decode(instruction: u16) -> Box<dyn ThumbInstruction> {
    let h = instruction.bit(11) == 1;
    let s = instruction.bit(10) == 1;
    match (s, h) {
        (false, false) => Box::new(Strh),
        (false, true) => Box::new(Ldrh),
        (true, false) => Box::new(Ldrsb),
        (true, true) => Box::new(Ldrsh),
    }
}

impl ThumbInstruction for Strh {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u16) {
        let rm = instruction.bits(6, 8);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        let address = cpu.get_reg(rn.into()).wrapping_add(cpu.get_reg(rm.into()));
        bus.write_half(address, cpu.get_reg(rd.into()) as u16);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rm = instruction.bits(6, 8);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);
        format!("STRH r{}, [r{}, r{}]", rd, rn, rm)
    }
}

impl ThumbInstruction for Ldrh {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u16) {
        let rm = instruction.bits(6, 8);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        let address = cpu.get_reg(rn.into()).wrapping_add(cpu.get_reg(rm.into()));
        let data = bus.read_half(address, cpu);
        cpu.set_reg(rd.into(), data.into());
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rm = instruction.bits(6, 8);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);
        format!("LDRH r{}, [r{}, r{}]", rd, rn, rm)
    }
}

impl ThumbInstruction for Ldrsb {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u16) {
        let rm = instruction.bits(6, 8);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        let address = cpu.get_reg(rn.into()).wrapping_add(cpu.get_reg(rm.into()));
        let data = i32::from(bus.read_byte(address, cpu) as i8) as u32;
        cpu.set_reg(rd.into(), data);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rm = instruction.bits(6, 8);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);
        format!("LDRSB r{}, [r{}, r{}]", rd, rn, rm)
    }
}

impl ThumbInstruction for Ldrsh {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u16) {
        let rm = instruction.bits(6, 8);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        let address = cpu.get_reg(rn.into()).wrapping_add(cpu.get_reg(rm.into()));

        // Misaligned LDRSH loads the signed byte quantity stored at the offset address
        let data: u32 = if address.bit(0) == 0 {
            i32::from(bus.read_half(address, cpu) as i16) as u32
        } else {
            i32::from(bus.read_byte(address, cpu) as i8) as u32
        };
        cpu.set_reg(rd.into(), data);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rm = instruction.bits(6, 8);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);
        format!("LDRSH r{}, [r{}, r{}]", rd, rn, rm)
    }
}
