use crate::cpu::registers::{Registers, U3};

/// The assembly instructions the emulator can execute.
enum Instruction {
    /// Add the value in r8 to A
    Add(TargetRegister8),
    /// Add the value in r16 to HL
    AddHl(TargetRegister16),
    /// Add the value in r8 plus the carry flag to A
    Adc(TargetRegister8),
    /// Subtract the value in r8 from A
    Sub(TargetRegister8),
    /// Subtract the value in r8 and the carry flag from A
    Sbc(TargetRegister8),
    /// Subtract the value in r8, but don't store the result
    Cp(TargetRegister8),
    /// Set A to the bitwise AND between the value in r8 and A
    And(TargetRegister8),
    /// Set A to the bitwise OR between the value in r8 and A
    Or(TargetRegister8),
    /// Set A to the bitwise XOR between the value in r8 and A
    Xor(TargetRegister8),
    /// Increment the value in register r8 by 1
    Inc(TargetRegister8),
    /// Decrement the value in register r8 by 1
    Dec(TargetRegister8),
    /// Complement Carry flag: Invert the Carry flag
    Ccf,
    /// Set Carry Flag to true
    Scf,
    /// Complement of A: Bitwise not
    Cpl,
    /// Test if bit u3 in register r8 is set. Changes the zero flag
    Bit(U3, TargetRegister8),
    /// Set bit u3 in register r8 to 0
    Res(U3, TargetRegister8),
    /// Set bit u3 in register r8 to 1
    Set(U3, TargetRegister8),
    /// Rotate a register r8 right, through the carry flag
    Rr(TargetRegister8),
    /// Rotate a register r8 left, through the carry flag
    Rl(TargetRegister8),
    /// Rotate a register r8 right, not through the carry flag
    Rrc(TargetRegister8),
    /// Rotate a register r8 left, not through the carry flag
    Rlc(TargetRegister8),
    /// Rotate register A right, through the carry flag
    Rra,
    /// Rotate register A left, through the carry flag
    Rla,
    /// Rotate register A right, not through the carry flag
    Rrca,
    /// Rotate register A left, not through the carry flag
    Rlca,
    /// Shift Right Logically register r8
    Srl(TargetRegister8),
    /// Shift Right Arithmetically register r8
    Sra(TargetRegister8),
    /// Shift Left Arithmetically register r8
    Sla(TargetRegister8),
    /// Swap the upper 4 bits with the lower 4 ones in r8
    Swap(TargetRegister8),
}

/// Which 8-bit register an instruction should affect. Note that F is missing.
enum TargetRegister8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

/// Combined 16-bit registers
enum TargetRegister16 {
    BC,
    DE,
    HL,
}

#[derive(Default)]
struct Cpu {
    registers: Registers,
}

