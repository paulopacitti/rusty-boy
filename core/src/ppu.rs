use crate::{
    memory::Memory,
    mmu::{MMU, OAM_BEGIN, OAM_END, VRAM_BEGIN, VRAM_END, VRAM_SIZE},
};

#[derive(Copy, Clone)]
enum Pixel {
    White,
    LightGray,
    DarkGray,
    Black,
}

impl Pixel {
    pub fn from_u8(value: u8) -> Pixel {
        match value {
            1 => Pixel::LightGray,
            2 => Pixel::DarkGray,
            3 => Pixel::Black,
            _ => Pixel::White,
        }
    }
}

pub struct PPU {
    oam: [u8; 0xA0], // "Object Attribute Memory", stores 40 sprites with 8x8 resolution.
    vram: [u8; 0x2000],
    scx: u8,
    scy: u8,
    wx: u8,
    wy: u8,
}

impl Memory for PPU {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            VRAM_BEGIN..=VRAM_END => self.vram[(address as usize) - 0x1FFF],
            OAM_BEGIN..=OAM_END => self.oam[(address as usize) - 0xFE00],

            _ => panic!("Unable to access this address from the PPU"),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            VRAM_BEGIN..=VRAM_END => self.vram[(address as usize) - 0x1FFF] = value,
            OAM_BEGIN..=OAM_END => self.oam[(address as usize) - 0xFE00] = value,

            _ => panic!("Unable to access this address from the PPU"),
        }
    }
}

impl PPU {
    pub fn new(&mut self) -> Self {
        PPU {
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            scx: 0,
            scy: 0,
            wx: 0,
            wy: 0,
        }
    }
}
