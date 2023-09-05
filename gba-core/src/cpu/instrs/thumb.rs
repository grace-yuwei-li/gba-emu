mod add_offset_to_stack_pointer;
mod add_subtract;
mod alu_operations;
mod branch;
mod high_reg_ops_or_bx;
mod load_address;
mod load_store_halfword;
mod load_store_immediate_offset;
mod load_store_register_offset;
mod load_store_sign_extended;
mod mov_cmp_add_sub_immediate;
mod move_shifted_register;
mod multiple_load_store;
mod pc_relative_load;
mod push_pop_regs;
mod sp_relative_ls;

use crate::{bus::Bus, cpu::Cpu};

use super::arm::TodoInstruction;

pub trait ThumbInstruction {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, instruction: u16);
    fn disassembly(&self, instruction: u16) -> String;
}

impl ThumbInstruction for TodoInstruction {
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, _: u16) {
        todo!(
            "TODO: {} at PC: {:x}",
            self.0,
            cpu.get_executing_instruction_pc()
        )
    }

    fn disassembly(&self, _: u16) -> String {
        format!("TODO: {}", self.0)
    }
}

fn format_mask(instruction: u16, format: u16, mask: u16) -> bool {
    instruction & mask == format
}

#[derive(Debug)]
enum ThumbInstrGroup {
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
    HiRegOpsOrBx,
    AluOperations,
    MoveCompareAddSubtractImmediate,
    AddSubtract,
    MoveShiftedRegister,
    Invalid,
}

impl ThumbInstrGroup {
    fn matches(&self, instruction: u16) -> bool {
        match *self {
            Self::SoftwareInterrupt => {
                format_mask(instruction, 0b1101_1111_0000_0000, 0b1111_1111_0000_0000)
            }
            Self::Branch => format_mask(instruction, 0b1110_0000_0000_0000, 0b1111_1000_0000_0000),
            Self::ConditionalBranch => {
                format_mask(instruction, 0b1101_0000_0000_0000, 0b1111_0000_0000_0000)
            }
            Self::MultipleLoadStore => {
                format_mask(instruction, 0b1100_0000_0000_0000, 0b1111_0000_0000_0000)
            }
            Self::LongBranchWithLink => {
                format_mask(instruction, 0b1111_0000_0000_0000, 0b1111_0000_0000_0000)
            }
            Self::AddOffsetToStackPointer => {
                format_mask(instruction, 0b1011_0000_0000_0000, 0b1111_1111_0000_0000)
            }
            Self::PushPopRegisters => {
                format_mask(instruction, 0b1011_0100_0000_0000, 0b1111_0110_0000_0000)
            }
            Self::LoadStoreHalfword => {
                format_mask(instruction, 0b1000_0000_0000_0000, 0b1111_0000_0000_0000)
            }
            Self::SpRelativeLoadStore => {
                format_mask(instruction, 0b1001_0000_0000_0000, 0b1111_0000_0000_0000)
            }
            Self::LoadAddress => {
                format_mask(instruction, 0b1010_0000_0000_0000, 0b1111_0000_0000_0000)
            }
            Self::LoadStoreImmediateOffset => {
                format_mask(instruction, 0b0110_0000_0000_0000, 0b1110_0000_0000_0000)
            }
            Self::LoadStoreRegisterOffset => {
                format_mask(instruction, 0b0101_0000_0000_0000, 0b1111_0010_0000_0000)
            }
            Self::LoadStoreSignExtended => {
                format_mask(instruction, 0b0101_0010_0000_0000, 0b1111_0010_0000_0000)
            }
            Self::PcRelativeLoad => {
                format_mask(instruction, 0b0100_1000_0000_0000, 0b1111_1000_0000_0000)
            }
            Self::HiRegOpsOrBx => {
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
            Self::Invalid => unreachable!(),
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
            Self::HiRegOpsOrBx,
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

        Self::Invalid
    }
}

impl Cpu {
    pub(super) fn decode_thumb(instruction: u16) -> Box<dyn ThumbInstruction> {
        let thumb_instr = ThumbInstrGroup::decode(instruction);
        match thumb_instr {
            ThumbInstrGroup::LoadStoreHalfword => load_store_halfword::decode(instruction),
            ThumbInstrGroup::MoveCompareAddSubtractImmediate => {
                mov_cmp_add_sub_immediate::decode(instruction)
            }
            ThumbInstrGroup::HiRegOpsOrBx => high_reg_ops_or_bx::decode(instruction),
            ThumbInstrGroup::LoadAddress => load_address::decode(instruction),
            ThumbInstrGroup::PcRelativeLoad => Box::new(pc_relative_load::LdrPc),
            ThumbInstrGroup::MoveShiftedRegister => move_shifted_register::decode(instruction),
            ThumbInstrGroup::ConditionalBranch => Box::new(branch::ConditionalBranch),
            ThumbInstrGroup::LongBranchWithLink => {
                branch::decode_long_branch_with_link(instruction)
            }
            ThumbInstrGroup::AddSubtract => add_subtract::decode(instruction),
            ThumbInstrGroup::AluOperations => alu_operations::decode(instruction),
            ThumbInstrGroup::MultipleLoadStore => multiple_load_store::decode(instruction),
            ThumbInstrGroup::Branch => Box::new(branch::Branch),
            ThumbInstrGroup::LoadStoreSignExtended => load_store_sign_extended::decode(instruction),
            ThumbInstrGroup::AddOffsetToStackPointer => add_offset_to_stack_pointer::decode(),
            ThumbInstrGroup::LoadStoreImmediateOffset => {
                load_store_immediate_offset::decode(instruction)
            }
            ThumbInstrGroup::LoadStoreRegisterOffset => {
                load_store_register_offset::decode(instruction)
            }
            ThumbInstrGroup::SpRelativeLoadStore => sp_relative_ls::decode(instruction),
            ThumbInstrGroup::PushPopRegisters => push_pop_regs::decode(instruction),
            ThumbInstrGroup::SoftwareInterrupt | ThumbInstrGroup::Invalid => Box::new(
                TodoInstruction::new_message(format!("{:?} {:016b}", thumb_instr, instruction)),
            ),
        }
    }
}
