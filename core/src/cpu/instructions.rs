impl super::CPU {
    /// 8-bit add operation with register A.
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

    /// 16-bit add operation with register HL.
    pub fn add16_hl(&mut self, value: u16) {
        let (result, carry) = self.registers.hl().overflowing_add(value);
        self.registers.f.set_n(false);
        self.registers
            .f
            .set_h((self.registers.hl() & 0x0FFF) + (value & 0x0FFF) > 0x0FFF);
        self.registers.f.set_c(carry);
        self.registers.set_hl(result);
    }

    /// 16-bit add operation of a byte with register SP.
    pub fn add16_sp(&mut self, value: u8) {
        let value_16 = value as u16;
        let result = self.registers.sp.wrapping_add(value_16);
        self.registers.f.set_z(false);
        self.registers.f.set_n(false);
        self.registers
            .f
            .set_h((self.registers.sp & 0x000F) + (value_16 & 0x000F) > 0x000F);
        self.registers
            .f
            .set_c((self.registers.sp & 0x00FF) + (value_16 & 0x00FF) > 0x00FF);
        self.registers.sp = result;
    }

    /// 8-bit add with carry operation with register A.
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

    /// Compare A with a value. This is basically an A - value subtraction instruction but the results are thrown away.
    pub fn cp(&mut self, value: u8) {
        let old = self.registers.a;
        self.sub(value);
        self.registers.a = old;
    }

    /// Complement carry flag.
    pub fn ccf(&mut self) {
        self.registers.f.set_n(false);
        self.registers.f.set_h(false);
        if self.registers.f.c() {
            self.registers.f.set_c(false);
        } else {
            self.registers.f.set_c(true);
        }
    }

    /// Complement A register (Flip all bits).
    pub fn cpl(&mut self) {
        self.registers.a = !self.registers.a;
        self.registers.f.set_n(true);
        self.registers.f.set_h(true);
    }

    /** Decimal adjust register A. This instruction adjusts register A so that the
       correct representation of Binary Coded Decimal (BCD) is obtained.
    */
    pub fn daa(&mut self) {
        let mut current_a = self.registers.a;
        let mut adjust = if self.registers.f.c() { 0x60 } else { 0x00 };
        if self.registers.f.h() {
            adjust |= 0x06;
        };
        if !self.registers.f.n() {
            if current_a & 0x0F > 0x09 {
                adjust |= 0x06;
            };
            if current_a > 0x99 {
                adjust |= 0x60;
            };
            current_a = current_a.wrapping_add(adjust);
        } else {
            current_a = current_a.wrapping_sub(adjust);
        }
        self.registers.f.set_z(current_a == 0);
        self.registers.f.set_h(false);
        self.registers.f.set_c(adjust >= 0x60);
        self.registers.a = current_a;
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

    /// Decrement 16-bit value.
    pub fn dec16(&mut self, value: u16) -> u16 {
        let result = value.wrapping_sub(1);
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

    /// Increment 16bit value.
    pub fn inc16(&mut self, value: u16) -> u16 {
        let result = value.wrapping_add(1);
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

    /// Sub operation with register A.
    pub fn sub(&mut self, value: u8) {
        let (result, carry) = self.registers.a.overflowing_sub(value);
        self.registers.f.set_z(result == 0);
        self.registers.f.set_n(true);
        self.registers
            .f
            .set_h((self.registers.a & 0x0F) < (value & 0x0F));
        self.registers.f.set_c(carry);
        self.registers.a = result;
    }

    /// Sub operation with carry with register A.
    pub fn sbc(&mut self, value: u8) {
        let carry = if self.registers.f.c() { 1 } else { 0 };
        let result = self.registers.a.wrapping_sub(value).wrapping_sub(carry);
        self.registers.f.set_z(result == 0);
        self.registers.f.set_n(true);

        self.registers
            .f
            .set_h((self.registers.a & 0x0F) < (value & 0x0F) + carry);
        self.registers
            .f
            .set_h(u16::from(self.registers.a) < u16::from(value) + u16::from(carry));
        self.registers.a = result;
    }

    // Set carry flag.
    pub fn scf(&mut self) {
        self.registers.f.set_n(false);
        self.registers.f.set_h(false);
        self.registers.f.set_c(true);
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
