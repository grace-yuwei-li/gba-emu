use crate::Cpu;
use crate::Bus;
use crate::logging::Targets;
use crate::utils::AddressableBits;
use tracing::error;
use tracing::trace;

struct HalfwordTransImmFields {
    p: bool,
    u: bool,
    w: bool,
    l: bool,
    rn: u32,
    rd: u32,
    offset: u32,
    s: bool,
    h: bool,
}

impl HalfwordTransImmFields {
    fn parse(instruction: u32) -> Self {
        let offset_high = instruction.bits(8, 11);
        let offset_low = instruction.bits(0, 3);
        let offset = offset_high << 4 | offset_low;

        Self {
            p: instruction.bit(24) == 1,
            u: instruction.bit(23) == 1,
            w: instruction.bit(21) == 1,
            l: instruction.bit(20) == 1,
            rn: instruction.bits(16, 19),
            rd: instruction.bits(12, 15),
            offset,
            s: instruction.bit(6) == 1,
            h: instruction.bit(5) == 1,
        }
    }
}

impl Cpu {
    fn ldrh(&mut self, bus: &mut Bus, instruction: u32) {
        trace!(target: Targets::Instr.value(), "LDRH");
        todo!()
    }

    /// Store halfword
    fn strh(&mut self, bus: &mut Bus, instruction: u32) {
        let fields = HalfwordTransImmFields::parse(instruction);

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

        trace!(target: Targets::Instr.value(), "STRH");
    }

    fn ldrsb(&mut self, bus: &mut Bus, instruction: u32) {
        trace!(target: Targets::Instr.value(), "LDRSB");
        todo!()
    }

    fn ldrsh(&mut self, bus: &mut Bus, instruction: u32) {
        trace!(target: Targets::Instr.value(), "LDRSH");
        todo!()
    }
   
    pub fn halfword_transfer_immediate(&mut self, bus: &mut Bus, instruction: u32) {
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
