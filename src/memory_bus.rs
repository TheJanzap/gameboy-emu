use crate::memory_map::*;
use super::gpu::{GPU, VRAM_BEGIN, VRAM_END};

pub(super) struct MemoryBus {
    /// The boot ROM of the emulator. Gets unloaded after the code from the cartridge has been loaded.
    boot_rom: Option<[u8; BOOT_ROM_SIZE]>,
    rom_bank_0: [u8; GAME_ROM_BANK_0_SIZE],
    rom_bank_n: [u8; GAME_ROM_BANK_N_SIZE],
    cartridge_ram: [u8; CARTRIDGE_RAM_SIZE],
    working_ram: [u8; WORKING_RAM_SIZE],
    high_ram: [u8; HIGH_RAM_SIZE],
    gpu: GPU,
}

impl Default for MemoryBus {
    fn default() -> Self {
        Self {
            boot_rom: Some([0; BOOT_ROM_SIZE]),
            rom_bank_0: [0; GAME_ROM_BANK_0_SIZE],
            rom_bank_n: [0; GAME_ROM_BANK_N_SIZE],
            cartridge_ram: [0; CARTRIDGE_RAM_SIZE],
            working_ram: [0; WORKING_RAM_SIZE],
            high_ram: [0; HIGH_RAM_SIZE],
            gpu: GPU::default(),
        }
    }
}

impl MemoryBus {
    /// Read a single byte from the Game Boy's memory.
    pub(super) fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            BOOT_ROM_START..=BOOT_ROM_END => {
                if let Some(boot_rom) = self.boot_rom {
                    boot_rom[address]
                } else {
                    self.rom_bank_0[address]
                }
            }
            GAME_ROM_BANK_0_START..=GAME_ROM_BANK_0_END => self.rom_bank_0[address],
            GAME_ROM_BANK_N_START..=GAME_ROM_BANK_N_END => self.rom_bank_n[address],
            VRAM_BEGIN..=VRAM_END => self.gpu.read_vram(address),
            CARTRIDGE_RAM_START..=CARTRIDGE_RAM_END => {
                self.cartridge_ram[address - CARTRIDGE_RAM_START]
            }
            WORKING_RAM_START..=WORKING_RAM_END => self.working_ram[address - WORKING_RAM_START],
            ECHO_RAM_START..=ECHO_RAM_END => self.working_ram[address - ECHO_RAM_START],
            OAM_START..=OAM_END => self.gpu.read_oam(address - OAM_START),
            IO_REGISTER_START..=IO_REGISTER_END => {
                self.read_io_register(address - IO_REGISTER_START)
            }
            UNUSED_MEMORY_START..=UNUSED_MEMORY_END => 0,
            HIGH_RAM_START..=HIGH_RAM_END => self.high_ram[address - HIGH_RAM_START],
            INTERRUPT_ENABLE_REGISTER => self.interrupt_enable(),
            _ => unreachable!("Memory address out of bounds: 0x{:x}", address),
        }
    }

    /// Write a single byte to the Game Boy's memory.
    pub(super) fn write_byte(&mut self, address: u16, value: u8) {
        let address = address as usize;
        match address {
            GAME_ROM_BANK_0_START..=GAME_ROM_BANK_0_END => self.rom_bank_0[address] = value,
            VRAM_BEGIN..=VRAM_END => self.gpu.write_vram(address - VRAM_BEGIN, value),
            CARTRIDGE_RAM_START..=CARTRIDGE_RAM_END => {
                self.cartridge_ram[address - CARTRIDGE_RAM_START] = value
            }
            WORKING_RAM_START..=WORKING_RAM_END => {
                self.working_ram[address - WORKING_RAM_START] = value
            }
            OAM_START..=OAM_END => self.gpu.write_oam(address - OAM_START, value),
            IO_REGISTER_START..=IO_REGISTER_END => {
                self.write_io_register(address - IO_REGISTER_START, value)
            }
            UNUSED_MEMORY_START..=UNUSED_MEMORY_END => (),
            HIGH_RAM_START..=HIGH_RAM_END => self.high_ram[address - HIGH_RAM_START] = value,
            INTERRUPT_ENABLE_REGISTER => todo!(),
            _ => unreachable!("Memory address out of bounds: 0x{:x}", address),
        }
    }

    fn read_io_register(&self, address: usize) -> u8 {
        todo!()
    }

    fn write_io_register(&mut self, address: usize, value: u8) {
        todo!()
    }

    fn interrupt_enable(&self) -> u8 {
        todo!()
    }
}
