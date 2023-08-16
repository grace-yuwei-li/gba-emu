use crate::Cpu;
use crate::Bus;
use crate::logging::Targets;
use tracing::trace;

impl Cpu {
    pub(super) fn branch(&mut self, _bus: &mut Bus, instruction: u32) {
        let link = (instruction >> 24) & 1 != 0;
        let offset = instruction & 0xffffff;
        let offset = ((offset << 8) as i32) >> 6;

        if link {
            self.set_reg(14, self.get_reg(15) - 4);
        }

        let dest = self.get_reg(15).wrapping_add_signed(offset);
        trace!(target: Targets::Arm.value(), "B{} {:x}", if link { "L" } else { "" }, dest);

        self.set_reg(15, dest);
        self.flush_pipeline();
        
    }
}
