use crate::cpu::CPSR;
use crate::utils::AddressableBits;
use crate::Bus;
use crate::Cpu;
use std::fmt::Debug;

use super::ArmInstruction;

#[derive(Clone, Copy)]
enum ShiftSource {
    Immediate(u32),
    Register(u32),
}

impl ShiftSource {
    fn get_amt(&self, cpu: &Cpu) -> u32 {
        match *self {
            Self::Immediate(imm) => imm,
            Self::Register(reg) => cpu.get_reg(reg as usize).bits(0, 7),
        }
    }
}
impl Debug for ShiftSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ShiftSource::Immediate(x) => write!(f, "{:x}", x),
            ShiftSource::Register(x) => write!(f, "r{}", x),
        }
    }
}

#[derive(Clone, Copy)]
enum ShifterOperand {
    Imm { imm: u32, c: Option<bool> },
    LSL { rm: u32, shift_source: ShiftSource },
    LSR { rm: u32, shift_source: ShiftSource },
    ASR { rm: u32, shift_source: ShiftSource },
    ROR { rm: u32, shift_source: ShiftSource },
    RRX { rm: u32 },
}

impl Debug for ShifterOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Imm { imm, c } => write!(f, "#{:x}", imm),
            Self::LSL { rm, shift_source } => match shift_source {
                ShiftSource::Immediate(0) => write!(f, "r{}", rm),
                ShiftSource::Immediate(imm) => write!(f, "r{}, LSL #{:x}", rm, imm),
                ShiftSource::Register(reg) => write!(f, "r{}, LSL r{}", rm, reg),
            },
            Self::LSR { rm, shift_source } => match shift_source {
                ShiftSource::Immediate(imm) => write!(f, "r{}, LSR #{:x}", rm, imm),
                ShiftSource::Register(reg) => write!(f, "r{}, LSR r{}", rm, reg),
            },
            Self::ASR { rm, shift_source } => match shift_source {
                ShiftSource::Immediate(imm) => write!(f, "r{}, ASR #{:x}", rm, imm),
                ShiftSource::Register(reg) => write!(f, "r{}, ASR r{}", rm, reg),
            },
            Self::ROR { rm, shift_source } => match shift_source {
                ShiftSource::Immediate(imm) => write!(f, "r{}, ROR #{:x}", rm, imm),
                ShiftSource::Register(reg) => write!(f, "r{}, ROR r{}", rm, reg),
            },
            Self::RRX { rm } => write!(f, "r{}, RRX", rm),
        }
    }
}

impl ShifterOperand {
    fn parse_immediate(instruction: u32) -> ShifterOperand {
        let imm = instruction.bits(0, 7);
        let rot = instruction.bits(8, 11);
        let value = imm.rotate_right(2 * rot);

        ShifterOperand::Imm {
            imm: value,
            c: if rot == 0 {
                None
            } else {
                Some(value.bit(31) == 1)
            },
        }
    }

    fn parse_shift(instruction: u32) -> ShifterOperand {
        let low_bit = (instruction >> 4) & 1;
        let shift_type_bits = (instruction >> 5) & 0b11;
        let rm = instruction & 0xf;

        let shift_source;
        if low_bit == 0 {
            // Read shift_amt from immediate field
            let shift_amt = instruction.bits(7, 11);
            shift_source = ShiftSource::Immediate(shift_amt);
        } else {
            // Read shift_amt from bottom byte of register
            let shift_reg = instruction.bits(8, 11);

            // TODO: Is this check necessary - maybe the format of instructions prevents this
            if shift_reg == 15 {
                panic!("shift reg cannot be r15")
            }

            shift_source = ShiftSource::Register(shift_reg);
        };

        match shift_type_bits {
            0b00 => Self::LSL { rm, shift_source },
            0b01 => Self::LSR { rm, shift_source },
            0b10 => Self::ASR { rm, shift_source },
            0b11 => {
                if let ShiftSource::Immediate(0) = shift_source {
                    Self::RRX { rm }
                } else {
                    Self::ROR { rm, shift_source }
                }
            }
            _ => unreachable!(),
        }
    }

