mod bus;
mod cpu;
mod ppu;
mod utils;

pub use utils::logging;

use bus::Bus;
use cpu::Cpu;

pub struct GbaCore {
    cpu: Cpu,
    bus: Bus,
}

impl Default for GbaCore {
    fn default() -> Self {
        pretty_env_logger::init();
        Self {
            cpu: Cpu::default(),
            bus: Bus::default(),
        }
    }
}

impl GbaCore {
    pub fn load_rom(&mut self, bytes: &[u8]) {
        self.bus.load_rom(bytes)
    }

    pub fn skip_bios(&mut self) {
        self.cpu.skip_bios(&self.bus);
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.bus)
    }

    pub fn regs(&mut self) -> Vec<u32> {
        self.cpu.get_all_regs()
    }
}
