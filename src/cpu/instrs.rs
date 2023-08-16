mod data_processing;
mod branch;
mod branch_and_exchange;
mod block_data_transfer;
mod halfword_transfer;
mod psr_transfer;
mod single_data_transfer;

use crate::bus::Bus;
use crate::cpu::State;
use crate::utils::AddressableBits;

use super::Cpu;
use super::CPSR;
use log;

#[derive(Debug, PartialEq, Eq)]
enum MetaInstr {
    DataProcessing,
    PsrTransfer,
    Multiply,
    MultiplyLong,
    SingleDataSwap,
    BranchAndExchange,
    HalfwordTransReg,
    HalfwordTransImm,
    SingleDataTrans,
    Undefined,
    BlockDataTrans,
    Branch,
    CoprocDataTrans,
    CoprocDataOp,
    CoprocRegTrans,
    SoftwareInterrupt,
}

impl MetaInstr {
    /// Returns masks for bits in (high, low) format.
    /// High bits are 20-27 and low bits are 7-4.
    fn bit_format(&self) -> (u32, u32) {
        match *self {
            Self::DataProcessing => (0b0000_0000, 0b0000),
            Self::PsrTransfer => unimplemented!(),
            Self::Multiply => (0b0000_0000, 0b1001),
            Self::MultiplyLong => (0b0000_1000, 0b1001),
            Self::SingleDataSwap => (0b0001_0000, 0b1001),
            Self::BranchAndExchange => (0b0001_0010, 0b0001),
            Self::HalfwordTransReg => (0b0000_0000, 0b1001),
            Self::HalfwordTransImm => (0b0000_0100, 0b1001),
            Self::SingleDataTrans => (0b0100_0000, 0b0000),
            Self::Undefined => (0b0110_0000, 0b0001),
            Self::BlockDataTrans => (0b1000_0000, 0b0000),
            Self::Branch => (0b1010_0000, 0b0000),
            Self::CoprocDataTrans => (0b1100_0000, 0b0000),
            Self::CoprocDataOp => (0b1110_0000, 0b0000),
            Self::CoprocRegTrans => (0b1110_0000, 0b0001),
            Self::SoftwareInterrupt => (0b1111_0000, 0b0000),
        }
    }

    fn bit_mask(&self) -> (u32, u32) {
        match *self {
            Self::DataProcessing => (0b1100_0000, 0b0000),
            Self::PsrTransfer => unimplemented!(),
            Self::Multiply => (0b1111_1100, 0b1111),
            Self::MultiplyLong => (0b1111_1000, 0b1111),
            Self::SingleDataSwap => (0b1111_1011, 0b1111),
            Self::BranchAndExchange => (0b1111_1111, 0b1111),
            Self::HalfwordTransReg => (0b1110_0100, 0b1001),
            Self::HalfwordTransImm => (0b1110_0100, 0b1001),
            Self::SingleDataTrans => (0b1100_0000, 0b0000),
            Self::Undefined => (0b1110_0000, 0b0001),
            Self::BlockDataTrans => (0b1110_0000, 0b0000),
            Self::Branch => (0b1110_0000, 0b0000),
            Self::CoprocDataTrans => (0b1110_0000, 0b0000),
            Self::CoprocDataOp => (0b1111_0000, 0b0001),
            Self::CoprocRegTrans => (0b1111_0000, 0b0001),
            Self::SoftwareInterrupt => (0b1111_0000, 0b0000),
        }
    }

    pub fn decode_arm(instruction: u32) -> Option<Self> {
        let high_bits = (instruction >> 20) & 0b1111_1111;
        let low_bits = (instruction >> 4) & 0b1111;

        let instrs = [
            Self::BranchAndExchange,
            Self::BlockDataTrans,
            Self::Branch,
            Self::SoftwareInterrupt,
            Self::Undefined,
            Self::SingleDataTrans,
            Self::SingleDataSwap,
            Self::Multiply,
            Self::MultiplyLong,
            Self::HalfwordTransReg,
            Self::HalfwordTransImm,
            Self::CoprocDataTrans,
            Self::CoprocDataOp,
            Self::CoprocRegTrans,
            Self::PsrTransfer,
            Self::DataProcessing,
        ];

        for meta_instr in instrs.into_iter() {
            if meta_instr != Self::PsrTransfer {
                let (high_mask, low_mask) = meta_instr.bit_mask();
                let (high_fmt, low_fmt) = meta_instr.bit_format();

                if high_bits & high_mask == high_fmt && low_bits & low_mask == low_fmt {
                    return Some(meta_instr);
                }
            } else {
                // PSR-specific check
                let opcode = instruction.bits(21, 24);
                let s = instruction.bit(20);
                let opcode_only_sets_flags = (0b1000 ..= 0b1011).contains(&opcode);
                if instruction.bits(26, 27) == 0 && opcode_only_sets_flags && s == 0 {
                    return Some(meta_instr);
                }
            }
        }

        None
    }
}

