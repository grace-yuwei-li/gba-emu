use super::Mode;

#[derive(Debug, Default, Clone)]
pub struct Regs {
    sys_user: [u32; 16],
    fiq: [u32; 7],
    supervisor: [u32; 2],
    abort: [u32; 2],
    irq: [u32; 2],
    undefined: [u32; 2],

    pub cpsr: u32,
    pub spsr_svc: u32,
    pub spsr_abt: u32,
    pub spsr_und: u32,
    pub spsr_irq: u32,
    pub spsr_fiq: u32,
}

impl Regs {
    pub fn get(&self, reg: u32, mode: &Mode) -> u32 {
        assert!(reg <= 15);
        let reg: usize = reg.try_into().unwrap();
        match *mode {
            Mode::User | Mode::System => self.sys_user[reg],
            Mode::FIQ if (8 <= reg && reg <= 14) => self.fiq[reg - 8],
            Mode::Supervisor if (13 <= reg && reg <= 14) => self.supervisor[reg - 13],
            Mode::Abort if (13 <= reg && reg <= 14) => self.abort[reg - 13],
            Mode::IRQ if (13 <= reg && reg <= 14) => self.irq[reg - 13],
            Mode::Undefined if (13 <= reg && reg <= 14) => self.undefined[reg - 13],
            _ => self.sys_user[reg],
        }
    }

    pub fn get_mut(&mut self, reg: u32, mode: &Mode) -> &mut u32 {
        assert!(reg <= 15);
        let reg: usize = reg.try_into().unwrap();
        match *mode {
            Mode::User | Mode::System => &mut self.sys_user[reg],
            Mode::FIQ if 8 <= reg && reg <= 14 => &mut self.fiq[reg - 8],
            Mode::Supervisor if 13 <= reg && reg <= 14 => &mut self.supervisor[reg - 13],
            Mode::Abort if 13 <= reg && reg <= 14 => &mut self.abort[reg - 13],
            Mode::IRQ if 13 <= reg && reg <= 14 => &mut self.irq[reg - 13],
            Mode::Undefined if 13 <= reg && reg <= 14 => &mut self.undefined[reg - 13],
            _ => &mut self.sys_user[reg],
        }
    }

    pub fn pc(&self) -> u32 {
        self.sys_user[15]
    }

    pub fn pc_mut(&mut self) -> &mut u32 {
        &mut self.sys_user[15]
    }

    pub fn spsr(&self, mode: &Mode) -> u32 {
        match *mode {
            Mode::User | Mode::System => self.cpsr,
            Mode::Supervisor => self.spsr_svc,
            Mode::Abort => self.spsr_abt,
            Mode::Undefined => self.spsr_und,
            Mode::IRQ => self.spsr_irq,
            Mode::FIQ => self.spsr_fiq,
        }
    }

    pub fn spsr_mut(&mut self, mode: &Mode) -> &mut u32 {
        match *mode {
            Mode::User | Mode::System => &mut self.cpsr,
            Mode::Supervisor => &mut self.spsr_svc,
            Mode::Abort => &mut self.spsr_abt,
            Mode::Undefined => &mut self.spsr_und,
            Mode::IRQ => &mut self.spsr_irq,
            Mode::FIQ => &mut self.spsr_fiq,
        }
    }
}
