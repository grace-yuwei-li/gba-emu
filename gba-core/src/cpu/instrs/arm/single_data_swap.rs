use crate::utils::AddressableBits;

use super::ArmInstruction;

pub struct Swp;
pub struct Swpb;

impl ArmInstruction for Swp {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, bus: &mut crate::bus::Bus, instruction: u32) {
        let rn = instruction.bits(16, 19);
        let rd = instruction.bits(12, 15);
        let rm = instruction.bits(0, 3);

        let address = cpu.get_reg(rn);
        let temp = bus.read(address, cpu);
        bus.write(address, cpu.get_reg(rm));
        cpu.set_reg(rd, temp);
    }

    fn disassembly(&self, instruction: u32) -> String {
        let rn = instruction.bits(16, 19);
        let rd = instruction.bits(12, 15);
        let rm = instruction.bits(0, 3);

        format!("SWP r{}, r{}, [r{}]", rd, rm, rn)
    }
}

impl ArmInstruction for Swpb {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, bus: &mut crate::bus::Bus, instruction: u32) {
        let rn = instruction.bits(16, 19);
        let rd = instruction.bits(12, 15);
        let rm = instruction.bits(0, 3);

        let address = cpu.get_reg(rn);
        let temp = bus.read_byte(address, cpu);
        bus.write_byte(address, cpu.get_reg(rm) as u8);
        cpu.set_reg(rd, temp.into());
    }

    fn disassembly(&self, instruction: u32) -> String {
        let rn = instruction.bits(16, 19);
        let rd = instruction.bits(12, 15);
        let rm = instruction.bits(0, 3);

        format!("SWPB r{}, r{}, [r{}]", rd, rm, rn)
    }
}

pub fn decode_swap(instruction: u32) -> Box<dyn ArmInstruction> {
    if instruction.bit(22) == 0 {
        Box::new(Swp)
    } else {
        Box::new(Swpb)
    }
}