pub type InstructionFp = fn(&mut Cpu, &mut Bus, u32);


impl Cpu {
    pub fn check_cond(&mut self, instruction: u32) -> bool {
        let cond_bits = instruction >> 28;

        match cond_bits {
            0b0000 => self.get_cpsr_bits(CPSR::Z) != 0,
            0b0001 => self.get_cpsr_bits(CPSR::Z) == 0,
            0b0010 => self.get_cpsr_bits(CPSR::C) != 0,
            0b0011 => self.get_cpsr_bits(CPSR::C) == 0,
            0b0100 => self.get_cpsr_bits(CPSR::N) != 0,
            0b0101 => self.get_cpsr_bits(CPSR::N) == 0,
            0b0110 => self.get_cpsr_bits(CPSR::V) != 0,
            0b0111 => self.get_cpsr_bits(CPSR::V) == 0,
            0b1000 => self.get_cpsr_bits(CPSR::C) != 0 && self.get_cpsr_bits(CPSR::Z) == 0,
            0b1001 => self.get_cpsr_bits(CPSR::C) == 0 || self.get_cpsr_bits(CPSR::Z) != 0,
            0b1010 => self.get_cpsr_bits(CPSR::N) == self.get_cpsr_bits(CPSR::V),
            0b1011 => self.get_cpsr_bits(CPSR::N) != self.get_cpsr_bits(CPSR::V),
            0b1100 => self.get_cpsr_bits(CPSR::Z) == 0 && (self.get_cpsr_bits(CPSR::N) == self.get_cpsr_bits(CPSR::V)),
            0b1101 => self.get_cpsr_bits(CPSR::Z) != 0 || (self.get_cpsr_bits(CPSR::N) != self.get_cpsr_bits(CPSR::V)),
            0b1110 => true,
            0b1111 => unimplemented!(),
            _ => unreachable!(),
        }
    }

    fn multiply(&mut self, bus: &mut Bus, instruction: u32) {
        todo!()
    }

    fn multiply_long(&mut self, bus: &mut Bus, instruction: u32) {
        todo!()
    }

    fn single_data_swap(&mut self, bus: &mut Bus, instruction: u32) {
        todo!()
    }

    fn undefined(&mut self, bus: &mut Bus, instruction: u32) {
        todo!()
    }

    fn coprocessor_data_transfer(&mut self, bus: &mut Bus, instruction: u32) {
        todo!()
    }

    fn coprocess_data_operation(&mut self, bus: &mut Bus, instruction: u32) {
        todo!()
    }

    fn coprocessor_register_transfer(&mut self, bus: &mut Bus, instruction: u32) {
        todo!()
    }

    fn software_interrupt(&mut self, bus: &mut Bus, instruction: u32) {
        todo!()
    }

    fn unimplemented_instruction(&mut self, bus: &mut Bus, instruction: u32) {
        todo!("Unimplemented instruction {:032b}", instruction)
    }

    fn decode_arm(&mut self, instruction: u32) -> InstructionFp {
        let meta_instr = match MetaInstr::decode_arm(instruction) {
            Some(meta_instr) => meta_instr,
            None => {
                log::warn!("Failed to decode arm instruction");
                return Self::unimplemented_instruction;
            }
        };

        log::trace!("Instruction {:08x} decoded to {:?}", instruction, meta_instr);

        match meta_instr {
            MetaInstr::DataProcessing => Self::data_processing,
            MetaInstr::PsrTransfer => Self::psr_transfer,
            MetaInstr::Multiply => Self::multiply,
            MetaInstr::MultiplyLong => Self::multiply_long,
            MetaInstr::SingleDataSwap => Self::single_data_swap,
            MetaInstr::BranchAndExchange => Self::branch_and_exchange,
            MetaInstr::HalfwordTransReg => Self::halfword_transfer,
            MetaInstr::HalfwordTransImm => Self::halfword_transfer,
            MetaInstr::SingleDataTrans => Self::single_data_transfer,
            MetaInstr::Undefined => Self::undefined,
            MetaInstr::BlockDataTrans => Self::block_data_transfer,
            MetaInstr::Branch => Self::branch,
            MetaInstr::CoprocDataTrans => Self::coprocessor_data_transfer,
            MetaInstr::CoprocDataOp => Self::coprocess_data_operation,
            MetaInstr::CoprocRegTrans => Self::coprocessor_register_transfer,
            MetaInstr::SoftwareInterrupt => Self::software_interrupt,
        }
    }

