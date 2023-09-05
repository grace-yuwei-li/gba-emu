use crate::{
    bus::Bus,
    cpu::{Cpu, CPSR},
    utils::{add_overflows, sub_overflows, AddressableBits},
};

use super::ThumbInstruction;

struct AND;
struct EOR;
struct LSL;
struct LSR;
struct ASR;
struct ADC;
struct SBC;
struct ROR;
struct TST;
struct NEG;
struct CMP;
struct CMN;
struct ORR;
struct MUL;
struct BIC;
struct MVN;

struct FlagUpdates {
    n: Option<bool>,
    z: Option<bool>,
    c: Option<bool>,
    v: Option<bool>,
}

impl FlagUpdates {
    fn default_nz(result: u32) -> Self {
        Self {
            n: Some(result.bit(31) == 1),
            z: Some(result == 0),
            c: None,
            v: None,
        }
    }
}

pub fn decode(instruction: u16) -> Box<dyn ThumbInstruction> {
    match instruction.bits(6, 9) {
        0b0000 => Box::new(AND),
        0b0001 => Box::new(EOR),
        0b0010 => Box::new(LSL),
        0b0011 => Box::new(LSR),
        0b0100 => Box::new(ASR),
        0b0101 => Box::new(ADC),
        0b0110 => Box::new(SBC),
        0b0111 => Box::new(ROR),
        0b1000 => Box::new(TST),
        0b1001 => Box::new(NEG),
        0b1010 => Box::new(CMP),
        0b1011 => Box::new(CMN),
        0b1100 => Box::new(ORR),
        0b1101 => Box::new(MUL),
        0b1110 => Box::new(BIC),
        0b1111 => Box::new(MVN),
        _ => unreachable!(),
    }
}

macro_rules! alu_thumb_instr_impl {
    ($SelfT:ty, $Op:literal, $Closure:expr) => {
        impl ThumbInstruction for $SelfT {
            fn execute(&self, cpu: &mut Cpu, _: &mut Bus, instruction: u16) {
                execute_op(cpu, instruction, $Closure);
            }

            fn disassembly(&self, instruction: u16) -> String {
                let rs = instruction.bits(3, 5);
                let rd = instruction.bits(0, 2);
                format!("{} r{}, r{}", $Op, rd, rs)
            }
        }
    };
}

alu_thumb_instr_impl!(AND, "AND", |op1, op2, _| {
    let result = op1 & op2;
    (Some(result), FlagUpdates::default_nz(result))
});

alu_thumb_instr_impl!(EOR, "EOR", |op1, op2, _| {
    let result = op1 ^ op2;
    (Some(result), FlagUpdates::default_nz(result))
});

alu_thumb_instr_impl!(LSL, "LSL", |op1, op2, _| {
    let rs_low = op2.bits(0, 7);
    let result;
    let carry;
    if rs_low == 0 {
        // Nothing
        result = op1;
        carry = None;
    } else if rs_low < 32 {
        result = op1 << rs_low;
        carry = Some(op1.bit((32 - rs_low).try_into().unwrap()) == 1);
    } else if rs_low == 32 {
        result = 0;
        carry = Some(op1.bit(0) == 1);
    } else {
        result = 0;
        carry = Some(false);
    }
    (
        Some(result),
        FlagUpdates {
            c: carry,
            ..FlagUpdates::default_nz(result)
        },
    )
});

alu_thumb_instr_impl!(LSR, "LSR", |op1, op2, _| {
    let rs_low = op2.bits(0, 7);
    let result;
    let carry;
    if rs_low == 0 {
        result = op1;
        carry = None;
    } else if rs_low < 32 {
        result = op1 >> rs_low;
        carry = Some(op1.bit((rs_low - 1).try_into().unwrap()) == 1);
    } else if rs_low == 32 {
        result = 0;
        carry = Some(op1.bit(31) == 1);
    } else {
        result = 0;
        carry = Some(false);
    }
    (
        Some(result),
        FlagUpdates {
            c: carry,
            ..FlagUpdates::default_nz(result)
        },
    )
});

alu_thumb_instr_impl!(ASR, "ASR", |op1, op2, _| {
    let rs_low = op2.bits(0, 7);
    let result;
    let carry;
    if rs_low == 0 {
        result = op1;
        carry = None;
    } else if rs_low < 32 {
        result = ((op1 as i32) >> rs_low) as u32;
        carry = Some(op1.bit((rs_low - 1).try_into().unwrap()) == 1);
    } else {
        result = if op1.bit(31) == 0 { 0 } else { 0xffffffff };
        carry = Some(op1.bit(31) == 1);
    }
    (
        Some(result),
        FlagUpdates {
            c: carry,
            ..FlagUpdates::default_nz(result)
        },
    )
});

