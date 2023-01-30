// General Memory Map
// 0000-3FFF   16KB ROM Bank 00     (in cartridge, fixed at bank 00)
// 4000-7FFF   16KB ROM Bank 01..NN (in cartridge, switchable bank number)
// 8000-9FFF   8KB Video RAM (VRAM) (switchable bank 0-1 in CGB Mode)
// A000-BFFF   8KB External RAM     (in cartridge, switchable bank, if any)
// C000-CFFF   4KB Work RAM Bank 0 (WRAM)
// D000-DFFF   4KB Work RAM Bank 1 (WRAM)  (switchable bank 1-7 in CGB Mode)
// E000-FDFF   Same as C000-DDFF (ECHO)    (typically not used)
// FE00-FE9F   Sprite Attribute Table (OAM)
// FEA0-FEFF   Not Usable
// FF00-FF7F   I/O Ports
// FF80-FFFE   High RAM (HRAM)
// FFFF        Interrupt Enable Register
//

use crate::{memory::Memory, ppu::PPU};

pub const IO_REGISTERS_BEGIN: u16 = 0xFF00;
pub const IO_REGISTERS_END: u16 = 0xFF7F;

pub const OAM_BEGIN: u16 = 0xFE00;
pub const OAM_END: u16 = 0xFE9F;
pub const OAM_SIZE: u16 = OAM_END - OAM_BEGIN + 1;

pub const VRAM_BEGIN: u16 = 0x8000;
pub const VRAM_END: u16 = 0x9FFF;
pub const VRAM_SIZE: u16 = VRAM_END - VRAM_BEGIN + 1;

pub const WRAM_BEGIN: u16 = 0xC000;
pub const WRAM_END: u16 = 0xDFFF;
pub const WRAM_SIZE: u16 = WRAM_END - WRAM_BEGIN + 1;

pub struct MMU {
    ppu: PPU,
    wram: [u8; 0x2000],
}

impl Memory for MMU {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            WRAM_BEGIN..=WRAM_END => self.wram[address as usize],
            VRAM_BEGIN..=VRAM_END => self.ppu.read_byte(address),

            _ => 0,
        };
        self.wram[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.wram[address as usize] = value;
    }
}

impl MMU {
    pub fn new(ppu: PPU) -> Self {
        MMU {
            wram: [0; 0x2000],
            ppu: ppu,
        }
    }
}
