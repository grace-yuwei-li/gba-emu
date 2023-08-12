use crate::Cpu;
use crate::Bus;
use crate::logging::Targets;
use crate::utils::AddressableBits;
use tracing::trace;

impl Cpu {
    fn bdt_stmfd(&mut self, bus: &mut Bus, instruction: u32) {
        let base_register = instruction.bits(16, 19);

        assert_ne!(base_register, 15);

        let mut dest_address = self.get_reg(base_register as usize) - 4;
        let mut reg_count = 0;

        for i in (0 ..= 15).rev() {
            if instruction.bit(i) == 1 {
                bus.set(dest_address, self.get_reg(i));
                dest_address -= 4;
                reg_count += 1;
            }
        }

        if instruction.bit(21) == 1 {
            let new_val = self.get_reg(base_register as usize) - 4 * reg_count;
            self.set_reg(base_register as usize, new_val);
        }

        trace!(target: Targets::Instr.value(), "STMFD r{}, {:b}", base_register, instruction.bits(0, 15));
    }

    pub fn block_data_transfer(&mut self, bus: &mut Bus, instruction: u32) {
        let base_register = instruction.bits(16, 19);
        log::trace!("PUSWL: {:05b} REG: {}", instruction.bits(20, 24), base_register);

        let arg_pu = instruction.bits(23, 24);
        let arg_s = instruction.bit(22);
        let arg_l = instruction.bit(20);

        if arg_s == 1{
            todo!("S bit behaviour not implemented");
        }

        match arg_pu << 1 | arg_l {
            0b100 => self.bdt_stmfd(bus, instruction),
            0b000 ..= 0b111 => todo!(),
            _ => unreachable!(),
        }
    }
}
