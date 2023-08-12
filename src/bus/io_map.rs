pub struct IoMap {
    mock: [u8; 0x400],
}

impl IoMap {
    pub fn new() -> Self {
        Self {
            mock: [0; 0x400]
        }
    }

    pub fn get(&self, index: usize) -> u32 {
        let index = index - 0x4000000;
        u32::from_le_bytes(self.mock[index .. index + 4].try_into().unwrap())
    }

    pub fn set(&mut self, index: usize, value: u32) {
        let index = index - 0x4000000;
        self.mock[index .. index + 4].clone_from_slice(&value.to_le_bytes());
    }
}
