use super::ThumbInstruction;
use crate::cpu::Bus;
use crate::cpu::Cpu;
use crate::utils::AddressableBits;

struct AddPc;
struct AddSp;

pub fn decode(instruction: u16) -> Box<dyn ThumbInstruction> {
    if instruction.bit(11) == 0 {
        Box::new(AddPc)
    } else {
        Box::new(AddSp)
    }
}

impl ThumbInstruction for AddPc {
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, instruction: u16) {
        let rd = instruction.bits(8, 10);
        let imm: u32 = instruction.bits(0, 7).into();
        let value = (cpu.get_reg(15) & 0xfffffffc).wrapping_add(imm * 4);
        cpu.set_reg(rd.into(), value);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rd = instruction.bits(8, 10);
        let imm: u32 = instruction.bits(0, 7).into();
        format!("ADD r{}, PC, {:x}", rd, imm * 4)
    }
}

impl ThumbInstruction for AddSp {
    fn execute(&self, cpu: &mut Cpu, _: &mut Bus, instruction: u16) {
        let rd = instruction.bits(8, 10);
        let imm: u32 = instruction.bits(0, 7).into();
        let value = cpu.get_reg(13).wrapping_add(imm * 4);
        cpu.set_reg(rd.into(), value);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rd = instruction.bits(8, 10);
        let imm: u32 = instruction.bits(0, 7).into();
        format!("ADD r{}, SP, {:x}", rd, imm * 4)
    }
}
