use tracing::trace;

use crate::{bus::Bus, utils::AddressableBits, logging::Targets, cpu::{Cpu, CPSR}};


type InstructionFpThumb = fn(&mut Cpu, &mut Bus, u16);

fn format_mask(instruction: u16, format: u16, mask: u16) -> bool {
    instruction & mask == format
}

#[derive(Debug)]
enum ThumbInstr {
    SoftwareInterrupt,
    Branch,
    ConditionalBranch,
    MultipleLoadStore,
    LongBranchWithLink,
    AddOffsetToStackPointer,
    PushPopRegisters,
    LoadStoreHalfword,
    SpRelativeLoadStore,
    LoadAddress,
    LoadStoreImmediateOffset,
    LoadStoreRegisterOffset,
    LoadStoreSExByteHalfword,
    PcRelativeLoad,
    HiRegisterOperationsBranchExchange,
    AluOperations,
    MoveCompareAddSubtractImmediate,
    AddSubtract,
    MoveShiftedRegister,
}

impl ThumbInstr {
    fn matches(&self, instruction: u16) -> bool {
        match *self {
            ThumbInstr::SoftwareInterrupt => {
                let format = 0b1101_1111_0000_0000;
                let mask = 0b1111_1111_0000_0000;
                instruction & mask == format
            }
            ThumbInstr::Branch => {
                let format = 0b1110_0000_0000_0000;
                let mask = 0b1111_1000_0000_0000;
                instruction & mask == format
            }
            ThumbInstr::ConditionalBranch => {
                let format = 0b1101_0000_0000_0000;
                let mask = 0b1111_0000_0000_0000;
                instruction & mask == format
            }
            ThumbInstr::MultipleLoadStore => {
                let format = 0b1100_0000_0000_0000;
                let mask = 0b1111_0000_0000_0000;
                instruction & mask == format
            }
            ThumbInstr::LongBranchWithLink => {
                let format = 0b1111_0000_0000_0000;
                let mask = 0b1111_0000_0000_0000;
                instruction & mask == format
            } 
            ThumbInstr::AddOffsetToStackPointer => {
                let format = 0b1011_0000_0000_0000;
                let mask = 0b1111_1111_0000_0000;
                instruction & mask == format
            }
            ThumbInstr::PushPopRegisters => {
                let format = 0b1011_0100_0000_0000;
                let mask = 0b1111_0110_0000_0000;
                instruction & mask == format
            }
            ThumbInstr::LoadStoreHalfword => {
                let format = 0b1000_0000_0000_0000;
                let mask = 0b1111_0000_0000_0000;
                instruction & mask == format
            }
            ThumbInstr::SpRelativeLoadStore => {
                let format = 0b1001_0000_0000_0000;
                let mask = 0b1111_0000_0000_0000;
                instruction & mask == format
            }
            ThumbInstr::LoadAddress => format_mask(instruction, 0b1010_0000_0000_0000, 0b1111_0000_0000_0000),
            ThumbInstr::LoadStoreImmediateOffset => format_mask(instruction, 0b0110_0000_0000_0000, 0b1110_0000_0000_0000),
            ThumbInstr::LoadStoreRegisterOffset => format_mask(instruction, 0b0101_0000_0000_0000, 0b1111_0010_0000_0000),
            ThumbInstr::LoadStoreSExByteHalfword => format_mask(instruction, 0b0101_0010_0000_0000, 0b1111_0010_0000_0000),
            ThumbInstr::PcRelativeLoad => format_mask(instruction, 0b0100_1000_0000_0000, 0b1111_1000_0000_0000),
            Self::HiRegisterOperationsBranchExchange => format_mask(instruction, 0b0100_0100_0000_0000, 0b1111_1100_0000_0000),
            Self::AluOperations => format_mask(instruction, 0b0100_0000_0000_0000, 0b1111_1100_0000_0000),
            Self::MoveCompareAddSubtractImmediate => format_mask(instruction, 0b0010_0000_0000_0000, 0b1110_0000_0000_0000),
            Self::AddSubtract => format_mask(instruction, 0b0001_1000_0000_0000, 0b1111_1000_0000_0000),
            Self::MoveShiftedRegister => format_mask(instruction, 0b0000_0000_0000_0000, 0b1110_0000_0000_0000),

            _ => todo!("implement instruction {:#018b}", instruction)
        }
    }

