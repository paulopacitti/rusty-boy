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

    /// Decode op code and execute instruction. Returns how many clocks were necessary to run the instruction.
    fn execute(&mut self, op: u8) -> u32 {
        match op {
            0x00 => {
                self.nop();
                1
            }
            0x02 => {
                self.mmu.write_byte(self.registers.bc(), self.registers.a);
                2
            }
            0x03 => {
                let result = self.inc16(self.registers.bc());
                self.registers.set_bc(result);
                2
            }
            0x04 => {
                self.registers.b = self.inc(self.registers.b);
                1
            }
            0x05 => {
                self.registers.b = self.dec(self.registers.b);
                1
            }
            0x06 => {
                self.registers.b = self.fetch_byte();
                2
            }
            0x09 => {
                self.add16_hl(self.registers.bc());
                1
            }
            0x0A => {
                let address = self.registers.bc();
                let data = self.mmu.read_byte(address);
                self.registers.a = data;
                2
            }
            0x0B => {
                let result = self.dec16(self.registers.bc());
                self.registers.set_bc(result);
                2
            }
            0x0C => {
                self.registers.c = self.inc(self.registers.c);
                1
            }
            0x0D => {
                self.registers.c = self.dec(self.registers.c);
                1
            }
            0x0E => {
                self.registers.c = self.fetch_byte();
                2
            }
            0x12 => {
                self.mmu.write_byte(self.registers.de(), self.registers.a);
                2
            }
            0x13 => {
                let result = self.inc16(self.registers.de());
                self.registers.set_de(result);
                2
            }
            0x14 => {
                self.registers.d = self.inc(self.registers.d);
                1
            }
            0x15 => {
                self.registers.d = self.dec(self.registers.d);
                1
            }
            0x16 => {
                self.registers.d = self.fetch_byte();
                2
            }
            0x19 => {
                self.add16_hl(self.registers.de());
                2
            }
            0x1A => {
                let address = self.registers.de();
                let data = self.mmu.read_byte(address);
                self.registers.a = data;
                2
            }
            0x1B => {
                let result = self.dec16(self.registers.de());
                self.registers.set_de(result);
                2
            }
            0x1C => {
                self.registers.e = self.inc(self.registers.e);
                1
            }
            0x1D => {
                self.registers.e = self.dec(self.registers.e);
                1
            }
            0x1E => {
                self.registers.e = self.fetch_byte();
                2
            }
            0x22 => {
                self.mmu.write_byte(self.registers.hl(), self.registers.a);
                let inc = self.registers.hl().wrapping_add(1);
                self.registers.set_hl(inc);
                2
            }
            0x23 => {
                let result = self.inc16(self.registers.hl());
                self.registers.set_hl(result);
                2
            }
            0x24 => {
                self.registers.h = self.inc(self.registers.h);
                1
            }
            0x25 => {
                self.registers.h = self.dec(self.registers.h);
                1
            }
            0x26 => {
                self.registers.h = self.fetch_byte();
                2
            }
            0x27 => {
                self.daa();
                1
            }
            0x29 => {
                self.add16_hl(self.registers.hl());
                2
            }
            0x2A => {
                let address = self.registers.hl();
                let new_address = self.registers.hl().wrapping_add(1);
                self.registers.set_hl(new_address);

                let data = self.mmu.read_byte(address);
                self.registers.a = data;
                2
            }
            0x2B => {
                let result = self.dec16(self.registers.hl());
                self.registers.set_hl(result);
                2
            }
            0x2C => {
                self.registers.l = self.inc(self.registers.l);
                1
            }
            0x2D => {
                self.registers.l = self.dec(self.registers.l);
                1
            }
            0x2E => {
                self.registers.l = self.fetch_byte();
                2
            }
            0x2F => {
                self.cpl();
                1
            }
            0x32 => {
                self.mmu.write_byte(self.registers.hl(), self.registers.a);
                let dec = self.registers.hl().wrapping_sub(1);
                self.registers.set_hl(dec);
                2
            }
            0x33 => {
                let result = self.inc16(self.registers.sp);
                self.registers.sp = result;
                2
            }
            0x34 => {
                let address = self.registers.hl();
                let result = self.inc(self.mmu.read_byte(address));
                self.mmu.write_byte(address, result);
                3
            }
            0x35 => {
                let address = self.registers.hl();
                let result = self.dec(self.mmu.read_byte(address));
                self.mmu.write_byte(address, result);
                3
            }
            0x36 => {
                let imm = self.fetch_byte();
                let address = self.registers.hl();
                self.mmu.write_byte(address, imm);
                3
            }
            0x37 => {
                self.scf();
                1
            }
            0x39 => {
                self.add16_hl(self.registers.sp);
                2
            }
            0x3A => {
                let address = self.registers.hl();
                let new_address = self.registers.hl().wrapping_sub(1);
                self.registers.set_hl(new_address);

                let data = self.mmu.read_byte(address);
                self.registers.a = data;
                2
            }
            0x3B => {
                let result = self.dec16(self.registers.sp);
                self.registers.sp = result;
                2
            }
            0x3C => {
                self.registers.a = self.inc(self.registers.a);
                1
            }
            0x3D => {
                self.registers.a = self.dec(self.registers.a);
                1
            }
            0x3E => {
                self.registers.a = self.fetch_byte();
                2
            }
            0x3F => {
                self.ccf();
                1
            }
            0x40 => 1,
            0x41 => {
                self.registers.b = self.registers.c;
                1
            }
            0x42 => {
                self.registers.b = self.registers.d;
                1
            }
            0x43 => {
                self.registers.b = self.registers.e;
                1
            }
            0x44 => {
                self.registers.b = self.registers.h;
                1
            }
            0x45 => {
                self.registers.b = self.registers.l;
                1
            }
            0x46 => {
                let address = self.registers.hl();
                self.registers.b = self.mmu.read_byte(address);
                2
            }
            0x47 => {
                self.registers.b = self.registers.a;
                1
            }
            0x48 => {
                self.registers.c = self.registers.b;
                1
            }
            0x49 => 1,
            0x4A => {
                self.registers.c = self.registers.d;
                1
            }
            0x4B => {
                self.registers.c = self.registers.e;
                1
            }
            0x4C => {
                self.registers.c = self.registers.h;
                1
            }
            0x4D => {
                self.registers.c = self.registers.l;
                1
            }
            0x4E => {
                let address = self.registers.hl();
                self.registers.c = self.mmu.read_byte(address);
                2
            }
            0x4F => {
                self.registers.c = self.registers.a;
                1
            }
            0x50 => {
                self.registers.d = self.registers.b;
                1
            }
            0x51 => {
                self.registers.d = self.registers.c;
                1
            }
            0x52 => 1,
            0x53 => {
                self.registers.d = self.registers.e;
                1
            }
            0x54 => {
                self.registers.d = self.registers.h;
                1
            }
            0x55 => {
                self.registers.d = self.registers.l;
                1
            }
            0x56 => {
                let address = self.registers.hl();
                self.registers.d = self.mmu.read_byte(address);
                2
            }
            0x57 => {
                self.registers.d = self.registers.a;
                1
            }
            0x58 => {
                self.registers.e = self.registers.b;
                1
            }
            0x59 => {
                self.registers.e = self.registers.c;
                1
            }
            0x5A => {
                self.registers.e = self.registers.d;
                1
            }
            0x5B => 1,
            0x5C => {
                self.registers.e = self.registers.h;
                1
            }
            0x5D => {
                self.registers.e = self.registers.l;
                1
            }
            0x5E => {
                let address = self.registers.hl();
                self.registers.e = self.mmu.read_byte(address);
                2
            }
            0x5F => {
                self.registers.e = self.registers.a;
                1
            }
            0x60 => {
                self.registers.h = self.registers.b;
                1
            }
            0x61 => {
                self.registers.h = self.registers.c;
                1
            }
            0x62 => {
                self.registers.h = self.registers.d;
                1
            }
            0x63 => {
                self.registers.h = self.registers.e;
                1
            }
            0x64 => 1,
            0x65 => {
                self.registers.h = self.registers.l;
                1
            }
            0x66 => {
                let address = self.registers.hl();
                self.registers.h = self.mmu.read_byte(address);
                2
            }
            0x67 => {
                self.registers.h = self.registers.a;
                1
            }
            0x68 => {
                self.registers.l = self.registers.b;
                1
            }
            0x69 => {
                self.registers.l = self.registers.c;
                1
            }
            0x6A => {
                self.registers.l = self.registers.d;
                1
            }
            0x6B => {
                self.registers.l = self.registers.e;
                1
            }
            0x6C => {
                self.registers.l = self.registers.h;
                1
            }
            0x6D => 1,
            0x6E => {
                let address = self.registers.hl();
                self.registers.l = self.mmu.read_byte(address);
                2
            }
            0x6F => {
                self.registers.l = self.registers.a;
                1
            }
            0x70 => {
                let address = self.registers.hl();
                self.mmu.write_byte(address, self.registers.b);
                2
            }
            0x71 => {
                let address = self.registers.hl();
                self.mmu.write_byte(address, self.registers.c);
                2
            }
            0x72 => {
                let address = self.registers.hl();
                self.mmu.write_byte(address, self.registers.d);
                2
            }
            0x73 => {
                let address = self.registers.hl();
                self.mmu.write_byte(address, self.registers.e);
                2
            }
            0x74 => {
                let address = self.registers.hl();
                self.mmu.write_byte(address, self.registers.h);
                2
            }
            0x75 => {
                let address = self.registers.hl();
                self.mmu.write_byte(address, self.registers.l);
                2
            }
            0x76 => unimplemented!("HALT not implemented yet"),
            0x77 => {
                let address = self.registers.hl();
                self.mmu.write_byte(address, self.registers.a);
                2
            }
            0x78 => {
                self.registers.a = self.registers.b;
                1
            }
            0x79 => {
                self.registers.a = self.registers.c;
                1
            }
            0x7A => {
                self.registers.a = self.registers.d;
                1
            }
            0x7B => {
                self.registers.a = self.registers.e;
                1
            }
            0x7C => {
                self.registers.a = self.registers.h;
                1
            }
            0x7D => {
                self.registers.a = self.registers.l;
                1
            }
            0x7E => {
                let address = self.registers.hl();
                self.registers.a = self.mmu.read_byte(address);
                2
            }
            0x7F => 1,
            0x80 => {
                self.add(self.registers.b);
                1
            }
            0x81 => {
                self.add(self.registers.c);
                1
            }
            0x82 => {
                self.add(self.registers.d);
                1
            }
            0x83 => {
                self.add(self.registers.e);
                1
            }
            0x84 => {
                self.add(self.registers.h);
                1
            }
            0x85 => {
                self.add(self.registers.l);
                1
            }
            0x86 => {
                let address = self.registers.hl();
                self.add(self.mmu.read_byte(address));
                2
            }
            0x87 => {
                self.add(self.registers.a);
                1
            }
            0x88 => {
                self.adc(self.registers.b);
                1
            }
            0x89 => {
                self.adc(self.registers.c);
                1
            }
            0x8A => {
                self.adc(self.registers.d);
                1
            }
            0x8B => {
                self.adc(self.registers.e);
                1
            }
            0x8C => {
                self.adc(self.registers.h);
                1
            }
            0x8D => {
                self.adc(self.registers.l);
                1
            }
            0x8E => {
                let address = self.registers.hl();
                self.adc(self.mmu.read_byte(address));
                2
            }
            0x8F => {
                self.adc(self.registers.a);
                1
            }
            0x90 => {
                self.sub(self.registers.b);
                1
            }
            0x91 => {
                self.sub(self.registers.c);
                1
            }
            0x92 => {
                self.sub(self.registers.d);
                1
            }
            0x93 => {
                self.sub(self.registers.e);
                1
            }
            0x94 => {
                self.sub(self.registers.h);
                1
            }
            0x95 => {
                self.sub(self.registers.l);
                1
            }
            0x96 => {
                let address = self.registers.hl();
                self.sub(self.mmu.read_byte(address));
                2
            }
            0x97 => {
                self.sub(self.registers.a);
                1
            }
            0x98 => {
                self.sub(self.registers.b);
                1
            }
            0x99 => {
                self.sub(self.registers.c);
                1
            }
            0x9A => {
                self.sub(self.registers.d);
                1
            }
            0x9B => {
                self.sub(self.registers.e);
                1
            }
            0x9C => {
                self.sub(self.registers.h);
                1
            }
            0x9D => {
                self.sub(self.registers.l);
                1
            }
            0x9E => {
                let address = self.registers.hl();
                self.sub(self.mmu.read_byte(address));
                2
            }
            0x9F => {
                self.sub(self.registers.a);
                1
            }
            0xA0 => {
                self.and(self.registers.b);
                1
            }
            0xA1 => {
                self.and(self.registers.c);
                1
            }
            0xA2 => {
                self.and(self.registers.d);
                1
            }
            0xA3 => {
                self.and(self.registers.e);
                1
            }
            0xA4 => {
                self.and(self.registers.h);
                1
            }
            0xA5 => {
                self.and(self.registers.l);
                1
            }
            0xA6 => {
                let address = self.registers.hl();
                self.and(self.mmu.read_byte(address));
                2
            }
            0xA7 => {
                self.and(self.registers.a);
                1
            }
            0xA8 => {
                self.xor(self.registers.b);
                1
            }
            0xA9 => {
                self.xor(self.registers.c);
                1
            }
            0xAA => {
                self.xor(self.registers.d);
                1
            }
            0xAB => {
                self.xor(self.registers.e);
                1
            }
            0xAC => {
                self.xor(self.registers.h);
                1
            }
            0xAD => {
                self.xor(self.registers.l);
                1
            }
            0xAE => {
                let address = self.registers.hl();
                self.xor(self.mmu.read_byte(address));
                2
            }
            0xAF => {
                self.xor(self.registers.a);
                1
            }
            0xB0 => {
                self.or(self.registers.b);
                1
            }
            0xB1 => {
                self.or(self.registers.c);
                1
            }
            0xB2 => {
                self.or(self.registers.d);
                1
            }
            0xB3 => {
                self.or(self.registers.e);
                1
            }
            0xB4 => {
                self.or(self.registers.h);
                1
            }
            0xB5 => {
                self.or(self.registers.l);
                1
            }
            0xB6 => {
                let address = self.registers.hl();
                self.or(self.mmu.read_byte(address));
                2
            }
            0xB7 => {
                self.or(self.registers.a);
                1
            }
            0xB8 => {
                self.cp(self.registers.b);
                1
            }
            0xB9 => {
                self.cp(self.registers.c);
                1
            }
            0xBA => {
                self.cp(self.registers.d);
                1
            }
            0xBB => {
                self.cp(self.registers.e);
                1
            }
            0xBC => {
                self.cp(self.registers.h);
                1
            }
            0xBD => {
                self.cp(self.registers.l);
                1
            }
            0xBE => {
                let address = self.registers.hl();
                self.cp(self.mmu.read_byte(address));
                2
            }
            0xBF => {
                self.cp(self.registers.a);
                1
            }
            0xC1 => {
                let value = self.pop();
                self.registers.set_bc(value);
                1
            }
            0xC5 => {
                let value = self.registers.bc();
                self.push(value);
                1
            }
            0xC6 => {
                let value = self.fetch_byte();
                self.add(value);
                2
            }
            0xCE => {
                let value = self.fetch_byte();
                self.adc(value);
                2
            }
            0xD1 => {
                let value = self.pop();
                self.registers.set_de(value);
                1
            }
            0xD5 => {
                let value = self.registers.de();
                self.push(value);
                1
            }
            0xD6 => {
                let value = self.fetch_byte();
                self.sub(value);
                2
            }
            0xDE => {
                let value = self.fetch_byte();
                self.sbc(value);
                2
            }
            0xE1 => {
                let value = self.pop();
                self.registers.set_hl(value);
                1
            }
            0xE2 => {
                let address = self.mmu.io_ports_address + (self.registers.c as u16);
                let data = self.registers.a;
                self.mmu.write_byte(address, data);
                2
            }
            0xE5 => {
                let value = self.registers.hl();
                self.push(value);
                1
            }
            0xE6 => {
                let value = self.fetch_byte();
                self.and(value);
                2
            }
            0xE8 => {
                let value = self.fetch_byte();
                self.add16_sp(value);
                4
            }
            0xEA => {
                let address = self.fetch_word();
                let data = self.registers.a;
                self.mmu.write_byte(address, data);
                4
            }
            0xEE => {
                let value = self.fetch_byte();
                self.xor(value);
                2
            }

            0xF1 => {
                let value = self.pop();
                self.registers.set_af(value);
                1
            }
            0xF2 => {
                let address = self.mmu.io_ports_address + (self.registers.c as u16);
                let data = self.mmu.read_byte(address);
                self.registers.a = data;
                2
            }
            0xF5 => {
                let value = self.registers.af();
                self.push(value);
                1
            }
            0xF6 => {
                let value = self.fetch_byte();
                self.or(value);
                2
            }
            0xFA => {
                let address = self.fetch_word();
                let data = self.mmu.read_byte(address);
                self.registers.a = data;
                4
            }
            0xFE => {
                let value = self.fetch_byte();
                self.cp(value);
                2
            }

            _ => unimplemented!("Unkown instruction found for: 0x{:x}", op),
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.mmu.read_byte(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        byte
    }

    fn fetch_word(&mut self) -> u16 {
        let word = self.mmu.read_word(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);
        word
    }

    fn step(&mut self) {
        let instruction_byte = self.mmu.read_byte(self.registers.pc);
        self.execute(instruction_byte);
        self.registers.pc = self.registers.pc.wrapping_add(1);
    }
}
