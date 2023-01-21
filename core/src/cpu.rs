use crate::mmu::Memory;

mod instructions;
mod registers;

pub struct CPU {
    registers: registers::Registers,
    mmu: Memory,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: registers::Registers::new(),
            mmu: Memory::new(),
        }
    }

    fn execute(&mut self, op: u8) {
        match op {
            0x00 => self.nop(),
            0x04 => self.registers.b = self.inc(self.registers.b),
            0x05 => self.registers.b = self.dec(self.registers.b),
            0x0C => self.registers.c = self.inc(self.registers.c),
            0x0D => self.registers.c = self.dec(self.registers.c),
            0x14 => self.registers.d = self.inc(self.registers.d),
            0x15 => self.registers.d = self.dec(self.registers.d),
            0x1C => self.registers.e = self.inc(self.registers.e),
            0x1D => self.registers.e = self.dec(self.registers.e),
            0x24 => self.registers.h = self.inc(self.registers.h),
            0x25 => self.registers.h = self.dec(self.registers.h),
            0x2C => self.registers.l = self.inc(self.registers.l),
            0x2D => self.registers.l = self.dec(self.registers.l),
            0x34 => {
                let address = self.registers.hl();
                let result = self.inc(self.mmu.read_byte(address));
                self.mmu.write_byte(address, result);
            }
            0x35 => {
                let address = self.registers.hl();
                let result = self.dec(self.mmu.read_byte(address));
                self.mmu.write_byte(address, result);
            }
            0x3C => self.registers.a = self.inc(self.registers.a),
            0x3D => self.registers.a = self.dec(self.registers.a),
            0x80 => self.add(self.registers.b),
            0x81 => self.add(self.registers.c),
            0x82 => self.add(self.registers.d),
            0x83 => self.add(self.registers.e),
            0x84 => self.add(self.registers.h),
            0x85 => self.add(self.registers.l),
            0x86 => {
                let address = self.registers.hl();
                self.add(self.mmu.read_byte(address));
            }
            0x87 => self.add(self.registers.a),
            0x88 => self.adc(self.registers.b),
            0x89 => self.adc(self.registers.c),
            0x8A => self.adc(self.registers.d),
            0x8B => self.adc(self.registers.e),
            0x8C => self.adc(self.registers.h),
            0x8D => self.adc(self.registers.l),
            0x8E => {
                let address = self.registers.hl();
                self.adc(self.mmu.read_byte(address));
            }
            0x8F => self.adc(self.registers.a),
            0x90 => self.sub(self.registers.b),
            0x91 => self.sub(self.registers.c),
            0x92 => self.sub(self.registers.d),
            0x93 => self.sub(self.registers.e),
            0x94 => self.sub(self.registers.h),
            0x95 => self.sub(self.registers.l),
            0x96 => {
                let address = self.registers.hl();
                self.sub(self.mmu.read_byte(address));
            }
            0x97 => self.sub(self.registers.a),
            0x98 => self.sub(self.registers.b),
            0x99 => self.sub(self.registers.c),
            0x9A => self.sub(self.registers.d),
            0x9B => self.sub(self.registers.e),
            0x9C => self.sub(self.registers.h),
            0x9D => self.sub(self.registers.l),
            0x9E => {
                let address = self.registers.hl();
                self.sub(self.mmu.read_byte(address));
            }
            0x9F => self.sub(self.registers.a),
            0xA0 => self.and(self.registers.b),
            0xA1 => self.and(self.registers.c),
            0xA2 => self.and(self.registers.d),
            0xA3 => self.and(self.registers.e),
            0xA4 => self.and(self.registers.h),
            0xA5 => self.and(self.registers.l),
            0xA6 => {
                let address = self.registers.hl();
                self.and(self.mmu.read_byte(address));
            }
            0xA7 => self.and(self.registers.a),
            0xA8 => self.xor(self.registers.b),
            0xA9 => self.xor(self.registers.c),
            0xAA => self.xor(self.registers.d),
            0xAB => self.xor(self.registers.e),
            0xAC => self.xor(self.registers.h),
            0xAD => self.xor(self.registers.l),
            0xAE => {
                let address = self.registers.hl();
                self.xor(self.mmu.read_byte(address));
            }
            0xAF => self.xor(self.registers.a),
            0xB0 => self.or(self.registers.b),
            0xB1 => self.or(self.registers.c),
            0xB2 => self.or(self.registers.d),
            0xB3 => self.or(self.registers.e),
            0xB4 => self.or(self.registers.h),
            0xB5 => self.or(self.registers.l),
            0xB6 => {
                let address = self.registers.hl();
                self.or(self.mmu.read_byte(address));
            }
            0xB7 => self.or(self.registers.a),
            0xC1 => {
                let value = self.pop();
                self.registers.set_bc(value);
            }
            0xC5 => {
                let value = self.registers.bc();
                self.push(value);
            }
            0xC6 => {
                let value = self.fetch_byte();
                self.add(value);
            }
            0xCE => {
                let value = self.fetch_byte();
                self.adc(value);
            }
            0xD1 => {
                let value = self.pop();
                self.registers.set_de(value);
            }
            0xD5 => {
                let value = self.registers.de();
                self.push(value);
            }
            0xD6 => {
                let value = self.fetch_byte();
                self.sub(value);
            }
            0xDE => {
                let value = self.fetch_byte();
                self.sbc(value);
            }
            0xE1 => {
                let value = self.pop();
                self.registers.set_hl(value);
            }
            0xE5 => {
                let value = self.registers.hl();
                self.push(value);
            }
            0xF1 => {
                let value = self.pop();
                self.registers.set_af(value);
            }
            0xF5 => {
                let value = self.registers.af();
                self.push(value);
            }

            _ => unimplemented!("Unkown instruction found for: 0x{:x}", op),
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.mmu.read_byte(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        byte
    }

    fn step(&mut self) {
        let instruction_byte = self.mmu.read_byte(self.registers.pc);
        self.execute(instruction_byte);
        self.registers.pc = self.registers.pc.wrapping_add(1);
    }
}
