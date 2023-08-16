use std::fmt::Debug;
use crate::Cpu;
use crate::Bus;
use crate::cpu::CPSR;
use crate::utils::AddressableBits;
use crate::logging::Targets;
use tracing::error;
use tracing::trace;
use tracing::warn;

#[derive(Debug)]
enum ShiftType {
    LSL,
    LSR,
    ASR,
    ROR,
    RRX,
}

enum ShiftSource {
    Immediate(u32),
    Register(u32),
}

impl Debug for ShiftSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ShiftSource::Immediate(x) => write!(f, "{:x}", x),
            ShiftSource::Register(x) => write!(f, "r{}", x),
        }
    }
}

#[derive(Debug)]
struct RegOperandInfo {
    shift_type: ShiftType,
    shift_source: ShiftSource,
    rm: u32,
}

struct DataProcessingFields {
    set: bool,
    rn: u32,
    rd: u32,
    op2: u32,
    carry_out: u32, // Single bit
    /// Debug/logging information about how op2 was derived
    op2_info: Option<RegOperandInfo>,
}

impl DataProcessingFields {
    fn data_processing_op2_immediate(cpu: &Cpu, instruction: u32) -> (u32, u32, Option<RegOperandInfo>) {
        let imm = instruction & 0xff;
        let rot = (instruction >> 8) & 0xf;
        let value = imm.rotate_right(2 * rot);

        let carry_out = if rot == 0 {
            cpu.get_cpsr_bits(CPSR::C)
        } else {
            value.bit(31)
        };

        (value, carry_out, None)
    }

    fn data_processing_op2_shift_reg(cpu: &Cpu, instruction: u32) -> (u32, u32, Option<RegOperandInfo>) {
        let low_bit = (instruction >> 4) & 1;
        let shift_type_bits = (instruction >> 5) & 0b11;
        let rm = instruction & 0xf;
        let rm_contents = cpu.get_reg(rm as usize);

        let shift_source;
        let shift_amt;
        if low_bit == 0 {
            // Read shift_amt from immediate field
            shift_amt = instruction.bits(7, 11);
            shift_source = ShiftSource::Immediate(shift_amt);
        } else {
            // Read shift_amt from bottom byte of register
            let shift_reg = instruction.bits(8, 11);

            // TODO: Is this check necessary - maybe the format of instructions prevents this
            if shift_reg == 15 {
                panic!("shift reg cannot be r15")
            }

            shift_amt = cpu.get_reg(shift_reg as usize) & 0xff;
            shift_source = ShiftSource::Register(shift_reg);
        };

        let op2;
        let shift_type;
        let carry_out;
        match shift_type_bits {
            0b00 => {
                shift_type = ShiftType::LSL;
                if shift_amt == 0 {
                    op2 = rm_contents;
                    carry_out = cpu.get_cpsr_bits(CPSR::C);
                } else {
                    op2 = rm_contents << shift_amt;
                    carry_out = rm_contents.bit(32 - shift_amt as usize);
                }
            },
            0b01 => {
                shift_type = ShiftType::LSR;
                if shift_amt == 0 {
                    // Special case - treat shift_amt as 32
                    op2 = 0;
                    carry_out = rm_contents.bit(31);
                } else {
                    op2 = rm_contents >> shift_amt;
                    carry_out = rm_contents.bit(shift_amt as usize - 1);
                }
            },
            0b10 => {
                shift_type = ShiftType::ASR;
                if shift_amt == 0 {
                    // Special case - treat shift_amt as 32
                    op2 = ((rm_contents as i32) >> 31) as u32;
                    carry_out = rm_contents.bit(31);
                } else {
                    op2 = ((rm_contents as i32) >> shift_amt) as u32;
                    carry_out = rm_contents.bit(shift_amt as usize - 1);
                }
            },
            0b11 => {
                if shift_amt == 0 {
                    // Special case - rotate right extended
                    shift_type = ShiftType::RRX;
                    let carry_in = cpu.get_cpsr_bits(CPSR::C);
                    op2 = (rm_contents >> 1).bits(0, 30) | carry_in << 31;
                    carry_out = rm_contents.bit(0);
                } else {
                    shift_type = ShiftType::ROR;
                    op2 = rm_contents.rotate_right(shift_amt);
                    carry_out = rm_contents.bit(shift_amt as usize - 1);
                }
            },
            _ => unreachable!()
        }

        let reg_op_info = RegOperandInfo {
            shift_type,
            shift_source,
            rm,
        };

        (op2, carry_out, Some(reg_op_info))
    }

