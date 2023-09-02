use crate::cpu::CPSR;
use crate::utils::AddressableBits;
use crate::Bus;
use crate::Cpu;

use super::ArmInstruction;
use super::MetaInstr;

struct STR;
struct LDR;

#[derive(PartialEq, Eq)]
pub enum AddressingModeSource {
    Immediate {
        rn: u32,
        u: bool,
        offset: u32,
    },
    // Scaled or regular - determined by shift and shift_imm
    Register {
        rn: u32,
        u: bool,
        rm: u32,
        shift: u32,
        shift_imm: u32,
    },
}

impl AddressingModeSource {
    fn rn(&self) -> u32 {
        match *self {
            Self::Immediate {
                rn,
                u: _,
                offset: _,
            } => rn,
            Self::Register {
                rn,
                u: _,
                rm: _,
                shift: _,
                shift_imm: _,
            } => rn,
        }
    }
    fn u(&self) -> bool {
        match *self {
            Self::Immediate {
                rn: _,
                u,
                offset: _,
            } => u,
            Self::Register {
                rn: _,
                u,
                rm: _,
                shift: _,
                shift_imm: _,
            } => u,
        }
    }
    pub fn offset(&self, cpu: &Cpu) -> u32 {
        match *self {
            Self::Immediate {
                rn: _,
                u: _,
                offset,
            } => offset,
            Self::Register {
                rn: _,
                u: _,
                rm,
                shift,
                shift_imm,
            } => {
                let rm = cpu.get_reg(rm);
                match shift {
                    0b00 => rm << shift_imm,
                    0b01 if shift_imm == 0 => 0,
                    0b01 => rm >> shift_imm,
                    0b10 if shift_imm == 0 => {
                        if rm.bit(31) == 0 {
                            0
                        } else {
                            0xffffffff
                        }
                    }
                    0b10 => ((rm as i32) >> shift_imm) as u32,
                    0b11 if shift_imm == 0 => cpu.get_cpsr_bits(CPSR::C) << 31 | rm >> 1,
                    0b11 => rm.rotate_right(shift_imm),
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum AddressingModeIndexing {
    Offset,
    PreIndexed,
    PostIndexed,
}

pub struct AddressingMode {
    source: AddressingModeSource,
    indexing: AddressingModeIndexing,
}

pub struct Address {
    pub address: u32,
    pub write_back: Option<u32>,
}

impl AddressingMode {
    pub fn decode(instruction: u32) -> Self {
        let i = instruction.bit(25) == 1;
        let p = instruction.bit(24) == 1;
        let u = instruction.bit(23) == 1;
        let w = instruction.bit(21) == 1;
        let rn = instruction.bits(16, 19);

        let source = match i {
            false => AddressingModeSource::Immediate {
                u,
                rn,
                offset: instruction.bits(0, 11),
            },
            true => AddressingModeSource::Register {
                rn,
                u,
                rm: instruction.bits(0, 3),
                shift: instruction.bits(5, 6),
                shift_imm: instruction.bits(7, 11),
            },
        };

        let indexing = match (p, w) {
            (true, false) => AddressingModeIndexing::Offset,
            (true, true) => AddressingModeIndexing::PreIndexed,
            (false, false) => AddressingModeIndexing::PostIndexed,
            // Not sure what happens if p false and w true, none of
            // the 9 addressing modes seem to correspond to this case.
            (false, true) => AddressingModeIndexing::PostIndexed,
        };

        Self { source, indexing }
    }

    pub fn decode_halfword(instruction: u32) -> Self {
        let p = instruction.bit(24) == 1;
        let u = instruction.bit(23) == 1;
        let i = instruction.bit(22) == 1;
        let w = instruction.bit(21) == 1;
        let rn = instruction.bits(16, 19);

        let source = if i {
            AddressingModeSource::Immediate {
                u,
                rn,
                offset: instruction.bits(0, 3) | (instruction.bits(8, 11) << 4),
            }
        } else {
            AddressingModeSource::Register {
                rn,
                u,
                rm: instruction.bits(0, 3),
                shift: 0,
                shift_imm: 0,
            }
        };

        let indexing = match (p, w) {
            (true, false) => AddressingModeIndexing::Offset,
            (true, true) => AddressingModeIndexing::PreIndexed,
            (false, false) => AddressingModeIndexing::PostIndexed,
            // Not sure what happens if p false and w true, none of
            // the 9 addressing modes seem to correspond to this case.
            (false, true) => AddressingModeIndexing::PostIndexed,
        };

        Self { source, indexing }
    }

    /// Returns address and optionally the write-back address
    pub fn address(&self, cpu: &Cpu) -> Address {
        match self.indexing {
            AddressingModeIndexing::Offset => {
                if self.source.u() {
                    let address = cpu
                        .get_reg(self.source.rn())
                        .wrapping_add(self.source.offset(cpu));
                    Address {
                        address,
                        write_back: None,
                    }
                } else {
                    let address = cpu
                        .get_reg(self.source.rn())
                        .wrapping_sub(self.source.offset(cpu));
                    Address {
                        address,
                        write_back: None,
                    }
                }
            }
            AddressingModeIndexing::PreIndexed => {
                if self.source.u() {
                    let address = cpu
                        .get_reg(self.source.rn())
                        .wrapping_add(self.source.offset(cpu));
                    Address {
                        address,
                        write_back: Some(address),
                    }
                } else {
                    let address = cpu
                        .get_reg(self.source.rn())
                        .wrapping_sub(self.source.offset(cpu));
                    Address {
                        address,
                        write_back: Some(address),
                    }
                }
            }
            AddressingModeIndexing::PostIndexed => {
                let address = cpu.get_reg(self.source.rn());
                if self.source.u() {
                    Address {
                        address,
                        write_back: Some(address.wrapping_add(self.source.offset(cpu))),
                    }
                } else {
                    Address {
                        address,
                        write_back: Some(address.wrapping_sub(self.source.offset(cpu))),
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for AddressingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = if self.source.u() { "+" } else { "-" };

        let filler = match self.source {
            AddressingModeSource::Immediate {
                rn: _,
                u: _,
                offset,
            } => {
                format!("#{}{:x}", sign, offset)
            }
            AddressingModeSource::Register {
                rn: _,
                u: _,
                rm,
                shift,
                shift_imm,
            } if shift == 0 && shift_imm == 0 => {
                format!("{}r{}", sign, rm)
            }
            AddressingModeSource::Register {
                rn: _,
                u: _,
                rm,
                shift,
                shift_imm,
            } if shift == 0b11 && shift_imm == 0 => {
                format!("{}r{}, RRX", sign, rm)
            }
            AddressingModeSource::Register {
                rn: _,
                u: _,
                rm,
                shift,
                shift_imm,
            } => {
                let shift = match shift {
                    0b00 => "LSL",
                    0b01 => "LSR",
                    0b10 => "ASR",
                    0b11 => "ROR",
                    _ => unreachable!(),
                };
                format!("{}r{}, {}, #{}", sign, rm, shift, shift_imm)
            }
        };

        match self.indexing {
            AddressingModeIndexing::Offset => write!(f, "[r{}, {}]", self.source.rn(), filler),
            AddressingModeIndexing::PreIndexed => write!(f, "[r{}, {}]!", self.source.rn(), filler),
            AddressingModeIndexing::PostIndexed => write!(f, "[r{}], {}", self.source.rn(), filler),
        }
    }
}

impl ArmInstruction for STR {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32) {
        let b = instruction.bit(22);
        let rn = instruction.bits(16, 19);
        let rd = instruction.bits(12, 15);
        let rd = if rd == 15 {
            cpu.get_reg(rd) + 4
        } else {
            cpu.get_reg(rd)
        };
        let addressing_mode = AddressingMode::decode(instruction);

        let Address {
            address,
            write_back,
        } = addressing_mode.address(cpu);

        if b == 0 {
            bus.write(address, rd);
        } else {
            bus.write_byte(address, rd as u8);
        }

        if let Some(address) = write_back {
            cpu.set_reg(rn, address);
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        let b = instruction.bit(22);
        let rd = instruction.bits(12, 15);
        let addressing_mode = AddressingMode::decode(instruction);
        format!(
            "STR{} r{} {}",
            if b == 1 { "B" } else { "" },
            rd,
            addressing_mode
        )
    }
}

impl ArmInstruction for LDR {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32) {
        let b = instruction.bit(22);
        let rn = instruction.bits(16, 19);
        let rd = instruction.bits(12, 15);
        let addressing_mode = AddressingMode::decode(instruction);

        let Address {
            address,
            write_back,
        } = addressing_mode.address(cpu);

        let val = if b == 0 {
            bus.read(address, cpu)
        } else {
            bus.read_byte(address, cpu) as u32
        };

        cpu.set_reg(rd, val);
        if let Some(address) = write_back {
            // Don't write back if rd == rn
            if rd != rn {
                cpu.set_reg(rn, address);
            }
        }

        if rd == 15 {
            cpu.flush_pipeline();
        }
    }

    fn disassembly(&self, instruction: u32) -> String {
        let b = instruction.bit(22);
        let rd = instruction.bits(12, 15);
        let addressing_mode = AddressingMode::decode(instruction);
        format!(
            "LDR{} r{} {}",
            if b == 1 { "B" } else { "" },
            rd,
            addressing_mode
        )
    }
}

impl MetaInstr {
    pub(super) fn decode_single_data_transfer(instruction: u32) -> Box<dyn ArmInstruction> {
        let l = instruction.bit(20);

        if l == 1 {
            Box::new(LDR)
        } else {
            Box::new(STR)
        }
    }
}
