use super::ImeFlag;

impl super::CPU {
    /// Decode op code and execute instruction. Returns how many clocks were necessary to run the instruction.
    pub fn execute(&mut self, op: u8) -> u32 {
        match op {
            0x00 => {
                self.nop();
                1
            }
            0x01 => {
                let data = self.fetch_word();
                self.registers.set_bc(data);
                3
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
            0x07 => {
                self.rlca();
                2
            }
            0x08 => {
                let address = self.fetch_word();
                let data = self.registers.sp;
                self.mmu.write_word(address, data);
                5
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
            0x0F => {
                self.rrca();
                2
            }
            0x11 => {
                let data = self.fetch_word();
                self.registers.set_de(data);
                3
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
            0x17 => {
                self.rla();
                2
            }
            0x18 => {
                self.jr();
                3
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
            0x1F => {
                self.rra();
                2
            }
            0x20 => {
                if !self.registers.f.z() {
                    self.jr();
                    3
                } else {
                    2
                }
            }
            0x21 => {
                let data = self.fetch_word();
                self.registers.set_hl(data);
                3
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
            0x28 => {
                if self.registers.f.z() {
                    self.jr();
                    3
                } else {
                    2
                }
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
            0x30 => {
                if !self.registers.f.c() {
                    self.jr();
                    3
                } else {
                    2
                }
            }
            0x31 => {
                let data = self.fetch_word();
                self.registers.sp = data;
                3
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
            0x38 => {
                if self.registers.f.c() {
                    self.jr();
                    3
                } else {
                    2
                }
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
            0x76 => {
                self.halt = true;
                1
            }
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
            0xC0 => {
                if !self.registers.f.z() {
                    self.ret();
                    5
                } else {
                    2
                }
            }
            0xC1 => {
                let value = self.pop();
                self.registers.set_bc(value);
                3
            }
            0xC2 => {
                if !self.registers.f.z() {
                    self.jp();
                    4
                } else {
                    3
                }
            }
            0xC3 => {
                self.jp();
                4
            }
            0xC4 => {
                if !self.registers.f.z() {
                    self.call();
                    6
                } else {
                    3
                }
            }
            0xC5 => {
                let value = self.registers.bc();
                self.push(value);
                4
            }
            0xC6 => {
                let value = self.fetch_byte();
                self.add(value);
                2
            }
            0xC7 => {
                self.rst(0x0000);
                4
            }
            0xC8 => {
                if self.registers.f.z() {
                    self.ret();
                    5
                } else {
                    2
                }
            }
            0xC9 => {
                self.ret();
                4
            }
            0xCA => {
                if self.registers.f.z() {
                    self.jp();
                    4
                } else {
                    3
                }
            }
            0xCC => {
                if self.registers.f.z() {
                    self.call();
                    6
                } else {
                    3
                }
            }
            0xCD => {
                self.call();
                6
            }
            0xCE => {
                let value = self.fetch_byte();
                self.adc(value);
                2
            }
            0xCF => {
                self.rst(0x0008);
                4
            }
            0xD0 => {
                if !self.registers.f.c() {
                    self.ret();
                    5
                } else {
                    2
                }
            }
            0xD1 => {
                let value = self.pop();
                self.registers.set_de(value);
                3
            }
            0xD2 => {
                if !self.registers.f.c() {
                    self.jp();
                    4
                } else {
                    3
                }
            }
            0xD4 => {
                if !self.registers.f.c() {
                    self.call();
                    6
                } else {
                    3
                }
            }
            0xD5 => {
                let value = self.registers.de();
                self.push(value);
                4
            }
            0xD6 => {
                let value = self.fetch_byte();
                self.sub(value);
                2
            }
            0xD7 => {
                self.rst(0x0010);
                4
            }
            0xD8 => {
                if self.registers.f.c() {
                    self.ret();
                    5
                } else {
                    2
                }
            }
            0xD9 => {
                self.ret();
                self.ime = ImeFlag::Enabled;
                4
            }
            0xDA => {
                if self.registers.f.c() {
                    self.jp();
                    4
                } else {
                    3
                }
            }
            0xDC => {
                if self.registers.f.c() {
                    self.call();
                    6
                } else {
                    3
                }
            }
            0xDE => {
                let value = self.fetch_byte();
                self.sbc(value);
                2
            }
            0xDF => {
                self.rst(0x0018);
                4
            }
            0xE0 => {
                let partial_address = self.fetch_byte();
                let address = self.mmu.io_ports_address | partial_address as u16;
                let data = self.registers.a;
                self.mmu.write_byte(address, data);
                3
            }
            0xE1 => {
                let value = self.pop();
                self.registers.set_hl(value);
                3
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
                4
            }
            0xE6 => {
                let value = self.fetch_byte();
                self.and(value);
                2
            }
            0xE7 => {
                self.rst(0x0020);
                4
            }
            0xE8 => {
                let value = self.fetch_byte();
                self.add16_sp(value);
                4
            }
            0xE9 => {
                self.registers.pc = self.registers.hl();
                1
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
            0xEF => {
                self.rst(0x0028);
                4
            }
            0xF0 => {
                let partial_address = self.fetch_byte();
                let address = self.mmu.io_ports_address | partial_address as u16;
                let data = self.mmu.read_byte(address);
                self.registers.a = data;
                3
            }
            0xF1 => {
                let value = self.pop();
                self.registers.set_af(value);
                3
            }
            0xF3 => {
                self.ime = ImeFlag::WillDisable;
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
                4
            }
            0xF6 => {
                let value = self.fetch_byte();
                self.or(value);
                2
            }
            0xF7 => {
                self.rst(0x0030);
                4
            }
            0xF8 => {
                let offset = self.fetch_byte() as u16;
                let sum = self.registers.sp.wrapping_add(offset);
                self.registers.f.set_z(false);
                self.registers.f.set_n(false);
                self.registers
                    .f
                    .set_h((self.registers.sp & 0x000F) + (offset & 0x000F) > 0x000F);
                self.registers
                    .f
                    .set_c((self.registers.sp & 0x00FF) + (offset & 0x00FF) > 0x00FF);

                self.registers.set_hl(sum);
                3
            }
            0xF9 => {
                self.registers.sp = self.registers.hl();
                2
            }
            0xFA => {
                let address = self.fetch_word();
                let data = self.mmu.read_byte(address);
                self.registers.a = data;
                4
            }
            0xFB => {
                self.ime = ImeFlag::WillEnable;
                1
            }
            0xFE => {
                let value = self.fetch_byte();
                self.cp(value);
                2
            }
            0xFF => {
                self.rst(0x0038);
                4
            }

            _ => unimplemented!("Unkown instruction found for: 0x{:x}", op),
        }
    }

    pub fn execute_cb(&mut self, op: u8) -> u32 {
        let op = self.fetch_byte();
        match op {
            0x00 => {
                self.registers.b = self.rlc(self.registers.b);
                2
            }
            0x01 => {
                self.registers.c = self.rlc(self.registers.c);
                2
            }
            0x02 => {
                self.registers.d = self.rlc(self.registers.d);
                2
            }
            0x03 => {
                self.registers.e = self.rlc(self.registers.e);
                2
            }
            0x04 => {
                self.registers.h = self.rlc(self.registers.h);
                2
            }
            0x05 => {
                self.registers.l = self.rlc(self.registers.l);
                2
            }
            0x06 => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                let rotation = self.rlc(data);
                self.mmu.write_byte(address, rotation);
                4
            }
            0x07 => {
                self.registers.a = self.rlc(self.registers.a);
                2
            }
            0x08 => {
                self.registers.b = self.rrc(self.registers.b);
                2
            }
            0x09 => {
                self.registers.c = self.rrc(self.registers.c);
                2
            }
            0x0A => {
                self.registers.d = self.rrc(self.registers.d);
                2
            }
            0x0B => {
                self.registers.e = self.rrc(self.registers.e);
                2
            }
            0x0C => {
                self.registers.h = self.rrc(self.registers.h);
                2
            }
            0x0D => {
                self.registers.l = self.rrc(self.registers.l);
                2
            }
            0x0E => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                let rotation = self.rrc(data);
                self.mmu.write_byte(address, rotation);
                4
            }
            0x0F => {
                self.registers.a = self.rrc(self.registers.a);
                2
            }
            0x10 => {
                self.registers.b = self.rl(self.registers.b);
                2
            }
            0x11 => {
                self.registers.c = self.rl(self.registers.c);
                2
            }
            0x12 => {
                self.registers.d = self.rl(self.registers.d);
                2
            }
            0x13 => {
                self.registers.e = self.rl(self.registers.e);
                2
            }
            0x14 => {
                self.registers.h = self.rl(self.registers.h);
                2
            }
            0x15 => {
                self.registers.l = self.rl(self.registers.l);
                2
            }
            0x16 => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                let rotation = self.rl(data);
                self.mmu.write_byte(address, rotation);
                4
            }
            0x17 => {
                self.registers.a = self.rl(self.registers.a);
                2
            }
            0x18 => {
                self.registers.b = self.rr(self.registers.b);
                2
            }
            0x19 => {
                self.registers.c = self.rr(self.registers.c);
                2
            }
            0x1A => {
                self.registers.d = self.rr(self.registers.d);
                2
            }
            0x1B => {
                self.registers.e = self.rr(self.registers.e);
                2
            }
            0x1C => {
                self.registers.h = self.rr(self.registers.h);
                2
            }
            0x1D => {
                self.registers.l = self.rr(self.registers.l);
                2
            }
            0x1E => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                let rotation = self.rr(data);
                self.mmu.write_byte(address, rotation);
                4
            }
            0x1F => {
                self.registers.a = self.rr(self.registers.a);
                2
            }
            0x20 => {
                self.registers.b = self.sla(self.registers.b);
                2
            }
            0x21 => {
                self.registers.c = self.sla(self.registers.c);
                2
            }
            0x22 => {
                self.registers.d = self.sla(self.registers.d);
                2
            }
            0x23 => {
                self.registers.e = self.sla(self.registers.e);
                2
            }
            0x24 => {
                self.registers.h = self.sla(self.registers.h);
                2
            }
            0x25 => {
                self.registers.l = self.sla(self.registers.l);
                2
            }
            0x26 => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                let rotation = self.sla(data);
                self.mmu.write_byte(address, rotation);
                4
            }
            0x27 => {
                self.registers.a = self.sla(self.registers.a);
                2
            }
            0x28 => {
                self.registers.b = self.sra(self.registers.b);
                2
            }
            0x29 => {
                self.registers.c = self.sra(self.registers.c);
                2
            }
            0x2A => {
                self.registers.d = self.sra(self.registers.d);
                2
            }
            0x2B => {
                self.registers.e = self.sra(self.registers.e);
                2
            }
            0x2C => {
                self.registers.h = self.sra(self.registers.h);
                2
            }
            0x2D => {
                self.registers.l = self.sra(self.registers.l);
                2
            }
            0x2E => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                let rotation = self.sra(data);
                self.mmu.write_byte(address, rotation);
                4
            }
            0x2F => {
                self.registers.a = self.sra(self.registers.a);
                2
            }
            0x30 => {
                self.registers.b = self.swap(self.registers.b);
                2
            }
            0x31 => {
                self.registers.c = self.swap(self.registers.c);
                2
            }
            0x32 => {
                self.registers.d = self.swap(self.registers.d);
                2
            }
            0x33 => {
                self.registers.e = self.swap(self.registers.e);
                2
            }
            0x34 => {
                self.registers.h = self.swap(self.registers.h);
                2
            }
            0x35 => {
                self.registers.l = self.swap(self.registers.l);
                2
            }
            0x36 => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                let rotation = self.swap(data);
                self.mmu.write_byte(address, rotation);
                4
            }
            0x37 => {
                self.registers.a = self.swap(self.registers.a);
                2
            }
            0x38 => {
                self.registers.b = self.srl(self.registers.b);
                2
            }
            0x39 => {
                self.registers.c = self.srl(self.registers.c);
                2
            }
            0x3A => {
                self.registers.d = self.srl(self.registers.d);
                2
            }
            0x3B => {
                self.registers.e = self.srl(self.registers.e);
                2
            }
            0x3C => {
                self.registers.h = self.srl(self.registers.h);
                2
            }
            0x3D => {
                self.registers.l = self.srl(self.registers.l);
                2
            }
            0x3E => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                let rotation = self.srl(data);
                self.mmu.write_byte(address, rotation);
                4
            }
            0x3F => {
                self.registers.a = self.srl(self.registers.a);
                2
            }
            0x40 => {
                self.bit(self.registers.b, 0);
                2
            }
            0x41 => {
                self.bit(self.registers.c, 0);
                2
            }
            0x42 => {
                self.bit(self.registers.d, 0);
                2
            }
            0x43 => {
                self.bit(self.registers.e, 0);
                2
            }
            0x44 => {
                self.bit(self.registers.h, 0);
                2
            }
            0x45 => {
                self.bit(self.registers.l, 0);
                2
            }
            0x46 => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                self.bit(data, 0);
                3
            }
            0x47 => {
                self.bit(self.registers.a, 0);
                2
            }
            0x48 => {
                self.bit(self.registers.b, 1);
                2
            }
            0x49 => {
                self.bit(self.registers.c, 1);
                2
            }
            0x4A => {
                self.bit(self.registers.d, 1);
                2
            }
            0x4B => {
                self.bit(self.registers.e, 1);
                2
            }
            0x4C => {
                self.bit(self.registers.h, 1);
                2
            }
            0x4D => {
                self.bit(self.registers.l, 1);
                2
            }
            0x4E => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                self.bit(data, 1);
                3
            }
            0x4F => {
                self.bit(self.registers.a, 1);
                2
            }
            0x50 => {
                self.bit(self.registers.b, 2);
                2
            }
            0x51 => {
                self.bit(self.registers.c, 2);
                2
            }
            0x52 => {
                self.bit(self.registers.d, 2);
                2
            }
            0x53 => {
                self.bit(self.registers.e, 2);
                2
            }
            0x54 => {
                self.bit(self.registers.h, 2);
                2
            }
            0x55 => {
                self.bit(self.registers.l, 2);
                2
            }
            0x56 => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                self.bit(data, 2);
                3
            }
            0x57 => {
                self.bit(self.registers.a, 2);
                2
            }
            0x58 => {
                self.bit(self.registers.b, 3);
                2
            }
            0x59 => {
                self.bit(self.registers.c, 3);
                2
            }
            0x5A => {
                self.bit(self.registers.d, 3);
                2
            }
            0x5B => {
                self.bit(self.registers.e, 3);
                2
            }
            0x5C => {
                self.bit(self.registers.h, 3);
                2
            }
            0x5D => {
                self.bit(self.registers.l, 3);
                2
            }
            0x5E => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                self.bit(data, 3);
                3
            }
            0x5F => {
                self.bit(self.registers.a, 3);
                2
            }
            0x60 => {
                self.bit(self.registers.b, 4);
                2
            }
            0x61 => {
                self.bit(self.registers.c, 4);
                2
            }
            0x62 => {
                self.bit(self.registers.d, 4);
                2
            }
            0x63 => {
                self.bit(self.registers.e, 4);
                2
            }
            0x64 => {
                self.bit(self.registers.h, 4);
                2
            }
            0x65 => {
                self.bit(self.registers.l, 4);
                2
            }
            0x66 => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                self.bit(data, 4);
                3
            }
            0x67 => {
                self.bit(self.registers.a, 4);
                2
            }
            0x68 => {
                self.bit(self.registers.b, 5);
                2
            }
            0x69 => {
                self.bit(self.registers.c, 5);
                2
            }
            0x6A => {
                self.bit(self.registers.d, 5);
                2
            }
            0x6B => {
                self.bit(self.registers.e, 5);
                2
            }
            0x6C => {
                self.bit(self.registers.h, 5);
                2
            }
            0x6D => {
                self.bit(self.registers.l, 5);
                2
            }
            0x6E => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                self.bit(data, 5);
                3
            }
            0x6F => {
                self.bit(self.registers.a, 5);
                2
            }
            0x70 => {
                self.bit(self.registers.b, 6);
                2
            }
            0x71 => {
                self.bit(self.registers.c, 6);
                2
            }
            0x72 => {
                self.bit(self.registers.d, 6);
                2
            }
            0x73 => {
                self.bit(self.registers.e, 6);
                2
            }
            0x74 => {
                self.bit(self.registers.h, 6);
                2
            }
            0x75 => {
                self.bit(self.registers.l, 6);
                2
            }
            0x76 => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                self.bit(data, 6);
                3
            }
            0x77 => {
                self.bit(self.registers.a, 6);
                2
            }
            0x78 => {
                self.bit(self.registers.b, 7);
                2
            }
            0x79 => {
                self.bit(self.registers.c, 7);
                2
            }
            0x7A => {
                self.bit(self.registers.d, 7);
                2
            }
            0x7B => {
                self.bit(self.registers.e, 7);
                2
            }
            0x7C => {
                self.bit(self.registers.h, 7);
                2
            }
            0x7D => {
                self.bit(self.registers.l, 7);
                2
            }
            0x7E => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                self.bit(data, 7);
                3
            }
            0x7F => {
                self.bit(self.registers.a, 7);
                2
            }
            0x80 => {
                self.registers.b = self.res(self.registers.b, 0);
                2
            }
            0x81 => {
                self.registers.c = self.res(self.registers.c, 0);
                2
            }
            0x82 => {
                self.registers.d = self.res(self.registers.d, 0);
                2
            }
            0x83 => {
                self.registers.e = self.res(self.registers.e, 0);
                2
            }
            0x84 => {
                self.registers.h = self.res(self.registers.h, 0);
                2
            }
            0x85 => {
                self.registers.l = self.res(self.registers.l, 0);
                2
            }
            0x86 => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                let reset = self.res(data, 0);
                self.mmu.write_byte(address, reset);
                4
            }
            0x87 => {
                self.registers.a = self.res(self.registers.a, 0);
                2
            }
            0x88 => {
                self.registers.b = self.res(self.registers.b, 1);
                2
            }
            0x89 => {
                self.registers.c = self.res(self.registers.c, 1);
                2
            }
            0x8A => {
                self.registers.d = self.res(self.registers.d, 1);
                2
            }
            0x8B => {
                self.registers.e = self.res(self.registers.e, 1);
                2
            }
            0x8C => {
                self.registers.h = self.res(self.registers.h, 1);
                2
            }
            0x8D => {
                self.registers.l = self.res(self.registers.l, 1);
                2
            }
            0x8E => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                self.res(data, 1);
                4
            }
            0x8F => {
                self.registers.a = self.res(self.registers.a, 1);
                2
            }
            0x90 => {
                self.registers.b = self.res(self.registers.b, 2);
                2
            }
            0x91 => {
                self.registers.c = self.res(self.registers.c, 2);
                2
            }
            0x92 => {
                self.registers.d = self.res(self.registers.d, 2);
                2
            }
            0x93 => {
                self.registers.e = self.res(self.registers.e, 2);
                2
            }
            0x94 => {
                self.registers.h = self.res(self.registers.h, 2);
                2
            }
            0x95 => {
                self.registers.l = self.res(self.registers.l, 2);
                2
            }
            0x96 => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                let reset = self.res(data, 2);
                self.mmu.write_byte(address, reset);
                4
            }
            0x97 => {
                self.registers.a = self.res(self.registers.a, 2);
                2
            }
            0x98 => {
                self.registers.b = self.res(self.registers.b, 3);
                2
            }
            0x99 => {
                self.registers.c = self.res(self.registers.c, 3);
                2
            }
            0x9A => {
                self.registers.d = self.res(self.registers.d, 3);
                2
            }
            0x9B => {
                self.registers.e = self.res(self.registers.e, 3);
                2
            }
            0x9C => {
                self.registers.h = self.res(self.registers.h, 3);
                2
            }
            0x9D => {
                self.registers.l = self.res(self.registers.l, 3);
                2
            }
            0x9E => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                self.res(data, 3);
                4
            }
            0x9F => {
                self.registers.a = self.res(self.registers.a, 3);
                2
            }
            0xA0 => {
                self.registers.b = self.res(self.registers.b, 4);
                2
            }
            0xA1 => {
                self.registers.c = self.res(self.registers.c, 4);
                2
            }
            0xA2 => {
                self.registers.d = self.res(self.registers.d, 4);
                2
            }
            0xA3 => {
                self.registers.e = self.res(self.registers.e, 4);
                2
            }
            0xA4 => {
                self.registers.h = self.res(self.registers.h, 4);
                2
            }
            0xA5 => {
                self.registers.l = self.res(self.registers.l, 4);
                2
            }
            0xA6 => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                let reset = self.res(data, 4);
                self.mmu.write_byte(address, reset);
                4
            }
            0xA7 => {
                self.registers.a = self.res(self.registers.a, 4);
                2
            }
            0xA8 => {
                self.registers.b = self.res(self.registers.b, 5);
                2
            }
            0xA9 => {
                self.registers.c = self.res(self.registers.c, 5);
                2
            }
            0xAA => {
                self.registers.d = self.res(self.registers.d, 5);
                2
            }
            0xAB => {
                self.registers.e = self.res(self.registers.e, 5);
                2
            }
            0xAC => {
                self.registers.h = self.res(self.registers.h, 5);
                2
            }
            0xAD => {
                self.registers.l = self.res(self.registers.l, 5);
                2
            }
            0xAE => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                self.res(data, 5);
                4
            }
            0xAF => {
                self.registers.a = self.res(self.registers.a, 5);
                2
            }
            0xB0 => {
                self.registers.b = self.res(self.registers.b, 6);
                2
            }
            0xB1 => {
                self.registers.c = self.res(self.registers.c, 6);
                2
            }
            0xB2 => {
                self.registers.d = self.res(self.registers.d, 6);
                2
            }
            0xB3 => {
                self.registers.e = self.res(self.registers.e, 6);
                2
            }
            0xB4 => {
                self.registers.h = self.res(self.registers.h, 6);
                2
            }
            0xB5 => {
                self.registers.l = self.res(self.registers.l, 6);
                2
            }
            0xB6 => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                let reset = self.res(data, 6);
                self.mmu.write_byte(address, reset);
                4
            }
            0xB7 => {
                self.registers.a = self.res(self.registers.a, 6);
                2
            }
            0xB8 => {
                self.registers.b = self.res(self.registers.b, 7);
                2
            }
            0xB9 => {
                self.registers.c = self.res(self.registers.c, 7);
                2
            }
            0xBA => {
                self.registers.d = self.res(self.registers.d, 7);
                2
            }
            0xBB => {
                self.registers.e = self.res(self.registers.e, 7);
                2
            }
            0xBC => {
                self.registers.h = self.res(self.registers.h, 7);
                2
            }
            0xBD => {
                self.registers.l = self.res(self.registers.l, 7);
                2
            }
            0xBE => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                self.res(data, 7);
                4
            }
            0xBF => {
                self.registers.a = self.res(self.registers.a, 7);
                2
            }
            0xC0 => {
                self.registers.b = self.set(self.registers.b, 0);
                2
            }
            0xC1 => {
                self.registers.c = self.set(self.registers.c, 0);
                2
            }
            0xC2 => {
                self.registers.d = self.set(self.registers.d, 0);
                2
            }
            0xC3 => {
                self.registers.e = self.set(self.registers.e, 0);
                2
            }
            0xC4 => {
                self.registers.h = self.set(self.registers.h, 0);
                2
            }
            0xC5 => {
                self.registers.l = self.set(self.registers.l, 0);
                2
            }
            0xC6 => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                let reset = self.set(data, 0);
                self.mmu.write_byte(address, reset);
                4
            }
            0xC7 => {
                self.registers.a = self.set(self.registers.a, 0);
                2
            }
            0xC8 => {
                self.registers.b = self.set(self.registers.b, 1);
                2
            }
            0xC9 => {
                self.registers.c = self.set(self.registers.c, 1);
                2
            }
            0xCA => {
                self.registers.d = self.set(self.registers.d, 1);
                2
            }
            0xCB => {
                self.registers.e = self.set(self.registers.e, 1);
                2
            }
            0xCC => {
                self.registers.h = self.set(self.registers.h, 1);
                2
            }
            0xCD => {
                self.registers.l = self.set(self.registers.l, 1);
                2
            }
            0xCE => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                self.set(data, 1);
                4
            }
            0xCF => {
                self.registers.a = self.set(self.registers.a, 1);
                2
            }
            0xD0 => {
                self.registers.b = self.set(self.registers.b, 2);
                2
            }
            0xD1 => {
                self.registers.c = self.set(self.registers.c, 2);
                2
            }
            0xD2 => {
                self.registers.d = self.set(self.registers.d, 2);
                2
            }
            0xD3 => {
                self.registers.e = self.set(self.registers.e, 2);
                2
            }
            0xD4 => {
                self.registers.h = self.set(self.registers.h, 2);
                2
            }
            0xD5 => {
                self.registers.l = self.set(self.registers.l, 2);
                2
            }
            0xD6 => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                let reset = self.set(data, 2);
                self.mmu.write_byte(address, reset);
                4
            }
            0xD7 => {
                self.registers.a = self.set(self.registers.a, 2);
                2
            }
            0xD8 => {
                self.registers.b = self.set(self.registers.b, 3);
                2
            }
            0xD9 => {
                self.registers.c = self.set(self.registers.c, 3);
                2
            }
            0xDA => {
                self.registers.d = self.set(self.registers.d, 3);
                2
            }
            0xDB => {
                self.registers.e = self.set(self.registers.e, 3);
                2
            }
            0xDC => {
                self.registers.h = self.set(self.registers.h, 3);
                2
            }
            0xDD => {
                self.registers.l = self.set(self.registers.l, 3);
                2
            }
            0xDE => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                self.set(data, 3);
                4
            }
            0xDF => {
                self.registers.a = self.set(self.registers.a, 3);
                2
            }
            0xE0 => {
                self.registers.b = self.set(self.registers.b, 4);
                2
            }
            0xE1 => {
                self.registers.c = self.set(self.registers.c, 4);
                2
            }
            0xE2 => {
                self.registers.d = self.set(self.registers.d, 4);
                2
            }
            0xE3 => {
                self.registers.e = self.set(self.registers.e, 4);
                2
            }
            0xE4 => {
                self.registers.h = self.set(self.registers.h, 4);
                2
            }
            0xE5 => {
                self.registers.l = self.set(self.registers.l, 4);
                2
            }
            0xE6 => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                let reset = self.set(data, 4);
                self.mmu.write_byte(address, reset);
                4
            }
            0xE7 => {
                self.registers.a = self.set(self.registers.a, 4);
                2
            }
            0xE8 => {
                self.registers.b = self.set(self.registers.b, 5);
                2
            }
            0xE9 => {
                self.registers.c = self.set(self.registers.c, 5);
                2
            }
            0xEA => {
                self.registers.d = self.set(self.registers.d, 5);
                2
            }
            0xEB => {
                self.registers.e = self.set(self.registers.e, 5);
                2
            }
            0xEC => {
                self.registers.h = self.set(self.registers.h, 5);
                2
            }
            0xED => {
                self.registers.l = self.set(self.registers.l, 5);
                2
            }
            0xEE => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                self.set(data, 5);
                4
            }
            0xEF => {
                self.registers.a = self.set(self.registers.a, 5);
                2
            }
            0xF0 => {
                self.registers.b = self.set(self.registers.b, 6);
                2
            }
            0xF1 => {
                self.registers.c = self.set(self.registers.c, 6);
                2
            }
            0xF2 => {
                self.registers.d = self.set(self.registers.d, 6);
                2
            }
            0xF3 => {
                self.registers.e = self.set(self.registers.e, 6);
                2
            }
            0xF4 => {
                self.registers.h = self.set(self.registers.h, 6);
                2
            }
            0xF5 => {
                self.registers.l = self.set(self.registers.l, 6);
                2
            }
            0xF6 => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                let reset = self.set(data, 6);
                self.mmu.write_byte(address, reset);
                4
            }
            0xF7 => {
                self.registers.a = self.set(self.registers.a, 6);
                2
            }
            0xF8 => {
                self.registers.b = self.set(self.registers.b, 7);
                2
            }
            0xF9 => {
                self.registers.c = self.set(self.registers.c, 7);
                2
            }
            0xFA => {
                self.registers.d = self.set(self.registers.d, 7);
                2
            }
            0xFB => {
                self.registers.e = self.set(self.registers.e, 7);
                2
            }
            0xFC => {
                self.registers.h = self.set(self.registers.h, 7);
                2
            }
            0xFD => {
                self.registers.l = self.set(self.registers.l, 7);
                2
            }
            0xFE => {
                let address = self.registers.hl();
                let data = self.mmu.read_byte(address);
                self.set(data, 7);
                4
            }
            0xFF => {
                self.registers.a = self.set(self.registers.a, 7);
                2
            }
        }
    }
}
