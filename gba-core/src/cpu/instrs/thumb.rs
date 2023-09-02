use tracing::trace;

use crate::{
    bus::Bus,
    cpu::{Cpu, CPSR},
    logging::Targets,
    utils::AddressableBits,
};

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
    LoadStoreSignExtended,
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
            ThumbInstr::LoadAddress => {
                format_mask(instruction, 0b1010_0000_0000_0000, 0b1111_0000_0000_0000)
            }
            ThumbInstr::LoadStoreImmediateOffset => {
                format_mask(instruction, 0b0110_0000_0000_0000, 0b1110_0000_0000_0000)
            }
            ThumbInstr::LoadStoreRegisterOffset => {
                format_mask(instruction, 0b0101_0000_0000_0000, 0b1111_0010_0000_0000)
            }
            ThumbInstr::LoadStoreSignExtended => {
                format_mask(instruction, 0b0101_0010_0000_0000, 0b1111_0010_0000_0000)
            }
            ThumbInstr::PcRelativeLoad => {
                format_mask(instruction, 0b0100_1000_0000_0000, 0b1111_1000_0000_0000)
            }
            Self::HiRegisterOperationsBranchExchange => {
                format_mask(instruction, 0b0100_0100_0000_0000, 0b1111_1100_0000_0000)
            }
            Self::AluOperations => {
                format_mask(instruction, 0b0100_0000_0000_0000, 0b1111_1100_0000_0000)
            }
            Self::MoveCompareAddSubtractImmediate => {
                format_mask(instruction, 0b0010_0000_0000_0000, 0b1110_0000_0000_0000)
            }
            Self::AddSubtract => {
                format_mask(instruction, 0b0001_1000_0000_0000, 0b1111_1000_0000_0000)
            }
            Self::MoveShiftedRegister => {
                format_mask(instruction, 0b0000_0000_0000_0000, 0b1110_0000_0000_0000)
            }
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
            Self::LoadStoreSignExtended,
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
    fn load_address(&mut self, _: &mut Bus, instruction: u16) {
        let sp = instruction.bit(11);
        let rd = instruction.bits(8, 10);
        let imm = instruction.bits(0, 7);

        trace!(target: Targets::Thumb.value(), "ADD r{}, {}, {:x}", rd, if sp == 0 { "PC" } else { "SP" }, imm);

        let source = if sp == 0 {
            self.get_reg(15)
        } else {
            self.get_reg(13)
        };

        self.set_reg(rd.into(), source + imm as u32);
    }

    fn add_hi_reg(&mut self, _: &mut Bus, instruction: u16) {
        if instruction.bits(6, 7) == 0b00 {
            panic!("undefined behaviour for add when H1 = H2 = 0")
        }

        let dest = (instruction.bit(7) << 3) | instruction.bits(0, 2);
        let src = instruction.bits(3, 6);

        let result = self.get_reg(dest.into()) + self.get_reg(src.into());
        self.set_reg(dest.into(), result);
    }

    fn thumb_mov_hi_reg(&mut self, _: &mut Bus, instruction: u16) {
        let rd = (instruction.bit(7) << 3) | instruction.bits(0, 2);
        let rm = instruction.bits(3, 5);

        trace!(target: Targets::Thumb.value(), "MOV r{}, r{}", rd, rm);

        self.set_reg(rd.into(), self.get_reg(rm.into()));

        if rd == 15 {
            self.flush_pipeline();
        }
    }

    fn thumb_branch_exchange(&mut self, _: &mut Bus, instruction: u16) {
        let rm = instruction.bits(3, 6);
        let val = self.get_reg(rm.into());

        trace!(target: Targets::Thumb.value(), "BX r{}", rm);

        self.set_flag(CPSR::T, val.bit(0) == 1);
        self.set_reg(15, (val.bits(1, 31) as u32) << 1);
        self.flush_pipeline();
    }

    fn hi_register_operations_branch_exchange(&mut self, bus: &mut Bus, instruction: u16) {
        let opcode = instruction.bits(8, 9);

        match opcode {
            0b00 => self.add_hi_reg(bus, instruction),
            0b01 => todo!("{:b}", opcode),
            0b10 => self.thumb_mov_hi_reg(bus, instruction),
            0b11 => self.thumb_branch_exchange(bus, instruction),
            _ => unreachable!(),
        }
    }

    fn thumb_mov_imm(&mut self, _: &mut Bus, instruction: u16) {
        let rd = instruction.bits(8, 10);
        let imm = instruction.bits(0, 7);

        trace!(target: Targets::Thumb.value(), "MOV r{}, {:#x}", rd, imm);

        self.set_reg(rd.into(), imm as u32);

        self.set_flag(CPSR::N, false);
        self.set_flag(CPSR::Z, if imm == 0 { true } else { false });
    }

    fn thumb_add_imm(&mut self, _: &mut Bus, instruction: u16) {
        let rd = instruction.bits(8, 10);
        let imm = instruction.bits(0, 7) as u32;

        let (result, carry) = self.get_reg(rd.into()).overflowing_add(imm);
        let overflow = self.get_reg(rd.into()).bit(31) == imm.bit(31)
            && self.get_reg(rd.into()).bit(31) != result.bit(31);

        self.set_reg(rd.into(), result);
        self.set_flag(CPSR::N, result.bit(31) == 1);
        self.set_flag(CPSR::Z, result == 0);
        self.set_flag(CPSR::C, carry);
        self.set_flag(CPSR::V, overflow);
    }

    fn thumb_sub_imm(&mut self, _: &mut Bus, instruction: u16) {
        let rd = instruction.bits(8, 10);
        let imm = instruction.bits(0, 7) as u32;

        let (result, carry) = self.get_reg(rd.into()).overflowing_sub(imm);
        let overflow = self.get_reg(rd.into()).bit(31) != imm.bit(31)
            && self.get_reg(rd.into()).bit(31) != result.bit(31);

        self.set_reg(rd.into(), result);
        self.set_flag(CPSR::N, result.bit(31) == 1);
        self.set_flag(CPSR::Z, result == 0);
        self.set_flag(CPSR::C, !carry);
        self.set_flag(CPSR::V, overflow);
    }

    fn move_compare_add_subtract_immediate(&mut self, bus: &mut Bus, instruction: u16) {
        let opcode = instruction.bits(11, 12);
        match opcode {
            0b00 => self.thumb_mov_imm(bus, instruction),
            0b10 => self.thumb_add_imm(bus, instruction),
            0b11 => self.thumb_sub_imm(bus, instruction),
            0b01..=0b11 => todo!("{:02b}", opcode),
            _ => unreachable!(),
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

        let address = self.get_reg(rn.into()) + 2 * offset as u32;

        println!("rn: {} {}", rn, self.get_reg(rn.into()));

        trace!(target: Targets::Thumb.value(), "STRH, r{}, {:#x}", rd, address);

        if address.bit(0) == 0 {
            bus.write_half(address, self.get_reg(rd.into()) as u16);
        } else {
            todo!("UNPREDICTABLE")
        }
    }

    fn pc_relative_load(&mut self, bus: &mut Bus, instruction: u16) {
        let rd = instruction.bits(8, 10);
        let imm = instruction.bits(0, 7);

        let address = (self.get_reg(15) & 0xffff_fffc) + imm as u32 * 4;
        let value = bus.read(address, self);
        self.set_reg(rd.into(), value);
    }

    fn move_shifted_register(&mut self, bus: &mut Bus, instruction: u16) {
        let opcode = instruction.bits(11, 12);
        let imm = instruction.bits(6, 10);
        let rm = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        match opcode {
            0b00 => {
                // LSL
                if imm == 0 {
                    self.set_reg(rd.into(), self.get_reg(rm.into()));
                } else {
                    self.set_flag(CPSR::C, self.get_reg(rm.into()).bit(32 - imm as usize) == 1);
                    self.set_reg(rd.into(), self.get_reg(rm.into()) << imm);
                }
                self.set_flag(CPSR::N, self.get_reg(rd.into()).bit(31) == 1);
                self.set_flag(CPSR::Z, self.get_reg(rd.into()) == 0);
            }
            0b01 => {
                // LSR
                if imm == 0 {
                    self.set_flag(CPSR::C, self.get_reg(rm.into()).bit(31) == 1);
                    self.set_reg(rd.into(), 0);
                } else {
                    self.set_flag(CPSR::C, self.get_reg(rm.into()).bit(imm as usize - 1) == 1);
                    self.set_reg(rd.into(), self.get_reg(rm.into()) >> imm);
                }
                self.set_flag(CPSR::N, self.get_reg(rd.into()).bit(31) == 1);
                self.set_flag(CPSR::Z, self.get_reg(rd.into()) == 0);
            }
            0b10 => {
                // ASR
                if imm == 0 {
                    self.set_flag(CPSR::C, self.get_reg(rm.into()).bit(31) == 1);
                    if self.get_reg(rm.into()).bit(31) == 0 {
                        self.set_reg(rd.into(), 0);
                    } else {
                        self.set_reg(rd.into(), 0xffff_ffff);
                    }
                } else {
                    self.set_flag(CPSR::C, self.get_reg(rm.into()).bit(imm as usize - 1) == 1);
                    self.set_reg(rd.into(), ((self.get_reg(rm.into()) as i32) >> imm) as u32);
                }
                self.set_flag(CPSR::N, self.get_reg(rd.into()).bit(31) == 1);
                self.set_flag(CPSR::Z, self.get_reg(rd.into()) == 0);
            }
            _ => unreachable!(),
        }
    }

    fn conditional_branch(&mut self, _: &mut Bus, instruction: u16) {
        let cond = instruction.bits(8, 11);
        let signed_imm = instruction.bits(0, 7) as i16;

        if self.check_cond(cond as u32) {
            let offset = i32::from(signed_imm) << 1;
            self.set_reg(15, self.get_reg(15).wrapping_add_signed(offset));
            self.flush_pipeline();
        }
    }

    fn long_branch_with_link(&mut self, _: &mut Bus, instruction: u16) {
        let h = instruction.bits(11, 12);
        let offset = instruction.bits(0, 10) as i16;

        match h {
            0b10 => {
                let offset = i32::from(offset);
                self.set_reg(14, self.get_reg(15).wrapping_add_signed(offset << 12));
            }
            0b11 => {
                let lr = self.get_reg(14);
                let next_instr = self.get_reg(15) - 4;
                self.set_reg(15, lr.wrapping_add_signed((offset << 1).into()));
                self.set_reg(14, next_instr);
            }
            _ => unreachable!(),
        }
    }

    fn add_subtract(&mut self, bus: &mut Bus, instruction: u16) {
        let i = instruction.bit(10);
        let op = instruction.bit(9);

        let term: u32 = if i == 0 {
            let rn = instruction.bits(6, 8);
            self.get_reg(rn.into())
        } else {
            instruction.bits(6, 8).into()
        };

        let rs = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        let (result, carry, overflow) = if op == 0 {
            let (result, carry) = self.get_reg(rs.into()).overflowing_add(term);
            let overflow =
                self.get_reg(rs.into()).bit(31) == term.bit(31) && term.bit(31) != result.bit(31);
            (result, carry, overflow)
        } else {
            let (result, carry) = self.get_reg(rs.into()).overflowing_sub(term);
            let overflow = self.get_reg(rs.into()).bit(31) != term.bit(31)
                && self.get_reg(rs.into()).bit(31) != result.bit(31);
            (result, carry, overflow)
        };

        self.set_reg(rd.into(), result);

        if op == 0 {
            self.set_flag(CPSR::N, self.get_reg(rd.into()).bit(31) == 1);
            self.set_flag(CPSR::Z, self.get_reg(rd.into()) == 0);
            self.set_flag(CPSR::C, carry);
            self.set_flag(CPSR::V, overflow);
        } else {
            self.set_flag(CPSR::N, self.get_reg(rd.into()).bit(31) == 1);
            self.set_flag(CPSR::Z, self.get_reg(rd.into()) == 0);
            self.set_flag(CPSR::C, !carry); // Could be inverted, not 100%
            self.set_flag(CPSR::V, overflow);
        }
    }

    fn alu_operations(&mut self, _: &mut Bus, instruction: u16) {
        let opcode = instruction.bits(6, 9);
        let rs = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        match opcode {
            0b1110 => {
                let result = self.get_reg(rd.into()) & !self.get_reg(rs.into());
                self.set_reg(rd.into(), result);
                self.set_flag(CPSR::N, result.bit(31) == 1);
                self.set_flag(CPSR::Z, result == 0);
            }
            0b0000..=0b1111 => todo!("alu operation {:04b}", opcode),
            _ => unreachable!(),
        }
    }

    fn multiple_load_store(&mut self, bus: &mut Bus, instruction: u16) {
        let load = instruction.bit(11) == 1;
        let rn = instruction.bits(8, 10);
        let reg_list = instruction.bits(0, 7);

        if load {
            todo!()
        } else {
            let start_address = self.get_reg(rn.into());
            let mut address = start_address;
            for i in 0..=7 {
                if reg_list.bit(i) == 1 {
                    bus.write(address, self.get_reg(i.try_into().unwrap()));
                    address += 4;
                }
            }
            self.set_reg(
                rn.into(),
                self.get_reg(rn.into()) + reg_list.count_ones() * 4,
            );
        }
    }

    fn branch(&mut self, _: &mut Bus, instruction: u16) {
        let signed_imm = i32::from(instruction.bits(0, 10) as i16);
        self.set_reg(15, self.get_reg(15).wrapping_add_signed(signed_imm << 1));
        self.flush_pipeline();
    }

    fn load_store_sign_extended(&mut self, bus: &mut Bus, instruction: u16) {
        let h_flag = instruction.bit(11);
        let sign_extend = instruction.bit(10);
        let ro = instruction.bits(6, 8);
        let rb = instruction.bits(3, 5);
        let rd = instruction.bits(0, 2);

        match (h_flag, sign_extend) {
            (0, 1) => {
                let address = self.get_reg(rb.into()) + self.get_reg(ro.into());
                let data = bus.read_half(address, self);
                self.set_reg(rd.into(), data);
            }
            _ => todo!("H:{} S:{}", h_flag, sign_extend),
        }
    }

    pub(super) fn decode_thumb(&self, instruction: u16) -> InstructionFpThumb {
        let thumb_instr = ThumbInstr::decode(instruction);
        match thumb_instr {
            ThumbInstr::LoadStoreHalfword => Self::load_store_halfword,
            ThumbInstr::MoveCompareAddSubtractImmediate => {
                Self::move_compare_add_subtract_immediate
            }
            ThumbInstr::HiRegisterOperationsBranchExchange => {
                Self::hi_register_operations_branch_exchange
            }
            ThumbInstr::LoadAddress => Self::load_address,
            ThumbInstr::PcRelativeLoad => Self::pc_relative_load,
            ThumbInstr::MoveShiftedRegister => Self::move_shifted_register,
            ThumbInstr::ConditionalBranch => Self::conditional_branch,
            ThumbInstr::LongBranchWithLink => Self::long_branch_with_link,
            ThumbInstr::AddSubtract => Self::add_subtract,
            ThumbInstr::AluOperations => Self::alu_operations,
            ThumbInstr::MultipleLoadStore => Self::multiple_load_store,
            ThumbInstr::Branch => Self::branch,
            ThumbInstr::LoadStoreSignExtended => Self::load_store_sign_extended,
            _ => todo!("{:?} {:016b}", thumb_instr, instruction),
        }
    }
}
