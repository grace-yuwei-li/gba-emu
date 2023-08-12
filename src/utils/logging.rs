#[derive(Debug)]
pub enum Targets {
    Instr
}

impl Targets {
    pub const fn value(&self) -> &'static str {
        match *self {
            Targets::Instr => "instruction"
        }
    }
}
