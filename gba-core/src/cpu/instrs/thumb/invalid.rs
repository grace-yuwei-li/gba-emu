use super::ThumbInstruction;

pub struct Invalid;

impl ThumbInstruction for Invalid {
    fn execute(&self, _: &mut crate::cpu::Cpu, _: &mut crate::bus::Bus, instruction: u16) {
        panic!("Executed invalid thumb instruction {:018b}", instruction);
    }

    fn disassembly(&self, _: u16) -> String {
        format!("Invalid")
    }
}
