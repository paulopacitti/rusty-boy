/// Flags = (Zero flag, Subtraction flag (BCD), Half Carry flag (BCD), Carry flag)
pub struct Flags(pub u8);

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: Flags,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16, // holds 16bit values
}

impl Flags {
    pub fn new() -> Self {
        Flags(0x00)
    }

    /// Get zero flag.
    pub fn z(&self) -> bool {
        (self.0 >> 7) & 1 != 0
    }

    /// Get subtraction flag (BCD).
    pub fn n(&self) -> bool {
        (self.0 >> 6) & 1 != 0
    }

    /// Get half-carry flag (BCD).
    pub fn h(&self) -> bool {
        (self.0 >> 5) & 1 != 0
    }

    /// Get carry flag.
    pub fn c(&self) -> bool {
        (self.0 >> 4) & 1 != 0
    }

    /// Set zero flag.
    pub fn set_z(&mut self, value: bool) {
        self.0 = (self.0 & !(1 << 7)) | (value as u8) << 7;
    }

    /// Set subtraction flag (BCD).
    pub fn set_n(&mut self, value: bool) {
        self.0 = (self.0 & !(1 << 6)) | (value as u8) << 6;
    }

    /// Set half-carry flag (BCD).
    pub fn set_h(&mut self, value: bool) {
        self.0 = (self.0 & !(1 << 5)) | (value as u8) << 5;
    }

    /// Set carry flag.
    pub fn set_c(&mut self, value: bool) {
        self.0 = (self.0 & !(1 << 4)) | (value as u8) << 4;
    }
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            pc: 0,
            sp: 0,
            a: 0,
            f: Flags::new(),
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
        }
    }

    /// Get value of the 16 bit register using A and F.
    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | (self.f.0 as u16)
    }

    /// Get value of the 16 bit register using B and C.
    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    /// Get value of the 16 bit register using D and E.
    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    /// Get value of the 16 bit register using H and L.
    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    /// Sets a 16 bit register using A and F.
    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f.0 = (value & 0xF0) as u8; // only most significant bits are used in F
    }

    /// Sets a 16 bit register using B and C.
    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    /// Sets a 16 bit register using D and E.
    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    /// Sets a 16 bit register using H and L.
    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}
