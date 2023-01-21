impl super::CPU {
    /// Add operation with register A.
    pub fn add(&mut self, value: u8) {
        let (result, carry) = self.registers.a.overflowing_add(value);
        self.registers.f.set_z(result == 0);
        self.registers.f.set_n(false);
        self.registers
            .f
            .set_h((self.registers.a & 0x0F) + (value & 0x0F) > 0x0F);
        self.registers.f.set_c(carry);
        self.registers.a = result;
    }

    /// Add with carry operation with register A.
    pub fn adc(&mut self, value: u8) {
        let carry = if self.registers.f.c() { 1 } else { 0 };
        let result = self.registers.a.wrapping_add(value).wrapping_add(carry);
        self.registers.f.set_z(result == 0);
        self.registers.f.set_n(false);
        self.registers
            .f
            .set_h((self.registers.a & 0x0F) + (value & 0x0F) + (carry & 0x0F) > 0x0F);
        self.registers
            .f
            .set_c(u16::from(self.registers.a) + u16::from(value) + u16::from(carry) > 0xFF);
        self.registers.a = result;
    }

    /// Bitwise AND operation with register A.
    pub fn and(&mut self, value: u8) {
        let result = self.registers.a & value;
        if result == 0 {
            self.registers.f.set_z(true);
        }
        self.registers.a = result;
        self.registers.f.set_n(false); // reset
        self.registers.f.set_h(true); // set
        self.registers.f.set_c(false); // reset
    }

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

    /// Bitwise OR operation with register A.
    pub fn or(&mut self, value: u8) {
        let result = self.registers.a | value;
        if result == 0 {
            self.registers.f.set_z(true);
        }
        self.registers.a = result;
        self.registers.f.set_n(false); // reset
        self.registers.f.set_h(false); // reset
        self.registers.f.set_c(false); // reset
    }

    /// Pop a 16bit value from the stack.
    pub fn pop(&mut self) -> u16 {
        let value = self.mmu.read_word(self.registers.sp);
        self.registers.sp += 2;
        value
    }

    /// Push a 16bit value to the stack.
    pub fn push(&mut self, value: u16) {
        self.registers.sp -= 2;
        self.mmu.write_word(self.registers.sp, value);
    }

    /// Bitwise XOR operation with register A.
    pub fn xor(&mut self, value: u8) {
        let result = self.registers.a ^ value;
        if result == 0 {
            self.registers.f.set_z(true);
        }
        self.registers.a = result;
        self.registers.f.set_n(false); // reset
        self.registers.f.set_h(false); // reset
        self.registers.f.set_c(false); // reset
    }
}