    pub fn decode(instruction: u16) -> Self {
        let instrs = [
            Self::SoftwareInterrupt,
            Self::Branch,
            Self::ConditionalBranch,
            Self::MultipleLoadStore,
            Self::LongBranchWithLink,
            Self::AddOffsetToStackPointer,
            Self::PushPopRegisters,
            Self::LoadStoreHalfword,
            Self::SpRelativeLoadStore,
            Self::LoadAddress,
            Self::LoadStoreImmediateOffset,
            Self::LoadStoreRegisterOffset,
            Self::LoadStoreSExByteHalfword,
            Self::PcRelativeLoad,
            Self::HiRegisterOperationsBranchExchange,
            Self::AluOperations,
            Self::MoveCompareAddSubtractImmediate,
            Self::AddSubtract,
            Self::MoveShiftedRegister,
        ];

        for thumb_instr in instrs.into_iter() {
            if thumb_instr.matches(instruction) {
                return thumb_instr;
            }
        }

        panic!("failed to decode thumb instruction {:b}", instruction)
    }
}

impl Cpu {
    fn load_address(&mut self, _bus: &mut Bus, instruction: u16) {
        let sp = instruction.bit(11);
        let rd = instruction.bits(8, 10);
        let imm = instruction.bits(0, 7);

        trace!(target: Targets::Thumb.value(), "ADD r{}, {}, {:x}", rd, if sp == 0 { "PC" } else { "SP" }, imm);

        let source = if sp == 0 {
            self.get_reg(15)
        } else {
            self.get_reg(13)
        };

        self.set_reg(rd as usize, source + imm as u32);
    }

    fn thumb_mov_hi_reg(&mut self, _bus: &mut Bus, instruction: u16) {
        let rd = (instruction.bit(7) << 3) | instruction.bits(0, 2);
        let rm = instruction.bits(3, 5);

        trace!(target: Targets::Thumb.value(), "MOV r{}, r{}", rd, rm);

        self.set_reg(rd as usize, self.get_reg(rm as usize));

        if rd == 15 {
            self.flush_pipeline();
        }
    }

    fn thumb_branch_exchange(&mut self, _bus: &mut Bus, instruction: u16) {
        let rm = instruction.bits(3, 6);
        let val = self.get_reg(rm as usize);

        trace!(target: Targets::Thumb.value(), "BX r{}", rm);

        self.set_flag(CPSR::T, val.bit(0) == 1);
        self.set_reg(15, (val.bits(1, 31) as u32) << 1);
        self.flush_pipeline();
    }

    fn hi_register_operations_branch_exchange(&mut self, bus: &mut Bus, instruction: u16) {
        let opcode = instruction.bits(8, 9);

        match opcode {
            0b10 => self.thumb_mov_hi_reg(bus, instruction),
            0b11 => self.thumb_branch_exchange(bus, instruction),
            0b00 ..= 0b11 => todo!("{:b}", opcode),
            _ => unreachable!()
        }
    }

    fn thumb_mov_imm(&mut self, _bus: &mut Bus, instruction: u16) {
        let rd = instruction.bits(8, 10);
        let imm = instruction.bits(0, 7);

        trace!(target: Targets::Thumb.value(), "MOV r{}, {:#x}", rd, imm);

        self.set_reg(rd as usize, imm as u32);

        self.set_flag(CPSR::N, false);
        self.set_flag(CPSR::Z, if imm == 0 { true } else { false });
    }

    fn move_compare_add_subtract_immediate(&mut self, bus: &mut Bus, instruction: u16) {
        let opcode = instruction.bits(11, 12);
        match opcode {
            0b00 => self.thumb_mov_imm(bus, instruction),
            0b01 ..= 0b11 => todo!(),
            _ => unreachable!()
        }
    }

    fn load_store_halfword(&mut self, bus: &mut Bus, instruction: u16) {
        let load = instruction.bit(11);

        if load == 1 {
            todo!()
        }

        let offset = instruction.bits(6, 10);
        let rn = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        let address = self.get_reg(rn as usize) + 2 * offset as u32;

        println!("rn: {} {}", rn, self.get_reg(rn as usize));

        trace!(target: Targets::Thumb.value(), "STRH, r{}, {:#x}", rd, address);

        if address.bit(0) == 0 {
            bus.set_half(address, self.get_reg(rd as usize) as u16);
        } else {
            todo!("UNPREDICTABLE")
        }
    }

    pub(super) fn decode_thumb(&self, instruction: u16) -> InstructionFpThumb {
        let thumb_instr = ThumbInstr::decode(instruction);
        match thumb_instr {
            ThumbInstr::LoadStoreHalfword => Self::load_store_halfword,
            ThumbInstr::MoveCompareAddSubtractImmediate => Self::move_compare_add_subtract_immediate,
            ThumbInstr::HiRegisterOperationsBranchExchange => Self::hi_register_operations_branch_exchange,
            ThumbInstr::LoadAddress => Self::load_address,
            _ => todo!("{:?} {:016b}", thumb_instr, instruction)
        }
    }
}
