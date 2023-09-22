pub enum Dispstat {
    VBlank,
    HBlank,
    VCount,
    VBlankIrq,
    HBlankIrq,
    VCountIrq,
}

impl Dispstat {
    fn bit(&self) -> usize {
        match *self {
            Dispstat::VBlank => 0,
            Dispstat::HBlank => 1,
            Dispstat::VCount => 2,
            Dispstat::VBlankIrq => 3,
            Dispstat::HBlankIrq => 4,
            Dispstat::VCountIrq => 5,
        }
    }
}

impl From<Dispstat> for usize {
    fn from(value: Dispstat) -> Self {
        value.bit()
    }
}
