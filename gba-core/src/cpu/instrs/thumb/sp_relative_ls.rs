use crate::utils::AddressableBits;

use super::ThumbInstruction;

struct Str;
struct Ldr;

pub fn decode(instruction: u16) -> Box<dyn ThumbInstruction> {
    let l = instruction.bit(11);
    if l == 0 {
        Box::new(Str)
    } else {
        Box::new(Ldr)
    }
}

impl ThumbInstruction for Str {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, bus: &mut crate::bus::Bus, instruction: u16) {
        let rd: u32 = instruction.bits(8, 10).into();
        let imm: u32 = instruction.bits(0, 7).into();

        let address = cpu.get_reg(13).wrapping_add(imm * 4);
        bus.write(address, cpu.get_reg(rd));
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rd: u32 = instruction.bits(8, 10).into();
        let imm: u32 = instruction.bits(0, 7).into();
        format!("STR r{}, [SP, {:x}]", rd, imm * 4)
    }
}

impl ThumbInstruction for Ldr {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, bus: &mut crate::bus::Bus, instruction: u16) {
        let rd: u32 = instruction.bits(8, 10).into();
        let imm: u32 = instruction.bits(0, 7).into();

        let address = cpu.get_reg(13).wrapping_add(imm * 4);
        let data = bus.read(address, cpu);
        cpu.set_reg(rd, data);
    }

    fn disassembly(&self, instruction: u16) -> String {
        let rd: u32 = instruction.bits(8, 10).into();
        let imm: u32 = instruction.bits(0, 7).into();
        format!("LDR r{}, [SP, {:x}]", rd, imm * 4)
    }
}
