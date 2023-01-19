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
            0x0C => self.registers.c = self.inc(self.registers.c),
            0x14 => self.registers.d = self.inc(self.registers.d),
            0x20 => self.registers.e = self.inc(self.registers.e),
            0x24 => self.registers.l = self.inc(self.registers.l),
            0x34 => {
                let address = self.registers.hl();
                let result = self.inc(self.mmu.read_byte(address));
                self.mmu.write_byte(address, result);
            }
            0x3C => self.registers.a = self.inc(self.registers.a),

            _ => unimplemented!("Unkown instruction found for: 0x{:x}", op),
        }
    }

    fn pop(&mut self) -> u16 {
        let lsb = self.mmu.read_byte(self.registers.sp) as u16;
        self.registers.sp = self.registers.sp.wrapping_add(1);

        let msb = self.mmu.read_byte(self.registers.sp) as u16;
        self.registers.sp = self.registers.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

    fn push(&mut self, value: u16) {
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.mmu
            .write_byte(self.registers.sp, ((value & 0xFF00) >> 8) as u8);

        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.mmu.write_byte(self.registers.sp, (value & 0xFF) as u8);
    }

    fn step(&mut self) {
        let instruction_byte = self.mmu.read_byte(self.registers.pc);
        self.execute(instruction_byte);
        self.registers.pc += 1;
    }
}
