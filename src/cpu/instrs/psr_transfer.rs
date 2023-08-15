use crate::{cpu::Cpu, bus::Bus, utils::AddressableBits};

struct MsrImmFields {
    r: bool,
    field_mask: u32,
    operand: u32,
}

impl MsrImmFields {
    fn parse(instruction: u32, cpu: &Cpu) -> Self {
        let operand = if instruction.bit(25) == 1 {
            let rotate_imm = instruction.bits(8, 11);
            let imm = instruction.bits(0, 7);
            imm.rotate_right(2 * rotate_imm)
        } else {
            let rm = instruction.bits(0, 3);
            cpu.get_reg(rm as usize) 
        };

        Self {
            r: instruction.bit(22) == 1,
            field_mask: instruction.bits(16, 19),
            operand,
        }
    }
}

impl Cpu {
    fn mrs(&mut self, but: &mut Bus, instruction: u32) {
        todo!("MRS")
    }

    fn msr_reg(&mut self, but: &mut Bus, instruction: u32) {
        todo!("msr_reg")
    }

    fn msr_imm(&mut self, but: &mut Bus, instruction: u32) {
        let unalloc_mask = 0x07ffff00;
        let user_mask = 0xf8000000;
        let priv_mask = 0x0000000f;
        let state_mask = 0x00000020;

        let fields = MsrImmFields::parse(instruction, self);
        let field_mask = fields.field_mask;

        if (fields.operand & unalloc_mask != 0) {
            todo!("unpredictable")
        }

        let byte_mask: u32 = (if field_mask.bit(0) == 1 { 0xff } else { 0 }) |
            (if field_mask.bit(1) == 1 { 0xff00 } else { 0 }) |
            (if field_mask.bit(2) == 1 { 0xff0000 } else { 0 }) |
            (if field_mask.bit(3) == 1 { 0xff000000 } else { 0 });

        if !fields.r {
            let mask;
            println!("{:?}", self.mode);
            if self.in_privileged_mode() {
                todo!();
            } else {
                mask = byte_mask & user_mask;
            }
            self.cpsr = (self.cpsr & !mask) | (fields.operand & mask);
        } else {
            todo!("true")
        }
    }

    pub(super) fn psr_transfer(&mut self, bus: &mut Bus, instruction: u32) {
        if instruction.bit(21) == 0 {
            self.mrs(bus, instruction);
        } else {
            if instruction.bit(25) == 0 {
                self.msr_reg(bus, instruction);
            } else {
                self.msr_imm(bus, instruction);
            }
        }
    }
}
