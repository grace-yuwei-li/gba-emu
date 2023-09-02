use crate::{bus::Bus, cpu::Cpu, utils::AddressableBits};

use super::{ArmInstruction, MetaInstr, UnimplementedInstruction, single_data_transfer::{AddressingMode, Address}};

struct LDRH;
struct STRH;
struct LDRSB;
struct LDRSH;

impl ArmInstruction for LDRH {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32) {
        let rn = instruction.bits(16, 19);
        let rd = instruction.bits(12, 15);
        let Address { address, write_back } = AddressingMode::decode_halfword(instruction).address(cpu);

        let val = bus.read_half(address, cpu);
        cpu.set_reg(rd, val);

        if let Some(address) = write_back {
            if rd != rn {
                cpu.set_reg(rn, address);
            }
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        let rd = instruction.bits(12, 15);
        let addressing_mode = AddressingMode::decode_halfword(instruction);
        format!("LDRH r{}, {}", rd, addressing_mode)
    }
}

impl ArmInstruction for STRH {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32) {
        let rn = instruction.bits(16, 19);
        let rd = instruction.bits(12, 15);
        let Address { address, write_back } = AddressingMode::decode_halfword(instruction).address(cpu);

        bus.write_half(address, cpu.get_reg(rd) as u16);

        if let Some(address) = write_back {
            cpu.set_reg(rn, address);
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        let rd = instruction.bits(12, 15);
        let addressing_mode = AddressingMode::decode_halfword(instruction);
        format!("STRH r{}, {}", rd, addressing_mode)
    }
}

impl ArmInstruction for LDRSB {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32) {
        let rn = instruction.bits(16, 19);
        let rd = instruction.bits(12, 15);
        let Address { address, write_back } = AddressingMode::decode_halfword(instruction).address(cpu);

        let val = bus.read_byte(address, cpu);
        cpu.set_reg(rd, i32::from(val as i8) as u32);

        if let Some(address) = write_back {
            cpu.set_reg(rn, address);
        }
    }
    fn disassembly(&self, instruction: u32) -> String {
        let rd = instruction.bits(12, 15);
        let addressing_mode = AddressingMode::decode_halfword(instruction);
        format!("LDRSB r{}, {}", rd, addressing_mode)
    }
}

impl ArmInstruction for LDRSH {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32) {
        let rn = instruction.bits(16, 19);
        let rd = instruction.bits(12, 15);
        let Address { address, write_back } = AddressingMode::decode_halfword(instruction).address(cpu);

        // LDRSH has weird misaligned behaviour - it reads the value at the address as a byte and sign
        // extends to 32 bits
        let val = if address.bit(0) == 0 {
            bus.read_signed_half(address, cpu)
        } else {
            i32::from(bus.read_byte(address, cpu) as i8) as u32
        };
        cpu.set_reg(rd, val);

        if let Some(address) = write_back {
            cpu.set_reg(rn, address);
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        let rd = instruction.bits(12, 15);
        let addressing_mode = AddressingMode::decode_halfword(instruction);
        format!("LDRSH r{}, {}", rd, addressing_mode)
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
