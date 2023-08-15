use crate::Cpu;
use crate::Bus;
use crate::cpu::State;
use crate::logging::Targets;
use crate::utils::AddressableBits;
use tracing::trace;
use tracing::error;

impl Cpu {
    fn bdt_ldmfd(&mut self, bus: &mut Bus, instruction: u32) {
        let base_register = instruction.bits(16, 19);

        let start_address = self.get_reg(base_register as usize);
        let mut address = start_address;
        let mut reg_count = 0;

        trace!(target: Targets::Instr.value(), "LDMFD r{}, {:b}", base_register, instruction.bits(0, 15));

        if base_register == 15 {
            error!("UNPREDICTABLE");
        }

        for i in 0 ..= 14 {
            if instruction.bit(i) == 1 {
                reg_count += 1;
                let value = bus.get(address);
                self.set_reg(i, value);
                address += 4;
            }
        }

        if instruction.bit(15) == 1 {
            reg_count += 1;

            // Write to PC
            let value = bus.get(address);
            self.set_reg(15, value & 0xffff_fffe);

            self.state = if value & 1 == 0 {
                State::ARM
            } else {
                State::Thumb
            };

            // Flush after a write
            self.flush_pipeline();

            trace!("Set PC to {:x}", self.get_reg(15));
        }

        if instruction.bit(21) == 1 {
            let new_val = self.get_reg(base_register as usize) + 4 * reg_count;
            self.set_reg(base_register as usize, new_val);
        }

    }

    fn bdt_stmfd(&mut self, bus: &mut Bus, instruction: u32) {
        let base_register = instruction.bits(16, 19);

        let mut dest_address = self.get_reg(base_register as usize) - 4;
        let mut reg_count = 0;

        trace!(target: Targets::Instr.value(), "STMFD r{}, {:b}", base_register, instruction.bits(0, 15));

        if base_register == 15 {
            error!("UNPREDICTABLE");
        }

        if instruction.bit(15) == 1 {
            reg_count += 1;
            todo!("IMPLEMENTATION DEFINED");
        }

        for i in (0 ..= 14).rev() {
            if instruction.bit(i) == 1 {
                reg_count += 1;
                bus.set(dest_address, self.get_reg(i));
                dest_address -= 4;
            }
        }

        if instruction.bit(21) == 1 {
            let new_val = self.get_reg(base_register as usize) - 4 * reg_count;
            self.set_reg(base_register as usize, new_val);
        }
    }

    pub(super) fn block_data_transfer(&mut self, bus: &mut Bus, instruction: u32) {
        let base_register = instruction.bits(16, 19);
        log::trace!("PUSWL: {:05b} REG: {}", instruction.bits(20, 24), base_register);

        if instruction.bit(22) == 1 {
            todo!("user mode");
        }

        let arg_pu = instruction.bits(23, 24);
        let arg_s = instruction.bit(22);
        let arg_l = instruction.bit(20);

        if arg_s == 1{
            todo!("S bit behaviour not implemented");
        }

        match arg_pu << 1 | arg_l {
            0b011 => self.bdt_ldmfd(bus, instruction),
            0b100 => self.bdt_stmfd(bus, instruction),
            0b000 ..= 0b111 => todo!("{:03b}", arg_pu << 1 | arg_l),
            _ => unreachable!(),
        }
    }
}
