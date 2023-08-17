#[derive(Debug)]
pub enum Targets {
    Arm,
    Thumb,
}

impl Targets {
    pub const fn value(&self) -> &'static str {
        match *self {
            Targets::Arm => "ARM instruction",
            Targets::Thumb => "THUMB instruction",
        }
    }
}
