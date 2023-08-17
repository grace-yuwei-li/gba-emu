use crate::bus::Bus;

use super::{Cpu, State};

mod thumb;
mod arm;

impl Cpu {
    pub fn execute(&mut self, bus: &mut Bus, instruction: u32) {
        match self.get_state() {
            State::ARM => {
                if !self.check_cond(instruction) {
                    log::trace!("Cond check failed for instruction {:#034b}", instruction);
                    return;
                }

                log::trace!("Executing ARM instruction {:08x}", instruction);
                let fp = self.decode_arm(instruction);
                fp(self, bus, instruction)
            },
            State::Thumb => {
                log::trace!("Executing THUMB instruction {:04x}", instruction as u16);
                let fp = self.decode_thumb(instruction as u16);
                fp(self, bus, instruction as u16)
            }
        }
    }
}
