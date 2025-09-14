pub const BOOT_ROM_START: usize = 0x0000;
pub const BOOT_ROM_END: usize = 0x00FF;
pub const BOOT_ROM_SIZE: usize = BOOT_ROM_END - BOOT_ROM_START + 1;

pub const GAME_ROM_BANK_0_START: usize = 0x0000;
pub const GAME_ROM_BANK_0_END: usize = 0x03FFF;
pub const GAME_ROM_BANK_0_SIZE: usize = GAME_ROM_BANK_0_END - GAME_ROM_BANK_0_START;

pub const GAME_ROM_BANK_N_START: usize = 0x4000;
pub const GAME_ROM_BANK_N_END: usize = 0x7FFF;
pub const GAME_ROM_BANK_N_SIZE: usize = GAME_ROM_BANK_N_END - GAME_ROM_BANK_N_START;

pub const TILE_RAM_START: usize = 0x8000;
pub const TILE_RAM_END: usize = 0x97FF;
pub const TILE_RAM_SIZE: usize = TILE_RAM_END - TILE_RAM_START + 1;

pub const BACKGROUND_MAP_START: usize = 0x9800;
pub const BACKGROUND_MAP_END: usize = 0x9FFF;
pub const BACKGROUND_MAP_SIZE: usize = BACKGROUND_MAP_END - BACKGROUND_MAP_START + 1;

pub const CARTRIDGE_RAM_START: usize = 0xA000;
pub const CARTRIDGE_RAM_END: usize = 0xBFFF;
pub const CARTRIDGE_RAM_SIZE: usize = CARTRIDGE_RAM_END - CARTRIDGE_RAM_START + 1;

pub const WORKING_RAM_START: usize = 0xC000;
pub const WORKING_RAM_END: usize = 0xDFFF;
pub const WORKING_RAM_SIZE: usize = WORKING_RAM_END - WORKING_RAM_START + 1;

pub const ECHO_RAM_START: usize = 0xE000;
pub const ECHO_RAM_END: usize = 0xFDFF;

pub const OAM_START: usize = 0xFE00;
pub const OAM_END: usize = 0xFE9F;
pub const OAM_SIZE: usize = OAM_END - OAM_START + 1;

pub const UNUSED_MEMORY_START: usize = 0xFEA0;
pub const UNUSED_MEMORY_END: usize = 0xFEFF;

pub const IO_REGISTER_START: usize = 0xFF00;
pub const IO_REGISTER_END: usize = 0xFF7F;

pub const HIGH_RAM_START: usize = 0xFF80;
pub const HIGH_RAM_END: usize = 0xFFFE;
pub const HIGH_RAM_SIZE: usize = HIGH_RAM_END - HIGH_RAM_START + 1;

pub const INTERRUPT_ENABLE_REGISTER: usize = 0xFFFF;