    fn parse(cpu: &Cpu, instruction: u32) -> DataProcessingFields {
        let is_immediate = (instruction >> 25) & 1 != 0;

        let set = (instruction >> 20) & 1 != 0;

        let rn = (instruction >> 16) & 0xf;
        let rd = (instruction >> 12) & 0xf;

        let (op2, carry_out, op2_info) = if is_immediate {
            Self::data_processing_op2_immediate(cpu, instruction)
        } else {
            Self::data_processing_op2_shift_reg(cpu, instruction)
        };

        DataProcessingFields {
            set,
            rn,
            rd,
            op2,
            op2_info,
            carry_out,
        }
    }
}

impl Cpu {
    fn add(&mut self, _bus: &mut Bus, instruction: u32) {
        let fields = DataProcessingFields::parse(self, instruction);

        trace!(target: Targets::Arm.value(), "ADD r{}, r{}, {:x}", fields.rd, fields.rn, fields.op2);

        if fields.set {
            todo!();
        }

        let output = self.get_reg(fields.rn as usize) + fields.op2;
        self.set_reg(fields.rd as usize, output);
    }

    fn tst(&mut self, _bus: &mut Bus, instruction: u32) {
        let fields = DataProcessingFields::parse(self, instruction);

        trace!(target: Targets::Arm.value(), "TST r{}, {:x}", fields.rn, fields.op2);

        let output = self.get_reg(fields.rn as usize) & fields.op2;

        self.set_flag(CPSR::N, output.bit(31) == 1);
        self.set_flag(CPSR::Z, if output == 0 { true } else { false });
        self.set_flag(CPSR::C, fields.carry_out == 1);
    }

    fn teq(&mut self, _bus: &mut Bus, instruction: u32) {
        let fields = DataProcessingFields::parse(self, instruction);

        trace!(target: Targets::Arm.value(), "TEQ r{}, {:x}", fields.rn, fields.op2);

        let output = self.get_reg(fields.rn as usize) ^ fields.op2;

        let n_flag = output.bit(31);
        let z_flag = if output == 0 { 1 } else { 0 };
        let c_flag = fields.carry_out;

        self.set_flag(CPSR::N, n_flag == 1);
        self.set_flag(CPSR::Z, z_flag == 1);
        self.set_flag(CPSR::C, c_flag == 1);
    }

    fn orr(&mut self, _bus: &mut Bus, instruction: u32) {
        let fields = DataProcessingFields::parse(self, instruction);

        trace!(target: Targets::Arm.value(), "ORR r{}, r{}, {:x}", fields.rd, fields.rn, fields.op2);

        if fields.set {
            todo!("Status bits for SET not implemented")
        }


        let output = self.get_reg(fields.rn as usize) | fields.op2;
        self.set_reg(fields.rd as usize, output);
    }

    fn mov(&mut self, _bus: &mut Bus, instruction: u32) {
        let fields = DataProcessingFields::parse(self, instruction);

        if fields.set {
            warn!("Ignoring SET bit in MOV on purpose");
            //todo!("Status bits for SET not implemented")
        }

        trace!(target: Targets::Arm.value(), "MOV r{}, {:x}, {:?}", fields.rd, fields.op2, fields.op2_info);

        self.set_reg(fields.rd as usize, fields.op2);

        if fields.rd == 15 {
            self.flush_pipeline();
        }
    }

    pub(super) fn data_processing(&mut self, bus: &mut Bus, instruction: u32) {
        let opcode = (instruction >> 21) & 0xf;

        log::trace!("Data processing opcode {:#06b}", opcode);

        match opcode {
            0b0100 => self.add(bus, instruction),
            0b1000 => self.tst(bus, instruction),
            0b1001 => self.teq(bus, instruction),
            0b1100 => self.orr(bus, instruction),
            0b1101 => self.mov(bus, instruction),
            0b0000 ..= 0b1111 => todo!("opcode {:#06b} isn't implemented yet", opcode),
            _ => unreachable!(),
        }
    }
}
