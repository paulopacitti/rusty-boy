use crate::cpu::CPU;

impl CPU {
    /// 0x00: null operation
    pub fn nop(&mut self) {}

    // 0x04: increment 8bit value
    pub fn inc(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        if result == 0 {
            self.registers.f.set_z(true);
        }
        self.registers.f.set_n(false);
        self.registers.f.set_h((value & 0x0F) + 1 > 0x0F); // Check for Half-Carry
        result
    }
}