alu_thumb_instr_impl!(ADC, "ADC", |op1, op2, c| {
    let (result1, carry1) = op1.overflowing_add(op2);
    let (result2, carry2) = result1.overflowing_add(c);

    (
        Some(result2),
        FlagUpdates {
            c: Some(carry1 | carry2),
            v: Some(add_overflows(op1, op2, result1) | add_overflows(result1, c, result2)),
            ..FlagUpdates::default_nz(result2)
        },
    )
});

alu_thumb_instr_impl!(SBC, "SBC", |op1, op2, c| {
    let not_c = 1 - c;
    let (result1, borrow1) = op1.overflowing_sub(op2);
    let (result2, borrow2) = result1.overflowing_sub(not_c);

    (
        Some(result2),
        FlagUpdates {
            c: Some(!(borrow1 | borrow2)),
            v: Some(sub_overflows(op1, op2, result1) | sub_overflows(result1, not_c, result2)),
            ..FlagUpdates::default_nz(result2)
        },
    )
});

alu_thumb_instr_impl!(ROR, "ROR", |op1, op2, _| {
    let rs_low = op2.bits(0, 7);
    let rs_lower = op2.bits(0, 4);
    let result;
    let carry;
    if rs_low == 0 {
        // Nothing
        result = op1;
        carry = None;
    } else if rs_lower == 0 {
        result = op1;
        carry = Some(op1.bit(31) == 1);
    } else {
        result = op1.rotate_right(rs_lower);
        carry = Some(op1.bit((rs_lower - 1).try_into().unwrap()) == 1);
    }

    (
        Some(result),
        FlagUpdates {
            c: carry,
            ..FlagUpdates::default_nz(result)
        },
    )
});

alu_thumb_instr_impl!(TST, "TST", |op1, op2, _| {
    let result = op1 & op2;
    (None, FlagUpdates::default_nz(result))
});

alu_thumb_instr_impl!(NEG, "NEG", |_, op2, _| {
    let (result, borrow) = 0u32.overflowing_sub(op2);
    (
        Some(result),
        FlagUpdates {
            c: Some(!borrow),
            v: Some(sub_overflows(0, op2, result)),
            ..FlagUpdates::default_nz(result)
        },
    )
});

alu_thumb_instr_impl!(CMP, "CMP", |op1, op2, _| {
    let (result, borrow) = op1.overflowing_sub(op2);
    (
        None,
        FlagUpdates {
            c: Some(!borrow),
            v: Some(sub_overflows(op1, op2, result)),
            ..FlagUpdates::default_nz(result)
        },
    )
});

alu_thumb_instr_impl!(CMN, "CMN", |op1, op2, _| {
    let (result, carry) = op1.overflowing_add(op2);
    (
        None,
        FlagUpdates {
            c: Some(carry),
            v: Some(add_overflows(op1, op2, result)),
            ..FlagUpdates::default_nz(result)
        },
    )
});

alu_thumb_instr_impl!(ORR, "ORR", |op1, op2, _| {
    let result = op1 | op2;
    (Some(result), FlagUpdates::default_nz(result))
});

alu_thumb_instr_impl!(MUL, "MUL", |op1, op2, _| {
    let result = op1.wrapping_mul(op2);
    // TODO: MUL's C flag is unpredictable in v4
    (Some(result), FlagUpdates::default_nz(result))
});

alu_thumb_instr_impl!(BIC, "BIC", |op1, op2, _| {
    let result = op1 & !op2;
    (Some(result), FlagUpdates::default_nz(result))
});

alu_thumb_instr_impl!(MVN, "MVN", |_, op2, _| {
    let result = !op2;
    (Some(result), FlagUpdates::default_nz(result))
});

#[inline]
fn execute_op<F>(cpu: &mut Cpu, instruction: u16, op_closure: F)
where
    // op1, op2, c_flag -> result, flag_updates
    F: Fn(u32, u32, u32) -> (Option<u32>, FlagUpdates),
{
    let rs = instruction.bits(3, 5);
    let rd = instruction.bits(0, 2);

    let op1 = cpu.get_reg(rd.into());
    let op2 = cpu.get_reg(rs.into());

    let (result, flags) = op_closure(op1, op2, cpu.get_cpsr_bit(CPSR::C));

    if let Some(b) = flags.n {
        cpu.set_flag(CPSR::N, b);
    }
    if let Some(b) = flags.z {
        cpu.set_flag(CPSR::Z, b);
    }
    if let Some(b) = flags.c {
        cpu.set_flag(CPSR::C, b);
    }
    if let Some(b) = flags.v {
        cpu.set_flag(CPSR::V, b);
    }

    if let Some(result) = result {
        cpu.set_reg(rd.into(), result);
        // Impossible for rd to be 15 since rd is only 3 bits wide
        // Therefore we don't need check and flush the pipeline
    }
}
