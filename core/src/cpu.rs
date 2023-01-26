use crate::mmu::Memory;

mod decode;
mod instructions;
mod registers;

pub enum ImeFlag {
    Enabled,
    Disabled,
    WillEnable,
    WillDisable,
}

pub struct CPU {
    ime: ImeFlag,
    registers: registers::Registers,
    mmu: Memory,
    halt: bool,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: registers::Registers::new(),
            mmu: Memory::new(),
            ime: ImeFlag::Enabled,
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
        match self.ime {
            ImeFlag::WillEnable => self.ime = ImeFlag::Enabled,
            ImeFlag::WillDisable => self.ime = ImeFlag::Disabled,

            _ => return,
        }
    }
}
