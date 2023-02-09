use crate::{
    memory::Memory,
    mmu::{OAM_BEGIN, OAM_END, VRAM_BEGIN, VRAM_END},
};

struct LCDC {
    data: u8,
}

impl LCDC {
    pub fn new() -> Self {
        LCDC { data: 0 }
    }

    fn bit7(&self) -> bool {
        self.data & 0b1000_0000 != 0x00
    }

    fn bit6(&self) -> bool {
        self.data & 0b0100_0000 != 0x00
    }

    fn bit5(&self) -> bool {
        self.data & 0b0010_0000 != 0x00
    }

    fn bit4(&self) -> bool {
        self.data & 0b0001_0000 != 0x00
    }

    fn bit3(&self) -> bool {
        self.data & 0b0000_1000 != 0x00
    }

    fn bit2(&self) -> bool {
        self.data & 0b0000_0100 != 0x00
    }

    fn bit1(&self) -> bool {
        self.data & 0b0000_0010 != 0x00
    }

    fn bit0(&self) -> bool {
        self.data & 0b0000_0001 != 0x00
    }
}

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

// LCD Status Register.
pub struct Stat {
    // Bit 6 - LYC=LY Coincidence Interrupt (1=Enable) (Read/Write)
    enable_ly_interrupt: bool,
    // Bit 5 - Mode 2 OAM Interrupt         (1=Enable) (Read/Write)
    enable_m2_interrupt: bool,
    // Bit 4 - Mode 1 V-Blank Interrupt     (1=Enable) (Read/Write)
    enable_m1_interrupt: bool,
    // Bit 3 - Mode 0 H-Blank Interrupt     (1=Enable) (Read/Write)
    enable_m0_interrupt: bool,
    // Bit 1-0 - Mode Flag       (Mode 0-3, see below) (Read Only)
    //    0: During H-Blank
    //    1: During V-Blank
    //    2: During Searching OAM
    //    3: During Transferring Data to LCD Driver
    mode: u8,
}

impl Stat {
    pub fn new() -> Self {
        Self {
            enable_ly_interrupt: false,
            enable_m2_interrupt: false,
            enable_m1_interrupt: false,
            enable_m0_interrupt: false,
            mode: 0x00,
        }
    }
}

pub struct PPU {
    oam: [u8; 0xA0], // "Object Attribute Memory", stores 40 sprites with 8x8 resolution.
    vram: [u8; 0x2000],

    lcdc: LCDC,
    stat: Stat,

    bgp: u8,  // BG palette data
    obp0: u8, // OBJ palette 0 data
    obp1: u8, // OBJ palette 1 data

    ly: u8,
    lyc: u8,
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
            0xFF40 => self.lcdc.data,
            0xFF41 => {
                let bit6 = if self.stat.enable_ly_interrupt {
                    0x40
                } else {
                    0x00
                };
                let bit5 = if self.stat.enable_m2_interrupt {
                    0x20
                } else {
                    0x00
                };
                let bit4 = if self.stat.enable_m1_interrupt {
                    0x10
                } else {
                    0x00
                };
                let bit3 = if self.stat.enable_m0_interrupt {
                    0x08
                } else {
                    0x00
                };
                let bit2 = if self.ly == self.lyc { 0x04 } else { 0x00 };
                bit6 | bit5 | bit4 | bit3 | bit2 | self.stat.mode
            }
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => 0, // write-only
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wx,
            0xFF4B => self.wy,

            _ => panic!("Unable to read from this address from the PPU"),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            VRAM_BEGIN..=VRAM_END => self.vram[(address as usize) - 0x1FFF] = value,
            OAM_BEGIN..=OAM_END => self.oam[(address as usize) - 0xFE00] = value,
            0xFF40 => self.lcdc.data = value,
            0xFF41 => {
                // Mode and LYC=LY are read-only
                self.stat.enable_ly_interrupt = value & 0x40 != 0x00;
                self.stat.enable_m2_interrupt = value & 0x20 != 0x00;
                self.stat.enable_m1_interrupt = value & 0x10 != 0x00;
                self.stat.enable_m0_interrupt = value & 0x08 != 0x00;
            }
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => {} // ready-only
            0xFF45 => self.lyc = value,
            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,

            _ => panic!("Unable to write to this address from the PPU"),
        }
    }
}

impl PPU {
    pub fn new(&mut self) -> Self {
        PPU {
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            lcdc: LCDC::new(),
            stat: Stat::new(),
            bgp: 0,
            obp0: 0,
            obp1: 0,
            ly: 0,
            lyc: 0,
            scx: 0,
            scy: 0,
            wx: 0,
            wy: 0,
        }
    }

    fn draw_bg(&mut self) {}
    fn draw_sprites(&mut self) {}
}
