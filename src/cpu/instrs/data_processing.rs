use crate::Cpu;
use crate::Bus;
use crate::logging::Targets;
use tracing::trace;

struct DataProcessingFields {
    op2: u32,
    set: bool,
    rn: u32,
    rd: u32,
}

impl DataProcessingFields {
    fn data_processing_op2_immediate(instruction: u32) -> u32 {
        let imm = instruction & 0xff;
        let rot = (instruction >> 8) & 0xf;
        imm.rotate_right(2 * rot)
    }

    fn data_processing_op2_shift_reg(cpu: &Cpu, instruction: u32) -> u32 {
        let low_bit = (instruction >> 4) & 1;
        let shift_type = (instruction >> 5) & 0b11;
        let rm = instruction & 0xf;
        let rm_contents = cpu.get_reg(rm as usize);

        let shift_amt = if low_bit == 0 {
            // Read shift_amt from immediate field
            (instruction >> 7) & 0b11111
        } else {
            // Read shift_amt from bottom byte of register
            let shift_reg = (instruction >> 8) & 0xf;

            // TODO: Is this check necessary - maybe the format of instructions prevents this
            if shift_reg == 15 {
                panic!("shift reg cannot be r15")
            }

            cpu.get_reg(shift_reg as usize) & 0xff
        };

        match shift_type {
            0b00 => rm_contents << shift_amt,
            0b01 => rm_contents >> shift_amt,
            0b10 => ((rm_contents as i32) >> shift_amt) as u32,
            0b11 => rm_contents.rotate_right(shift_amt),
            _ => unreachable!()
        }
    }

    pub fn parse(cpu: &Cpu, instruction: u32) -> DataProcessingFields {
        let is_immediate = (instruction >> 25) & 1 != 0;

        let set = (instruction >> 20) & 1 != 0;

        let rn = (instruction >> 16) & 0xf;
        let rd = (instruction >> 12) & 0xf;

        let op2 = if is_immediate {
            Self::data_processing_op2_immediate(instruction)
        } else {
            Self::data_processing_op2_shift_reg(cpu, instruction)
        };

        DataProcessingFields {
            op2,
            set,
            rn,
            rd,
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

        trace!(target: Targets::Instr.value(), "MOV r{}, {:x}", fields.rd, fields.op2);

        self.set_reg(fields.rd as usize, fields.op2);
    }

    pub fn data_processing(&mut self, bus: &mut Bus, instruction: u32) {
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