    pub fn execute(&mut self, bus: &mut Bus, instruction: u32) {
        match self.get_state() {
            State::ARM => {
                if !self.check_cond(instruction) {
                    log::trace!("Cond check failed for instruction {:#034b}", instruction);
                    return;
                }

                log::trace!("Executing ARM instruction {:08x}", instruction);
                let fp = self.decode_arm(instruction);
                fp(self, bus, instruction)
            },
            State::Thumb => {
                log::trace!("Executing THUMB instruction {:04x}", instruction as u16);
                let fp = self.decode_thumb(instruction as u16);
                fp(self, bus, instruction as u16)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// We decode instructions by only looking at 12 bits, so
    /// the only relevant instructions are ones that set these bits
    /// all other bits can be zeroed.
    fn relevant_instructions() -> Vec<u32> {
        let mut out: Vec<u32> = vec![];

        for low_bits in 0..=0b1111 {
            for high_bits in 0..=0b1111_1111 {
                let instr = high_bits << 20 | low_bits << 4;
                out.push(instr);
            }
        }

        out
    }

    fn test_decode_arm_cond(target: MetaInstr, format: u32, mask: u32, cond: &dyn Fn(u32) -> bool) {
        for base_val in relevant_instructions().into_iter() {
            let val = (base_val & !format) | mask;

            if !cond(val) {
                continue
            }

            match MetaInstr::decode_arm(val) {
                Some(instr) if instr != target => {
                    panic!(
                        "instruction {:#034b} erroneously decoded as {:?}, expected {:?}",
                        val, instr, target
                    );
                }
                None => {
                    panic!(
                        "instruction {:#034b} failed to decode, expected {:?}",
                        val, target
                    );
                }
                _ => {}
            }
        }
    }

    fn test_decode_arm(target: MetaInstr, format: u32, mask: u32) {
        test_decode_arm_cond(target, format, mask, &|_| true);
    }

    #[test]
    fn decode_arm_data_processing() {
        let is_not_psr = |instr: u32| {
            let opcode = (instr >> 21) & 0xf;
            let s = (instr >> 20) & 1;
            !((0b1000 ..= 0b1011).contains(&opcode) && s == 0)
        };
        let reg_controlled_shift_needs_bit_7_low = |instr: u32| {
            let is_shift = (instr >> 25) & 1 == 0;
            let is_reg_controlled = (instr >> 4) & 1 == 1;
            let bit_7 = (instr >> 7) & 1;

            if is_shift && is_reg_controlled {
                bit_7 == 0
            } else {
                true
            }
        };
        let cond = |instr: u32| {
            is_not_psr(instr) && reg_controlled_shift_needs_bit_7_low(instr)
        };

        test_decode_arm_cond(
            MetaInstr::DataProcessing,
            0b0000_1100_0000_0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000_0000_0000_0000,
            &cond,
        );
    }

    #[test]
    fn decode_arm_multiply() {
        test_decode_arm(
            MetaInstr::Multiply,
            0b0000_1111_1100_0000_0000_0000_1111_0000,
            0b0000_0000_0000_0000_0000_0000_1001_0000,
        );
    }

    #[test]
    fn decode_arm_multiply_long() {
        test_decode_arm(
            MetaInstr::MultiplyLong,
            0b0000_1111_1000_0000_0000_0000_1111_0000,
            0b0000_0000_1000_0000_0000_0000_1001_0000,
        );
    }

    #[test]
    fn decode_arm_single_data_swap() {
        test_decode_arm(
            MetaInstr::SingleDataSwap,
            0b0000_1111_1011_0000_0000_1111_1111_0000,
            0b0000_0001_0000_0000_0000_0000_1001_0000,
        );
    }

    #[test]
    fn decode_arm_branch_and_exchange() {
        test_decode_arm(
            MetaInstr::BranchAndExchange,
            0b0000_1111_1111_1111_1111_1111_1111_0000,
            0b0000_0001_0010_1111_1111_1111_0001_0000,
        );
    }

    #[test]
    fn decode_arm_halfword_data_transfer_reg() {
        // SH can be 01, 10, or 11 but never 00 as that would make it a Single Data Swap instruction.
        let sh_non_zero = |x| (x >> 5) & 0b11u32 != 0u32;
        test_decode_arm_cond(
            MetaInstr::HalfwordTransReg,
            0b0000_1110_0100_0000_0000_1111_1001_0000,
            0b0000_0000_0000_0000_0000_0000_1001_0000,
            &sh_non_zero,
        );
    }

    #[test]
    fn decode_arm_halfword_data_transfer_imm() {
        // SH can be 01, 10, or 11 but never 00 as that would make it a Single Data Swap instruction.
        let sh_non_zero = |x| (x >> 5) & 0b11u32 != 0u32;
        test_decode_arm_cond(
            MetaInstr::HalfwordTransImm,
            0b0000_1110_0100_0000_0000_0000_1001_0000,
            0b0000_0000_0100_0000_0000_0000_1001_0000,
            &sh_non_zero,
        );
    }

    #[test]
    fn decode_arm_single_data_transfer() {
        let disambiguate_undefined = |x| (x >> 4) & 1u32 != 1u32;
        test_decode_arm_cond(
            MetaInstr::SingleDataTrans,
            0b0000_1110_0000_0000_0000_0000_0000_0000,
            0b0000_0110_0000_0000_0000_0000_0000_0000,
            &disambiguate_undefined,
        );
    }

    #[test]
    fn decode_arm_undefined() {
        test_decode_arm(
            MetaInstr::Undefined,
            0b0000_1110_0000_0000_0000_0000_0001_0000,
            0b0000_0110_0000_0000_0000_0000_0001_0000,
        );
    }

    #[test]
    fn decode_arm_block_data_transfer() {
        test_decode_arm(
            MetaInstr::BlockDataTrans,
            0b0000_1110_0000_0000_0000_0000_0000_0000,
            0b0000_1000_0000_0000_0000_0000_0000_0000,
        );
    }

    #[test]
    fn decode_arm_branch() {
        test_decode_arm(
            MetaInstr::Branch,
            0b0000_1110_0000_0000_0000_0000_0000_0000,
            0b0000_1010_0000_0000_0000_0000_0000_0000,
        );
    }

    #[test]
    fn decode_arm_coprocessor_data_transfer() {
        test_decode_arm(
            MetaInstr::CoprocDataTrans,
            0b0000_1110_0000_0000_0000_0000_0000_0000,
            0b0000_1100_0000_0000_0000_0000_0000_0000,
        );
    }

    #[test]
    fn decode_arm_coprocessor_data_operation() {
        test_decode_arm(
            MetaInstr::CoprocDataOp,
            0b0000_1111_0000_0000_0000_0000_0001_0000,
            0b0000_1110_0000_0000_0000_0000_0000_0000,
        );
    }

    #[test]
    fn decode_arm_coprocessor_register_transfer() {
        test_decode_arm(
            MetaInstr::CoprocRegTrans,
            0b0000_1111_0000_0000_0000_0000_0001_0000,
            0b0000_1110_0000_0000_0000_0000_0001_0000,
        );
    }

    #[test]
    fn decode_arm_software_interrupt() {
        test_decode_arm(
            MetaInstr::SoftwareInterrupt,
            0b0000_1111_0000_0000_0000_0000_0000_0000,
            0b0000_1111_0000_0000_0000_0000_0000_0000,
        );
    }

    #[test_log::test]
    fn decode_lsl() {
        let instr = MetaInstr::decode_arm( 0xe1a00c00 ).unwrap();
        assert_eq!(instr, MetaInstr::DataProcessing);
    }

    #[test_log::test]
    fn decode_psr_not_teq() {
        // This instruction was being decoded to DataProcessing (TEQ)
        // it should be LSL instead.
        let instr = MetaInstr::decode_arm( 0b1110_0011_0010_1000_1111_0001_0000_0000 ).unwrap();
        assert_eq!(instr, MetaInstr::PsrTransfer);
    }
}
