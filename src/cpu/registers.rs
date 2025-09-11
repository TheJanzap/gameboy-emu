use std::ops::{Shl, Shr};

/// The CPU registers of the Game Boy's CPU.
/// Some registers can be combined to 16-bit registers:
/// `af`, `bc`, `de`, `hl`
#[derive(Default)]
pub(super) struct Registers {
    pub(super) a: u8,
    pub(super) b: u8,
    pub(super) c: u8,
    pub(super) d: u8,
    pub(super) e: u8,
    pub(super) f: FlagsRegister,
    pub(super) h: u8,
    pub(super) l: u8,
}

impl Registers {
    fn get_16_bit_register(&self, high: u8, low: u8) -> u16 {
        ((high as u16) << 8) | (low as u16)
    }
    pub(super) fn get_bc(&self) -> u16 {
        self.get_16_bit_register(self.b, self.c)
    }

    pub(super) fn get_de(&self) -> u16 {
        self.get_16_bit_register(self.d, self.e)
    }

    pub(super) fn get_hl(&self) -> u16 {
        self.get_16_bit_register(self.h, self.l)
    }

    fn set_16_bit_register(high: &mut u8, low: &mut u8, value: u16) {
        *high = ((value & 0xFF00) >> 8) as u8;
        *low = value as u8;
    }

    pub(super) fn set_bc(&mut self, value: u16) {
        Self::set_16_bit_register(&mut self.b, &mut self.c, value)
    }

    pub(super) fn set_de(&mut self, value: u16) {
        Self::set_16_bit_register(&mut self.d, &mut self.e, value);
    }

    pub(super) fn set_hl(&mut self, value: u16) {
        Self::set_16_bit_register(&mut self.h, &mut self.l, value);
    }
}

/// The CPUs Flag register. 1 byte big. The values represent the upper 4 bits in the F register.
/// The lower bits are always zero and can be ignored.
#[derive(Default)]
pub(super) struct FlagsRegister {
    pub(super) zero: bool,
    pub(super) subtract: bool,
    /// Set if adding the lower nibbles of a value and a register together result
    /// in a value bigger than `0xF`.
    pub(super) half_carry: bool,
    pub(super) carry: bool,
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl From<u8> for FlagsRegister {
    fn from(byte: u8) -> FlagsRegister {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

/// Index to address the individual bits 0-7 inside a register.
#[derive(Copy, Clone)]
pub(super) struct U3(u8);

impl U3 {
    pub const MAX: u8 = 0b111; // 7

    /// Create a new valid index in the range `[0..=7]`.
    pub fn wrap(value: u8) -> Self {
        Self(value & Self::MAX)
    }
}

/// Implement the `>>` operator for U3
impl Shr<U3> for u8 {
    type Output = u8;

    fn shr(self, rhs: U3) -> Self::Output {
        self >> rhs.0
    }
}

/// Implement the `>>` operator for U3
impl Shl<U3> for u8 {
    type Output = u8;

    fn shl(self, rhs: U3) -> Self::Output {
        self << rhs.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_and_read_16bit_registers() {
        let mut registers = Registers::default();
        let value = 0xABCD;
        registers.set_bc(value);
        let result = registers.get_bc();
        assert_eq!(result, value);
    }

    #[test]
    fn set_f_as_u8() {
        let mut registers = Registers::default();
        let input = 0b1001_0000;
        registers.f = input.into();
        let result: u8 = registers.f.into();
        assert_eq!(result, input);
    }
}
