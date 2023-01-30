use crate::memory::Memory;

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

    /// Test if selected bit is zero and set ZERO flag.
    pub fn bit(&mut self, register: u8, value: u8) {
        let zero = (register & (1 << value)) == 0;

        self.registers.f.set_z(zero);
        self.registers.f.set_n(false); // reset
        self.registers.f.set_h(true); // set
    }

    /// Push address of next instruction onto stack and then jump to address in the next memory word.
    pub fn call(&mut self) {
        let next_instruction = self.registers.pc.wrapping_add(2);
        self.push(next_instruction);

        self.registers.pc = self.fetch_word();
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

    /// Jump to address provided in the next memory word.
    pub fn jp(&mut self) {
        let address = self.fetch_word();
        self.registers.pc = address;
    }

    /// Jump to PC + (next byte).
    pub fn jr(&mut self) {
        let offset = self.fetch_byte();
        self.registers.pc = self.registers.pc.wrapping_add(offset as u16);
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

    /// Reset bit in register.
    pub fn res(&mut self, register: u8, value: u8) -> u8 {
        let result = register & !(1 << value);
        result
    }

    /// Update PC register to return to instruction stored on the stack.
    pub fn ret(&mut self) {
        self.registers.pc = self.pop();
    }

    /// Rotate left through Carry flag.
    pub fn rl(&mut self, value: u8) -> u8 {
        let carry = (value & 0x80) == 0x80;
        let rotated = (value << 1) | if self.registers.f.c() { 1 } else { 0 };

        self.registers.f.set_z(rotated == 0);
        self.registers.f.set_n(false);
        self.registers.f.set_h(false);
        self.registers.f.set_c(carry);

        rotated
    }

    /// Rotate register A left through Carry flag.
    pub fn rla(&mut self) {
        let carry = (self.registers.a & 0x80) == 0x80;
        self.registers.a = (self.registers.a << 1) | if self.registers.f.c() { 1 } else { 0 };

        self.registers.f.set_z(false);
        self.registers.f.set_n(false);
        self.registers.f.set_h(false);
        self.registers.f.set_c(carry);
    }

    /// Rotate left. Old bit 7 to Carry flag.
    pub fn rlc(&mut self, value: u8) -> u8 {
        let carry = (value & 0x80) == 0x80;
        let rotated = value.rotate_left(1);

        self.registers.f.set_z(rotated == 0);
        self.registers.f.set_n(false);
        self.registers.f.set_h(false);
        self.registers.f.set_c(carry);

        rotated
    }

    /// Rotate register A left. Old bit 7 to Carry flag.
    pub fn rlca(&mut self) {
        let carry = (self.registers.a & 0x80) == 0x80;
        self.registers.a = self.registers.a.rotate_left(1);

        self.registers.f.set_z(false);
        self.registers.f.set_n(false);
        self.registers.f.set_h(false);
        self.registers.f.set_c(carry);
    }

    /// Rotate right through Carry flag.
    pub fn rr(&mut self, value: u8) -> u8 {
        let carry = (value & 0x01) == 0x01;
        let rotated = (value >> 1) | if self.registers.f.c() { 0x80 } else { 0 };

        self.registers.f.set_z(rotated == 0);
        self.registers.f.set_n(false);
        self.registers.f.set_h(false);
        self.registers.f.set_c(carry);

        rotated
    }

    /// Rotate register A right through Carry flag.
    pub fn rra(&mut self) {
        let carry = (self.registers.a & 0x01) == 0x01;
        self.registers.a = (self.registers.a >> 1) | if self.registers.f.c() { 0x80 } else { 0 };

        self.registers.f.set_z(false);
        self.registers.f.set_n(false);
        self.registers.f.set_h(false);
        self.registers.f.set_c(carry);
    }

    /// Rotate right. Old bit 0 to Carry flag.
    pub fn rrc(&mut self, value: u8) -> u8 {
        let carry = (value & 0x01) == 0x01;
        let rotated = value.rotate_right(1);

        self.registers.f.set_z(rotated == 0);
        self.registers.f.set_n(false);
        self.registers.f.set_h(false);
        self.registers.f.set_c(carry);

        rotated
    }

    /// Rotate register A right. Old bit 0 to Carry flag.
    pub fn rrca(&mut self) {
        let carry = (self.registers.a & 0x01) == 0x01;
        self.registers.a = self.registers.a.rotate_right(1);

        self.registers.f.set_z(false);
        self.registers.f.set_n(false);
        self.registers.f.set_h(false);
        self.registers.f.set_c(carry);
    }

    /// Push present address onto stack and jump to address 0x0000 + arg.
    pub fn rst(&mut self, address: u16) {
        self.push(self.registers.pc);
        self.registers.pc = 0x0000 + address;
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

    /// Set bit in register.
    pub fn set(&mut self, register: u8, value: u8) -> u8 {
        let result = register | (1 << value);
        result
    }

    /// Shift left into Carry. LSB of set to 0.
    pub fn sla(&mut self, value: u8) -> u8 {
        let carry = (value & 0x80) == 0x80;
        let shifted = value << 1;

        self.registers.f.set_z(shifted == 0);
        self.registers.f.set_n(false);
        self.registers.f.set_h(false);
        self.registers.f.set_c(carry);

        shifted
    }

    /// Shift right into Carry. MSB doesn't change.
    pub fn sra(&mut self, value: u8) -> u8 {
        let carry = (value & 0x01) == 0x01;
        let shifted = value >> 1 | (value & 0x80); // negative values, preserve MSB.

        self.registers.f.set_z(shifted == 0);
        self.registers.f.set_n(false);
        self.registers.f.set_h(false);
        self.registers.f.set_c(carry);

        shifted
    }

    /// Shift right into Carry. MSB set to 0.
    pub fn srl(&mut self, value: u8) -> u8 {
        let carry = (value & 0x01) == 0x01;
        let shifted = value >> 1;

        self.registers.f.set_z(shifted == 0);
        self.registers.f.set_n(false);
        self.registers.f.set_h(false);
        self.registers.f.set_c(carry);

        shifted
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

    /// Swap upper & lower nibles.
    pub fn swap(&mut self, value: u8) -> u8 {
        self.registers.f.set_z(value == 0);
        self.registers.f.set_n(false);
        self.registers.f.set_h(false);
        self.registers.f.set_c(false);
        (value >> 4) | (value << 4)
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

// #[cfg(test)]
// mod tests {
//     use crate::cpu::CPU;
//     use crate::mmu::MMU;

//     fn assert_flags(cpu: CPU, z: bool, n: bool, h: bool, c: bool) {
//         let flags = cpu.registers.f;
//         println!("Flags: {:?}", flags.0);
//         assert_eq!(flags.z(), z);
//         assert_eq!(flags.n(), n);
//         assert_eq!(flags.h(), h);
//         assert_eq!(flags.c(), c);
//     }

//     // INC
//     #[test]
//     fn test_inc() {
//         let mut cpu = CPU::new();
//         cpu.registers.a = 0x07;

//         cpu.registers.a = cpu.inc(cpu.registers.a);

//         assert_eq!(cpu.registers.a, 0x08);
//         assert_flags(cpu, false, false, false, false);
//     }

//     #[test]
//     fn test_inc_half_carry() {
//         let mut cpu = CPU::new();
//         cpu.registers.a = 0x0F;

//         cpu.registers.a = cpu.inc(cpu.registers.a);

//         assert_eq!(cpu.registers.a, 0x10);
//         assert_flags(cpu, false, false, true, false);
//     }

//     #[test]
//     fn test_inc_overflow() {
//         let mut cpu = CPU::new();
//         cpu.registers.a = 0xFF;

//         cpu.registers.a = cpu.inc(cpu.registers.a);

//         assert_eq!(cpu.registers.a, 0x00);
//         assert_flags(cpu, true, false, true, false);
//     }

//     #[test]
//     fn test_dec() {
//         let mut cpu = CPU::new();
//         cpu.registers.a = 0x07;

//         cpu.registers.a = cpu.dec(cpu.registers.a);

//         assert_eq!(cpu.registers.a, 0x06);
//         assert_flags(cpu, false, true, false, false);
//     }

//     #[test]
//     fn test_dec_half_carry() {
//         let mut cpu = CPU::new();
//         cpu.registers.a = 0x80;

//         cpu.registers.a = cpu.dec(cpu.registers.a);

//         assert_eq!(cpu.registers.a, 0x7F);
//         assert_flags(cpu, false, true, true, false);
//     }

//     #[test]
//     fn test_dec_overflow() {
//         let mut cpu = CPU::new();
//         cpu.registers.a = 0x00;

//         cpu.registers.a = cpu.dec(cpu.registers.a);

//         assert_eq!(cpu.registers.a, 0xFF);
//         assert_flags(cpu, false, true, true, false);
//     }
// }
