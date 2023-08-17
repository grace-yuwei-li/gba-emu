use crate::Cpu;
use crate::Bus;
use crate::cpu::State;
use crate::logging::Targets;
use crate::utils::AddressableBits;
use tracing::trace;

impl Cpu {
    pub(super) fn branch_and_exchange(&mut self, _bus: &mut Bus, instruction: u32) {
        let rn = instruction.bits(0, 3);

        trace!(target: Targets::Arm.value(), "BX r{}", rn);

        if rn == 15 {
            todo!("undefined behaviour");
        }

        let dest = self.get_reg(rn as usize);

        if dest.bit(0) == 1 {
            self.set_state(State::Thumb);
        }

        self.set_reg(15, dest & 0xffff_fffe);

        self.flush_pipeline();
    }
}
