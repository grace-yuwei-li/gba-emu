use crate::cpu::State;
use crate::cpu::instrs::arm::TodoInstruction;
use crate::utils::AddressableBits;
use crate::Bus;
use crate::Cpu;
use tracing::error;
use tracing::trace;

use super::ArmInstruction;
use super::MetaInstr;

struct LDMFD;
struct STMFD;

impl ArmInstruction for LDMFD {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32) {
        let base_register = instruction.bits(16, 19);

        let start_address = cpu.get_reg(base_register as usize);
        let mut address = start_address;
        let mut reg_count = 0;

        if base_register == 15 {
            error!("UNPREDICTABLE");
        }

        for i in 0..=14 {
            if instruction.bit(i) == 1 {
                reg_count += 1;
                let value = bus.read(address);
                cpu.set_reg(i, value);
                address += 4;
            }
        }

        if instruction.bit(15) == 1 {
            reg_count += 1;

            // Write to PC
            let value = bus.read(address);
            cpu.set_reg(15, value & 0xffff_fffe);

            let state = if value & 1 == 0 {
                State::ARM
            } else {
                State::Thumb
            };
            cpu.set_state(state);

            // Flush after a write
            cpu.flush_pipeline();

            trace!("Set PC to {:x}", cpu.get_reg(15));
        }

        if instruction.bit(21) == 1 {
            let new_val = cpu.get_reg(base_register as usize) + 4 * reg_count;
            cpu.set_reg(base_register as usize, new_val);
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        let base_register = instruction.bits(16, 19);
        format!("LDMFD r{}, {:b}", base_register, instruction.bits(0, 15))
    }
}

impl ArmInstruction for STMFD {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32) {
        let base_register = instruction.bits(16, 19);

        let mut dest_address = cpu.get_reg(base_register as usize) - 4;
        let mut reg_count = 0;

        if base_register == 15 {
            error!("UNPREDICTABLE");
        }

        if instruction.bit(15) == 1 {
            reg_count += 1;
            todo!("IMPLEMENTATION DEFINED");
        }

        for i in (0..=14).rev() {
            if instruction.bit(i) == 1 {
                reg_count += 1;
                bus.write(dest_address, cpu.get_reg(i));
                dest_address -= 4;
            }
        }

        if instruction.bit(21) == 1 {
            let new_val = cpu.get_reg(base_register as usize) - 4 * reg_count;
            cpu.set_reg(base_register as usize, new_val);
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        let base_register = instruction.bits(16, 19);
        format!("STMFD r{}, {:b}", base_register, instruction.bits(0, 15))
    }
}

impl MetaInstr {
    pub(super) fn decode_block_data_transfer(instruction: u32) -> Box<dyn ArmInstruction> {
        let base_register = instruction.bits(16, 19);
        log::trace!(
            "PUSWL: {:05b} REG: {}",
            instruction.bits(20, 24),
            base_register
        );

        if instruction.bit(22) == 1 {
            todo!("user mode");
        }

        let arg_pu = instruction.bits(23, 24);
        let arg_s = instruction.bit(22);
        let arg_l = instruction.bit(20);

        if arg_s == 1 {
            todo!("S bit behaviour not implemented");
        }

        match arg_pu << 1 | arg_l {
            0b011 => Box::new(LDMFD),
            0b100 => Box::new(STMFD),
            0b000..=0b111 => Box::new(TodoInstruction::new_message(format!("{:03b}", arg_pu << 1 | arg_l))),
            _ => unreachable!(),
        }
    }
}
