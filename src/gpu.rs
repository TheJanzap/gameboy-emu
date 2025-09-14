//! The Game Boy groups pixels in 8x8 squares called Tiles. Tile data is stored between `0x8000` and
//! `0x97FF`, where two different tile sets are stored. The first tile set resides at `0x8000` to
//! `0x8FFF`, while the second occupies `0x8800` to `0x97FF` -- meaning the chunk between `0x8800`
//! to `0x8FFF` is shared by the two tile sets.

use crate::memory_map::OAM_SIZE;

pub(super) const VRAM_BEGIN: usize = 0x8000;
pub(super) const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;
const TILESET_STORAGE_END: usize = 0x1800;

/// Each tile stores a color index for each of its pixels, ranging from 0 to 3
#[derive(Copy, Clone)]
enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

type Tile = [[TilePixelValue; 8]; 8];

fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}

pub(super) struct GPU {
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; 384],
    /// The Object Attribute Memory (OAM) stores objects.
    /// These can be moved independently of the background.
    oam: [u8; OAM_SIZE],
}

impl Default for GPU {
    fn default() -> Self {
        Self {
            vram: [0; VRAM_SIZE],
            tile_set: [empty_tile(); 384],
            oam: [0; OAM_SIZE],
        }
    }
}

impl GPU {
    pub(super) fn read_vram(&self, address: usize) -> u8 {
        self.vram[address]
    }

    pub(super) fn write_vram(&mut self, address: usize, value: u8) {
        self.vram[address] = value;
        if address >= TILESET_STORAGE_END {
            return;
        }

        // Rows of tiles are encoded in two bytes. The first byte is always on an even address.
        // To get the actual index, we have to bitwise AND it with `0xFFFE` to get the first index.
        let normalized_address = address & 0xFFFE;
        let byte1 = self.vram[normalized_address];
        let byte2 = self.vram[normalized_address + 1];

        // A tile is 8 rows tall. Every row is encoded with two bytes. A tile is therefore 16 bytes
        // in total.
        let tile_index = address / 16;
        // Every two bytes is a new row.
        let row_index = (address % 16) / 2;

        // Loop 8 times to get the 8 pixels of a row.
        for pixel_index in 0..8 {
            // The pixels are indexed from the left instead of the right, so pixel 0 is index 7.
            // We first create a mask to get the specified pixel and `AND` it.
            let mask = 1 << (7 - pixel_index);
            let lsb = byte1 & mask;
            let msb = byte2 & mask;

            let value = match (lsb != 0, msb != 0) {
                (false, false) => TilePixelValue::Zero,
                (true, false) => TilePixelValue::One,
                (false, true) => TilePixelValue::Two,
                (true, true) => TilePixelValue::Three,
            };

            self.tile_set[tile_index][row_index][pixel_index] = value;
        }
    }

    pub(super) fn read_oam(&self, address: usize) -> u8 {
        self.oam[address]
    }

    pub(super) fn write_oam(&mut self, address: usize, value: u8) {
        todo!()
    }
}
