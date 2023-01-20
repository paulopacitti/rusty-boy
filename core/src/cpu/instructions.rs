impl super::CPU {
    /// Decrement 8bit value.
    pub fn dec(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        if result == 0 {
            self.registers.f.set_z(true);
        }
        self.registers.f.set_n(true);
        self.registers.f.set_h(value.trailing_zeros() >= 4); // Check for Half-Carry in bit 4 (borrow)
        result
    }

    /// Increment 8bit value.
    pub fn inc(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        if result == 0 {
            self.registers.f.set_z(true);
        }
        self.registers.f.set_n(false);
        self.registers.f.set_h((value & 0x0F) + 1 > 0x0F); // Check for Half-Carry
        result
    }

    /// 0x00: null operation
    pub fn nop(&mut self) {}

    pub fn pop(&mut self) -> u16 {
        let value = self.mmu.read_word(self.registers.sp);
        self.registers.sp += 2;
        value
    }

    pub fn push(&mut self, value: u16) {
        self.registers.sp -= 2;
        self.mmu.write_word(self.registers.sp, value);
    }
}