    fn parse(instruction: u32) -> ShifterOperand {
        let is_immediate = instruction.bit(25) == 1;
        if is_immediate {
            Self::parse_immediate(instruction)
        } else {
            Self::parse_shift(instruction)
        }
    }

    /// Returns the value of op2 as well as the new carry flag
    fn op2(&self, cpu: &Cpu) -> (u32, bool) {
        match *self {
            Self::Imm { imm, c } => (
                imm,
                match c {
                    None => cpu.get_cpsr_bits(CPSR::C) == 1,
                    Some(b) => b,
                },
            ),
            Self::LSL { rm, shift_source } => {
                let shift_amt = shift_source.get_amt(cpu);
                if shift_amt == 0 {
                    (cpu.get_reg(rm as usize), cpu.get_cpsr_bits(CPSR::C) == 1)
                } else {
                    (
                        cpu.get_reg(rm as usize) << shift_amt,
                        cpu.get_reg(rm as usize).bit(32 - shift_amt as usize) == 1,
                    )
                }
            }
            Self::LSR { rm, shift_source } => {
                let shift_amt = shift_source.get_amt(cpu);
                if shift_amt == 0 {
                    (0, cpu.get_reg(rm as usize).bit(31) == 1)
                } else {
                    (
                        cpu.get_reg(rm as usize) >> shift_amt,
                        cpu.get_reg(rm as usize).bit(shift_amt as usize - 1) == 1,
                    )
                }
            }
            Self::ASR { rm, shift_source } => {
                let shift_amt = shift_source.get_amt(cpu);
                if shift_amt == 0 {
                    (
                        ((cpu.get_reg(rm as usize) as i32) >> 31) as u32,
                        cpu.get_reg(rm as usize).bit(31) == 1,
                    )
                } else {
                    (
                        ((cpu.get_reg(rm as usize) as i32) >> shift_amt) as u32,
                        cpu.get_reg(rm as usize).bit(shift_amt as usize - 1) == 1,
                    )
                }
            }
            Self::ROR { rm, shift_source } => {
                let shift_amt = shift_source.get_amt(cpu);
                (
                    cpu.get_reg(rm as usize).rotate_right(shift_amt),
                    cpu.get_reg(rm as usize).bit(shift_amt as usize - 1) == 1,
                )
            }
            Self::RRX { rm } => {
                let carry_in = cpu.get_cpsr_bits(CPSR::C);
                (
                    (cpu.get_reg(rm as usize) >> 1).bits(0, 30) | carry_in << 31,
                    cpu.get_reg(rm as usize).bit(0) == 1,
                )
            }
        }
    }
}

#[derive(Debug)]
struct RegOperandInfo {
    shift_source: ShiftSource,
    rm: u32,
}

struct DataProcessingOperand {
    op2: u32,
    carry_out: u32, // Single bit
    /// Debug/logging information about how op2 was derived
    op2_info: Option<RegOperandInfo>,
}

pub(super) struct DataProcessingFields {
    instruction: u32,
    set: bool,
    rn: u32,
    rd: u32,
    shifter: ShifterOperand,
}

impl DataProcessingFields {
    fn parse(instruction: u32) -> DataProcessingFields {
        let set = (instruction >> 20) & 1 != 0;
        let rn = (instruction >> 16) & 0xf;
        let rd = (instruction >> 12) & 0xf;

        DataProcessingFields {
            instruction,
            set,
            rn,
            rd,
            shifter: ShifterOperand::parse(instruction),
        }
    }
}

pub struct Add;
pub struct Tst;
pub struct Teq;
pub struct Cmp;
pub struct Orr;
pub struct Mov;

