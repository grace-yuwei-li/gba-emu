use crate::cpu::Mode;
use crate::utils::reg_list;
use crate::utils::AddressableBits;
use crate::Bus;
use crate::Cpu;
use tracing::error;

use super::ArmInstruction;
use super::MetaInstr;

struct Ldm(AddressingMode);
struct Stm(AddressingMode);

impl ArmInstruction for Ldm {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32) {
        let (start_address, write_back) = self.0.address(instruction, cpu);

        let rn = instruction.bits(16, 19);
        let s = instruction.bit(22);
        let mode = if s == 1 { Mode::User } else { cpu.get_mode() };

        if rn == 15 {
            error!("UNPREDICTABLE");
        }

        if instruction.bits(0, 15) != 0 {
            let mut address = start_address;
            // Normal case
            for i in 0..=14 {
                if instruction.bit(i) == 1 {
                    let value = bus.read(address & 0xfffffffc, cpu);
                    *cpu.regs.get_mut(i.try_into().unwrap(), &mode) = value;
                    address += 4;
                }
            }

            if instruction.bit(15) == 1 {
                // Write to PC
                let value = bus.read(address, cpu);
                cpu.set_reg(15, value & 0xffff_fffc);
                cpu.flush_pipeline();
            }

            if let Some(address) = write_back {
                // Don't writeback if the base register is in the reg list
                if instruction.bit(rn.try_into().unwrap()) == 0 {
                    cpu.set_reg(rn, address);
                }
            }
        } else {
            // Empty register list loads value from memory into PC
            let value = bus.read(start_address, cpu);
            cpu.set_reg(15, value & 0xffff_fffc);
            cpu.flush_pipeline();
            if self.0.is_increment() {
                cpu.set_reg(rn, cpu.get_reg(rn) + 0x40);
            } else {
                cpu.set_reg(rn, cpu.get_reg(rn) - 0x40);
            }
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        let s = if instruction.bit(22) == 1 { "^" } else { "" };
        let w = if instruction.bit(21) == 1 { "!" } else { "" };
        let rn = instruction.bits(16, 19);
        format!(
            "LDM{} r{}{}, {{{}}}{}",
            self.0,
            rn,
            w,
            reg_list(instruction, 16),
            s
        )
    }
}

impl ArmInstruction for Stm {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32) {
        let (start_address, write_back) = self.0.address(instruction, cpu);
        let rn = instruction.bits(16, 19);

        let s = instruction.bit(22);
        let mode = if s == 1 { Mode::User } else { cpu.get_mode() };

        if rn == 15 {
            error!("UNPREDICTABLE");
        }

        if instruction.bits(0, 15) != 0 {
            // Normal case
            let mut address = start_address;
            let mut first_reg = true;

            for i in 0..=14 {
                if instruction.bit(i) == 1 {
                    bus.write(address, cpu.regs.get(i.try_into().unwrap(), &mode));
                    address += 4;

                    // Write back in STM happens after first register
                    if first_reg {
                        first_reg = false;
                        if let Some(address) = write_back {
                            cpu.set_reg(rn, address);
                        }
                    }
                }
            }

            if instruction.bit(15) == 1 {
                bus.write(address, cpu.get_reg(15) + 4);
            }
        } else {
            // Empty register list stores PC
            let write_back = if self.0.is_increment() {
                cpu.get_reg(rn) + 0x40
            } else {
                cpu.get_reg(rn) - 0x40
            };
            match self.0 {
                AddressingMode::IncrementAfter | AddressingMode::IncrementBefore => {
                    bus.write(start_address, cpu.get_reg(15) + 4);
                    cpu.set_reg(rn, write_back);
                }
                AddressingMode::DecrementAfter => {
                    bus.write(write_back + 4, cpu.get_reg(15) + 4);
                    cpu.set_reg(rn, write_back);
                }
                AddressingMode::DecrementBefore => {
                    bus.write(write_back, cpu.get_reg(15) + 4);
                    cpu.set_reg(rn, write_back);
                }
            }
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        let s = if instruction.bit(22) == 1 { "^" } else { "" };
        let w = if instruction.bit(21) == 1 { "!" } else { "" };
        let rn = instruction.bits(16, 19);
        format!(
            "STM{} r{}{}, {{{}}}{}",
            self.0,
            rn,
            w,
            reg_list(instruction, 16),
            s
        )
    }
}

enum AddressingMode {
    IncrementAfter,
    IncrementBefore,
    DecrementAfter,
    DecrementBefore,
}

impl AddressingMode {
    /// Returns address and an optional write-back address
    fn address(&self, instruction: u32, cpu: &Cpu) -> (u32, Option<u32>) {
        let rn = cpu.get_reg(instruction.bits(16, 19));
        let reg_count = instruction.bits(0, 15).count_ones();
        let (address, write_back) = match *self {
            Self::IncrementAfter => (rn, rn + reg_count * 4),
            Self::IncrementBefore => (rn + 4, rn + reg_count * 4),
            Self::DecrementAfter => (rn - reg_count * 4 + 4, rn - reg_count * 4),
            Self::DecrementBefore => (rn - reg_count * 4, rn - reg_count * 4),
        };

        // Check w bit
        if instruction.bit(21) == 0 {
            (address, None)
        } else {
            (address, Some(write_back))
        }
    }

    fn is_increment(&self) -> bool {
        match *self {
            Self::IncrementAfter | Self::IncrementBefore => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for AddressingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match *self {
            Self::IncrementAfter => "IA",
            Self::IncrementBefore => "IB",
            Self::DecrementAfter => "DA",
            Self::DecrementBefore => "DB",
        };
        write!(f, "{}", str)
    }
}

impl MetaInstr {
    pub(super) fn decode_block_data_transfer(instruction: u32) -> Box<dyn ArmInstruction> {
        let p = instruction.bit(24) == 1;
        let u = instruction.bit(23) == 1;
        let l = instruction.bit(20) == 1;

        let addressing_mode = match (p, u) {
            (false, true) => AddressingMode::IncrementAfter,
            (true, true) => AddressingMode::IncrementBefore,
            (false, false) => AddressingMode::DecrementAfter,
            (true, false) => AddressingMode::DecrementBefore,
        };

        if l {
            Box::new(Ldm(addressing_mode))
        } else {
            Box::new(Stm(addressing_mode))
        }
    }
}
