use super::ArmInstruction;

pub struct Multiply;

impl ArmInstruction for Multiply {
    fn execute(&self, cpu: &mut crate::cpu::Cpu, bus: &mut crate::bus::Bus, instruction: u32) {
        // Do the wrong thing
    }

    fn disassembly(&self, instruction: u32) -> String {
        format!("MUL")
    }
}