#[inline]
fn execute_op<F>(cpu: &mut Cpu, bus: &mut Bus, instruction: u32, op_closure: F)
where
    F: Fn(u32, u32) -> u32,
{
    let fields = DataProcessingFields::parse(instruction);
    let (op2, c) = fields.shifter.op2(cpu);

    if fields.set {
        todo!();
    }

    let output = op_closure(cpu.get_reg(fields.rn as usize), op2);
    cpu.set_reg(fields.rd as usize, output);
}

impl ArmInstruction for Add {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32)
    {
        execute_op(cpu, bus, instruction, |op1, op2| op1 + op2);
    }

    fn disassembly(&self, instruction: u32) -> String
    {
        let fields = DataProcessingFields::parse(instruction);
        format!("ADD r{}, r{}, {:?}", fields.rd, fields.rn, fields.shifter)
    }
}

impl ArmInstruction for Tst {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32)
    {
        let fields = DataProcessingFields::parse(instruction);
        let (op2, c) = fields.shifter.op2(cpu);

        let output = cpu.get_reg(fields.rn as usize) & op2;

        cpu.set_flag(CPSR::N, output.bit(31) == 1);
        cpu.set_flag(CPSR::Z, if output == 0 { true } else { false });
        cpu.set_flag(CPSR::C, c)
    }

    fn disassembly(&self, instruction: u32) -> String
    {
        let fields = DataProcessingFields::parse(instruction);
        format!("TST r{}, {:?}", fields.rn, fields.shifter)
    }
}

impl ArmInstruction for Teq {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32)
    {
        let fields = DataProcessingFields::parse(instruction);
        let (op2, c) = fields.shifter.op2(cpu);

        let output = cpu.get_reg(fields.rn as usize) ^ op2;

        let n_flag = output.bit(31);
        let z_flag = if output == 0 { 1 } else { 0 };

        cpu.set_flag(CPSR::N, n_flag == 1);
        cpu.set_flag(CPSR::Z, z_flag == 1);
        cpu.set_flag(CPSR::C, c)
    }

    fn disassembly(&self, instruction: u32) -> String
    {
        let fields = DataProcessingFields::parse(instruction);
        format!("TEQ r{}, {:?}", fields.rn, fields.shifter)
    }
}

impl ArmInstruction for Cmp {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32)
    {
        let fields = DataProcessingFields::parse(instruction);
        let (op2, c) = fields.shifter.op2(cpu);

        let op1 = cpu.get_reg(fields.rn as usize);
        let (output, borrow) = op1.overflowing_sub(op2);

        cpu.set_flag(CPSR::N, output.bit(31) == 1);
        cpu.set_flag(CPSR::Z, output == 0);
        cpu.set_flag(CPSR::C, !borrow);

        let overflow = (op1.bit(31) != op2.bit(31)) && (op1.bit(31) != output.bit(31));
        cpu.set_flag(CPSR::V, overflow);
    }

    fn disassembly(&self, instruction: u32) -> String
    {
        let fields = DataProcessingFields::parse(instruction);
        format!("CMP r{}, {:?}", fields.rn, fields.shifter)
    }
}

impl ArmInstruction for Orr {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32)
    {
        execute_op(cpu, bus, instruction, |op1, op2| op1 | op2)
    }
    fn disassembly(&self, instruction: u32) -> String
    {
        let fields = DataProcessingFields::parse(instruction);
        format!("ORR r{}, r{}, {:?}", fields.rd, fields.rn, fields.shifter)
    }
}

impl ArmInstruction for Mov {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u32)
    {
        let fields = DataProcessingFields::parse(instruction);
        let (op2, c) = fields.shifter.op2(cpu);

        if fields.set {
            todo!()
        }

        cpu.set_reg(fields.rd as usize, op2);
        if fields.rd == 15 {
            cpu.flush_pipeline();
        }
    }
    fn disassembly(&self, instruction: u32) -> String
    {
        let fields = DataProcessingFields::parse(instruction);
        format!("MOV r{}, {:?}", fields.rd, fields.shifter)
    }
}
