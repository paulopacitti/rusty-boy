use crate::{memory::Memory, mmu::MMU};

mod decode;
mod instructions;
mod registers;

pub struct ImeFlagTimer {
    pub ei: u8,
    pub di: u8,
}

impl ImeFlagTimer {
    pub fn new() -> Self {
        ImeFlagTimer { ei: 0, di: 0 }
    }
}

pub struct CPU {
    halt: bool,
    ime: bool,
    ime_timer: ImeFlagTimer,
    mmu: MMU,
    registers: registers::Registers,
}

impl CPU {
    pub fn new(mmu: MMU) -> Self {
        CPU {
            registers: registers::Registers::new(),
            mmu: mmu,
            ime: true,
            ime_timer: ImeFlagTimer::new(),
            halt: false,
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

    fn step(&mut self) -> u32 {
        self.update_ime();
        let instruction = self.fetch_byte();

        if self.halt {
            1 // Emulate an noop instruction
        } else {
            self.execute(instruction)
        }
    }

    fn update_ime(&mut self) {
        self.ime_timer.ei = match self.ime_timer.ei {
            2 => 1,
            1 => {
                self.ime = true;
                0
            }

            _ => 0,
        };

        self.ime_timer.di = match self.ime_timer.ei {
            2 => 1,
            1 => {
                self.ime = false;
                0
            }

            _ => 0,
        };
    }
}
