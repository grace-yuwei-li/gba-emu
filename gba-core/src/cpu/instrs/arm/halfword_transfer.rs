use crate::{bus::Bus, cpu::Cpu, utils::AddressableBits};

use super::{ArmInstruction, MetaInstr, UnimplementedInstruction};

struct HalfwordTransFields {
    p: bool,
    u: bool,
    w: bool,
    l: bool,
    rn: u32,
    rd: u32,
    s: bool,
    h: bool,
    instruction: u32,
}

impl HalfwordTransFields {
    fn parse(instruction: u32) -> Self {
        Self {
            p: instruction.bit(24) == 1,
            u: instruction.bit(23) == 1,
            w: instruction.bit(21) == 1,
            l: instruction.bit(20) == 1,
            rn: instruction.bits(16, 19),
            rd: instruction.bits(12, 15),
            s: instruction.bit(6) == 1,
            h: instruction.bit(5) == 1,
            instruction,
        }
    }

    fn offset(&self, cpu: &Cpu) -> u32 {
        if self.instruction.bit(22) == 1 {
            let offset_high = self.instruction.bits(8, 11);
            let offset_low = self.instruction.bits(0, 3);
            offset_high << 4 | offset_low
        } else {
            let rm = self.instruction.bits(0, 3);
            cpu.get_reg(rm as usize)
        }
    }

    /// Returns address and final address
    fn address_mode_2(&self, cpu: &Cpu) -> (u32, u32) {
        let offset = self.offset(cpu);
        let final_address = if self.u {
            cpu.get_reg(self.rn as usize) + offset
        } else {
            cpu.get_reg(self.rn as usize) - offset
        };

        let address = if self.p {
            final_address
        } else {
            cpu.get_reg(self.rn as usize)
        };

        (address, final_address)
    }
}

struct LDRH;
struct STRH;
struct LDRSB;
struct LDRSH;

#[inline]
fn execute_h<F>(cpu: &mut Cpu, bus: &mut Bus, instruction: u32, func: F)
where
    F: FnOnce(&mut Cpu, &mut Bus, usize, u32) -> (),
{
    let fields = HalfwordTransFields::parse(instruction);
    let (address, final_address) = fields.address_mode_2(cpu);

    if address.bit(0) == 0 {
        func(cpu, bus, fields.rd as usize, address);
        let val = bus.read_half(address, cpu);
        cpu.set_reg(fields.rd as usize, val as u32);
    } else {
        todo!("UNPREDICTABLE, LDRH address is not halfword-aligned")
    }

    if !fields.p && !fields.w {
        cpu.set_reg(fields.rn as usize, final_address);
    } else if fields.p && fields.w {
        cpu.set_reg(fields.rn as usize, final_address);
    } else if !fields.p && fields.w {
        todo!("UNPREDICTABLE, STHR P=0 and W=1")
    }
}

impl ArmInstruction for LDRH {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32) {
        execute_h(cpu, bus, instruction, |cpu, bus, rd, address| {
            let val = bus.read_half(address, cpu);
            cpu.set_reg(rd, val as u32);
        });
    }

    fn disassembly(&self, instruction: u32) -> String {
        format!("LDRH")
    }
}

impl ArmInstruction for STRH {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32) {
        execute_h(cpu, bus, instruction, |cpu, bus, rd, address| {
            bus.write_half(address, cpu.get_reg(rd) as u16);
        });
    }

    fn disassembly(&self, instruction: u32) -> String {
        format!("STRH")
    }
}

impl ArmInstruction for LDRSB {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32) {
        todo!()
    }
    fn disassembly(&self, instruction: u32) -> String {
        "LDRSB".to_string()
    }
}

impl ArmInstruction for LDRSH {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32) {
        todo!()
    }
    fn disassembly(&self, instruction: u32) -> String {
        "LDRSH".to_string()
    }
}

impl MetaInstr {
    pub(super) fn decode_halfword_transfer(instruction: u32) -> Box<dyn ArmInstruction> {
        let l = instruction.bit(20);
        let sh = instruction.bits(5, 6);

        match (l, sh) {
            (0, 0b01) => Box::new(STRH),
            (1, 0b01) => Box::new(LDRH),
            (1, 0b10) => Box::new(LDRSB),
            (1, 0b11) => Box::new(LDRSH),
            _ => Box::new(UnimplementedInstruction),
        }
    }
}
