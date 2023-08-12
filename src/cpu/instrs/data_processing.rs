use std::fmt::Debug;
use crate::Cpu;
use crate::Bus;
use crate::utils::AddressableBits;
use crate::logging::Targets;
use tracing::trace;

#[derive(Debug)]
enum ShiftType {
    LSL,
    LSR,
    ASR,
    RR,
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
    /// Debug/logging information about how op2 was derived
    op2_info: Option<RegOperandInfo>,
}

impl DataProcessingFields {
    fn data_processing_op2_immediate(instruction: u32) -> u32 {
        let imm = instruction & 0xff;
        let rot = (instruction >> 8) & 0xf;
        imm.rotate_right(2 * rot)
    }

    fn data_processing_op2_shift_reg(cpu: &Cpu, instruction: u32) -> (u32, RegOperandInfo) {
        let low_bit = (instruction >> 4) & 1;
        let shift_type = (instruction >> 5) & 0b11;
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

        match shift_type {
            0b00 => (rm_contents << shift_amt, RegOperandInfo {
                shift_type: ShiftType::LSL,
                shift_source,
                rm,
            }),
            0b01 => (rm_contents >> shift_amt, RegOperandInfo {
                shift_type: ShiftType::LSR,
                shift_source,
                rm,
            }),
            0b10 => (((rm_contents as i32) >> shift_amt) as u32, RegOperandInfo {
                shift_type: ShiftType::ASR,
                shift_source,
                rm,
            }),
            0b11 => (rm_contents.rotate_right(shift_amt), RegOperandInfo {
                shift_type: ShiftType::RR,
                shift_source,
                rm,
            }),
            _ => unreachable!()
        }
    }

    fn parse(cpu: &Cpu, instruction: u32) -> DataProcessingFields {
        let is_immediate = (instruction >> 25) & 1 != 0;

        let set = (instruction >> 20) & 1 != 0;

        let rn = (instruction >> 16) & 0xf;
        let rd = (instruction >> 12) & 0xf;

        let op2;
        let op2_info;
        if is_immediate {
            op2 = Self::data_processing_op2_immediate(instruction);
            op2_info = None;
        } else {
            let (op2_local, op2_info_local) = Self::data_processing_op2_shift_reg(cpu, instruction);
            op2 = op2_local;
            op2_info = Some(op2_info_local);
        };

        DataProcessingFields {
            set,
            rn,
            rd,
            op2,
            op2_info,
        }
    }
}

impl Cpu {
    fn dp_orr(&mut self, _bus: &mut Bus, instruction: u32) {
        let fields = DataProcessingFields::parse(self, instruction);

        if fields.set {
            todo!("Status bits for SET not implemented")
        }

        trace!(target: Targets::Instr.value(), "ORR r{}, r{}, {:x}", fields.rd, fields.rn, fields.op2);

        let output = self.get_reg(fields.rn as usize) | fields.op2;
        self.set_reg(fields.rd as usize, output);
    }

    fn dp_mov(&mut self, _bus: &mut Bus, instruction: u32) {
        let fields = DataProcessingFields::parse(self, instruction);

        if fields.set {
            todo!("Status bits for SET not implemented")
        }

        trace!(target: Targets::Instr.value(), "MOV r{}, {:x}, {:?}", fields.rd, fields.op2, fields.op2_info);

        self.set_reg(fields.rd as usize, fields.op2);
    }

    pub(super) fn data_processing(&mut self, bus: &mut Bus, instruction: u32) {
        let opcode = (instruction >> 21) & 0xf;

        log::trace!("Data processing opcode {:#06b}", opcode);

        match opcode {
            0b1100 => self.dp_orr(bus, instruction),
            0b1101 => self.dp_mov(bus, instruction),
            0b0000 ..= 0b1111 => todo!("opcode {:#06b} isn't implemented yet", opcode),
            _ => unreachable!(),
        }
    }
}
