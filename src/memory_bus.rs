pub(super) struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    /// Read a single byte from the Game Boy's memory.
    pub(super) fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    /// Write a single byte to the Game Boy's memory.
    pub(super) fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

impl Default for MemoryBus {
    fn default() -> Self {
        Self {
            memory: [0; 0xFFFF],
        }
    }
}
