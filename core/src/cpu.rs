use crate::mmu::Memory;

mod decode;
mod instructions;
mod registers;

pub struct CPU {
    ime: bool,
    registers: registers::Registers,
    mmu: Memory,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: registers::Registers::new(),
            mmu: Memory::new(),
            ime: true,
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