impl Cpu {
    /// Execute an instruction on the CPU
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add(r8) => self.add_a(r8),
            Instruction::AddHl(r8) => self.add_hl(r8),
            Instruction::Adc(r8) => self.add_with_carry(r8),
            Instruction::Sub(r8) => self.sub(r8),
            Instruction::Sbc(r8) => self.sub_with_carry(r8),
            Instruction::Cp(r8) => _ = self.compare(r8),
            Instruction::And(r8) => self.and(r8),
            Instruction::Or(r8) => self.or(r8),
            Instruction::Xor(r8) => self.xor(r8),
            Instruction::Inc(r8) => self.increment(r8),
            Instruction::Dec(r8) => self.decrement(r8),
            Instruction::Ccf => self.invert_carry_flag(),
            Instruction::Scf => self.set_carry_flag(),
            Instruction::Cpl => self.complement_a(),
            Instruction::Bit(index, r8) => self.test_bit(index, r8),
            Instruction::Res(index, r8) => self.unset_bit(index, r8),
            Instruction::Set(index, r8) => self.set_bit(index, r8),
            Instruction::Rr(r8) => self.rotate_right_with_carry(r8),
            Instruction::Rl(r8) => self.rotate_left_with_carry(r8),
            Instruction::Rrc(r8) => self.rotate_right_no_carry(r8),
            Instruction::Rlc(r8) => self.rotate_left_no_carry(r8),
            Instruction::Rla => self.rotate_left_with_carry(TargetRegister8::A),
            Instruction::Rrca => self.rotate_right_no_carry(TargetRegister8::A),
            Instruction::Rlca => self.rotate_left_no_carry(TargetRegister8::A),
            Instruction::Rra => self.rotate_right_with_carry(TargetRegister8::A),
            Instruction::Srl(r8) => self.shift_right_logically(r8),
            Instruction::Sra(r8) => self.shift_right_arithmetically(r8),
            Instruction::Sla(r8) => self.shift_left_arithmetically(r8),
            Instruction::Swap(r8) => self.swap(r8),
        }
    }

    /// Gets the value of an 8-bit register
    fn get_r8_value(&self, target: TargetRegister8) -> u8 {
        match target {
            TargetRegister8::A => self.registers.a,
            TargetRegister8::B => self.registers.b,
            TargetRegister8::C => self.registers.c,
            TargetRegister8::D => self.registers.d,
            TargetRegister8::E => self.registers.e,
            TargetRegister8::H => self.registers.h,
            TargetRegister8::L => self.registers.l,
        }
    }

    /// Gets a reference to an 8-bit register. Useful when the register needs to be written to.
    fn get_r8_ref(&mut self, target: TargetRegister8) -> &mut u8 {
        match target {
            TargetRegister8::A => &mut self.registers.a,
            TargetRegister8::B => &mut self.registers.b,
            TargetRegister8::C => &mut self.registers.c,
            TargetRegister8::D => &mut self.registers.d,
            TargetRegister8::E => &mut self.registers.e,
            TargetRegister8::H => &mut self.registers.h,
            TargetRegister8::L => &mut self.registers.l,
        }
    }

    fn get_r16_value(&self, target: TargetRegister16) -> u16 {
        match target {
            TargetRegister16::BC => self.registers.get_bc(),
            TargetRegister16::DE => self.registers.get_de(),
            TargetRegister16::HL => self.registers.get_hl(),
        }
    }

    fn add_a(&mut self, target: TargetRegister8) {
        let value = self.get_r8_value(target);
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.a = new_value;
    }

    fn add_hl(&mut self, target: TargetRegister16) {
        let value = self.get_r16_value(target);
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.get_hl() & 0xF) + (value & 0xF) > 0xF;
        self.registers.set_hl(new_value);
    }

    fn add_with_carry(&mut self, target: TargetRegister8) {
        let old_carry = if self.registers.f.carry { 1 } else { 0 };
        let value = self.get_r8_value(target) + old_carry;
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.a = new_value;
    }

    fn compare(&mut self, target: TargetRegister8) -> u8 {
        let value = self.get_r8_value(target);
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (new_value & 0xF) >= 0xF;
        new_value
    }

    fn sub(&mut self, target: TargetRegister8) {
        // Call compare logic and set the result
        self.registers.a = self.compare(target);
    }

    fn sub_with_carry(&mut self, target: TargetRegister8) {
        let old_carry = if self.registers.f.carry { 1 } else { 0 };
        let value = self.get_r8_value(target) + old_carry;
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (new_value & 0xF) >= 0xF;
        self.registers.a = new_value;
    }

    fn and(&mut self, target: TargetRegister8) {
        let value = self.get_r8_value(target);
        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;
        self.registers.a &= value;
    }

    fn or(&mut self, target: TargetRegister8) {
        let value = self.get_r8_value(target);
        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        self.registers.a |= value;
    }

    fn xor(&mut self, target: TargetRegister8) {
        let value = self.get_r8_value(target);
        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        self.registers.a ^= value;
    }

    fn increment(&mut self, target: TargetRegister8) {
        let register = self.get_r8_ref(target);

        *register += 1;
        let new_value = *register;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = ((new_value - 1) & 0xF) + 1 > 0xF;
    }

    fn decrement(&mut self, target: TargetRegister8) {
        let register = self.get_r8_ref(target);

        *register -= 1;
        let new_value = *register;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (new_value & 0xF) + ((new_value + 1) & 0xF) >= 0xF;
    }

    fn invert_carry_flag(&mut self) {
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = !self.registers.f.carry;
    }

    fn set_carry_flag(&mut self) {
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = true;
    }

    fn complement_a(&mut self) {
        self.registers.f.subtract = true;
        self.registers.f.half_carry = true;
        self.registers.a = !self.registers.a;
    }

    fn test_bit(&mut self, index: U3, target: TargetRegister8) {
        let value = self.get_r8_value(target);
        let mask = 1 << index;
        let is_bit_set = ((value & mask) >> index) == 0;

        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.zero = !is_bit_set;
    }

    fn unset_bit(&mut self, index: U3, target: TargetRegister8) {
        let register = self.get_r8_ref(target);
        let zero_bit = !(1 << index);
        *register &= zero_bit;
    }

    fn set_bit(&mut self, index: U3, target: TargetRegister8) {
        let register = self.get_r8_ref(target);
        let one_bit = 1 << index;
        *register |= one_bit;
    }

    fn shift_right_logically(&mut self, target: TargetRegister8) {
        let register = self.get_r8_ref(target);
        let lsb = *register & 0b0000_0001;

        *register >>= 1;
        self.registers.f.zero = *register == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = lsb == 1;
    }

    fn rotate_right_with_carry(&mut self, target: TargetRegister8) {
        let old_carry: u8 = if self.registers.f.carry { 1 } else { 0 };
        let register = self.get_r8_ref(target);
        let is_lsb_set = (*register & 0b0000_0001) == 1;
        let shifted = *register >> 1;

        *register = (old_carry << 7) | shifted;
        self.registers.f.zero = *register == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = is_lsb_set;
    }

    fn rotate_left_with_carry(&mut self, target: TargetRegister8) {
        let old_carry: u8 = if self.registers.f.carry { 1 } else { 0 };
        let register = self.get_r8_ref(target);
        let is_msb_set = (*register & 0b1000_0000) == 128;
        let shifted = *register << 1;

        *register = old_carry | shifted;
        self.registers.f.zero = *register == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = is_msb_set;
    }

    fn rotate_right_no_carry(&mut self, target: TargetRegister8) {
        let register = self.get_r8_ref(target);
        let lsb = *register & 0b0000_0001;
        let shifted = *register >> 1;

        *register = (lsb << 7) | shifted;
        self.registers.f.zero = *register == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = lsb == 1;
    }

    fn rotate_left_no_carry(&mut self, target: TargetRegister8) {
        let register = self.get_r8_ref(target);
        let msb = *register & 0b1000_0000;
        let shifted = *register << 1;

        *register = msb | shifted;
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = msb == 128;
    }

    fn shift_right_arithmetically(&mut self, target: TargetRegister8) {
        let register = self.get_r8_ref(target);
        let lsb = *register & 0b0000_0001;

        *register >>= 1;
        self.registers.f.zero = *register == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = lsb == 1;
    }

    fn shift_left_arithmetically(&mut self, target: TargetRegister8) {
        let register = self.get_r8_ref(target);
        let msb = *register & 0b1000_0000;

        *register <<= 1;
        self.registers.f.zero = *register == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = msb == 128;
    }

    fn swap(&mut self, target: TargetRegister8) {
        let register = self.get_r8_ref(target);
        let new_upper = (*register & 0b0000_1111) << 4;
        let new_lower = (*register & 0b1111_0000) >> 4;

        *register = new_upper | new_lower;
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
    }
}

#[cfg(test)]
mod tests;
