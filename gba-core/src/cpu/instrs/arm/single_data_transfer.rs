use crate::Cpu;
use crate::Bus;
use crate::logging::Targets;
use crate::utils::AddressableBits;
use tracing::trace;

impl Cpu {
    fn str(&mut self, bus: &mut Bus, instruction: u32) {
        let i = instruction.bit(25);
        let p = instruction.bit(24);
        let u = instruction.bit(23);
        let b = instruction.bit(22);
        let w = instruction.bit(21);

        trace!(target: Targets::Arm.value(), "STR{}", if b == 1 { "B" } else { "" });

        if b == 1 {
            todo!();
        }

        todo!("str")
    }

    fn ldr_immediate_offset(&mut self, bus: &mut Bus, instruction: u32) {

    }

    fn ldr(&mut self, bus: &mut Bus, instruction: u32) {
        let i = instruction.bit(25);
        let p = instruction.bit(24);
        let u = instruction.bit(23);
        let b = instruction.bit(22);
        let w = instruction.bit(21);

        let rn = instruction.bits(16, 19);
        let rd = instruction.bits(12, 15);
        let offset = instruction.bits(0, 12);

        trace!(target: Targets::Arm.value(), "LDR{}", if b == 1 { "B" } else { "" });
        println!("I {} P {} U {} B {} W {}", i == 1, p == 1, u == 1, b == 1, w == 1);

        if b == 1 {
            todo!();
        }

        let address;
        match (i == 1, p == 1, u == 1, b == 1, w == 1) {
            (false, true, u, false, false) => {
                address = if u {
                    self.get_reg(rn as usize) + offset
                } else {
                    self.get_reg(rn as usize) - offset
                };
            }
            _ => todo!()
        }

        self.set_reg(rd as usize, address);

        if rd == 15 {
            self.flush_pipeline();
        }
    }

    pub(super) fn single_data_transfer(&mut self, bus: &mut Bus, instruction: u32) {
        let l = instruction.bit(20);

        if l == 1 {
            self.ldr(bus, instruction);
        } else {
            self.str(bus, instruction);
        }
    }
}

