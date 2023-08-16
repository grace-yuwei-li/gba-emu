use tracing::{trace, error};

use crate::{cpu::Cpu, bus::Bus, utils::AddressableBits, logging::Targets};

struct HalfwordTransFields {
    p: bool,
    u: bool,
    w: bool,
    l: bool,
    rn: u32,
    rd: u32,
    s: bool,
    h: bool,
    offset: u32,
}

impl HalfwordTransFields {
    fn parse(instruction: u32, cpu: &Cpu) -> Self {
        let offset = if instruction.bit(22) == 1 {
            let offset_high = instruction.bits(8, 11);
            let offset_low = instruction.bits(0, 3);
            offset_high << 4 | offset_low
        } else {
            let rm = instruction.bits(0, 3);
            cpu.get_reg(rm as usize)
        };

        Self {
            p: instruction.bit(24) == 1,
            u: instruction.bit(23) == 1,
            w: instruction.bit(21) == 1,
            l: instruction.bit(20) == 1,
            rn: instruction.bits(16, 19),
            rd: instruction.bits(12, 15),
            s: instruction.bit(6) == 1,
            h: instruction.bit(5) == 1,
            offset,
        }
    }
}

impl Cpu {
    fn addressing_mode_2(&self, fields: &HalfwordTransFields) -> (u32, u32) {
        let final_address = if fields.u {
            self.get_reg(fields.rn as usize) + fields.offset
        } else {
            self.get_reg(fields.rn as usize) - fields.offset
        };

        let address = if fields.p {
            final_address
        } else {
            self.get_reg(fields.rn as usize)
        };

        (address, final_address)
    }

    fn ldrh(&mut self, bus: &mut Bus, instruction: u32) {
        let fields = HalfwordTransFields::parse(instruction, self);

        trace!(target: Targets::Arm.value(), "LDRH");

        let (address, final_address) = self.addressing_mode_2(&fields);

        if address.bit(0) == 0 {
            let val = bus.get_half(address);
            self.set_reg(fields.rd as usize, val as u32);
        } else {
            error!("UNPREDICTABLE, LDRH address is not halfword-aligned")
        }

        if !fields.p && !fields.w {
            self.set_reg(fields.rn as usize, final_address);
        } else if fields.p && fields.w {
            self.set_reg(fields.rn as usize, final_address);
        } else if !fields.p && fields.w {
            error!("UNPREDICTABLE, STHR P=0 and W=1")
        }
    }

    /// Store halfword
    fn strh(&mut self, bus: &mut Bus, instruction: u32) {
        let fields = HalfwordTransFields::parse(instruction, self);

        trace!(target: Targets::Arm.value(), "STRH");

        let (address, final_address) = self.addressing_mode_2(&fields);

        if address.bit(0) == 0 {
            bus.set_half(address, self.get_reg(fields.rd as usize) as u16);
        } else {
            error!("UNPREDICTABLE, STRH address is not halfword-aligned")
        }


        if !fields.p && !fields.w {
            self.set_reg(fields.rn as usize, final_address);
        } else if fields.p && fields.w {
            self.set_reg(fields.rn as usize, final_address);
        } else if !fields.p && fields.w {
            error!("UNPREDICTABLE, STHR P=0 and W=1")
        }

    }

    fn ldrsb(&mut self, bus: &mut Bus, instruction: u32) {
        trace!(target: Targets::Arm.value(), "LDRSB");
        todo!()
    }

    fn ldrsh(&mut self, bus: &mut Bus, instruction: u32) {
        trace!(target: Targets::Arm.value(), "LDRSH");
        todo!()
    }
   
    pub(super) fn halfword_transfer(&mut self, bus: &mut Bus, instruction: u32) {
        let l = instruction.bit(20);
        let sh = instruction.bits(5, 6);

        match (l, sh) {
            (0, 0b01) => self.strh(bus, instruction),
            (1, 0b01) => self.ldrh(bus, instruction),
            (1, 0b10) => self.ldrsb(bus, instruction),
            (1, 0b11) => self.ldrsh(bus, instruction),
            _ => unreachable!(),
        }
    }
}
