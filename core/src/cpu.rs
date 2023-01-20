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
            0x0 => self.nop(),
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
            0xC1 => {
                let value = self.pop();
                self.registers.set_bc(value);
            }
            0xC5 => {
                let value = self.registers.bc();
                self.push(value);
            }
            0xD1 => {
                let value = self.pop();
                self.registers.set_de(value);
            }
            0xD5 => {
                let value = self.registers.de();
                self.push(value);
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

    fn step(&mut self) {
        let instruction_byte = self.mmu.read_byte(self.registers.pc);
        self.execute(instruction_byte);
        self.registers.pc += 1;
    }
}
