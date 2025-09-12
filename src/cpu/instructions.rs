mod opcodes;
pub(crate) mod parameter;

use super::Cpu;
use super::registers::U3;
use parameter::{
    JumpTest, LoadByteSource, LoadByteTarget, LoadType, StackTarget, TargetRegister8,
    TargetRegister16,
};

/// The assembly instructions the emulator can execute.
pub(super) enum Instruction {
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
    /// Jumps to a specified address if the specified condition is met:
    /// Zero flag set/not set, Carry flag set/not set or always jump
    Jp(JumpTest),
    /// Load a value into a register or memory location
    Ld(LoadType),
    /// Push register r16 into the stack.
    Push(StackTarget),
    /// Pop register r16 from the stack.
    Pop(StackTarget),
    /// Call address n16 if condition cc is met. This pushes the address of the instruction after
    /// the CALL on the stack, such that RET can pop it later; then, it executes an implicit JP n16.
    Call(JumpTest),
    /// Return from subroutine if condition is met.
    /// This is basically a `POP PC` (if such an instruction existed).
    Ret(JumpTest),
    /// No Operation. Does nothing.
    Nop,
    /// Enter CPU low-power consumption mode until an interrupt occurs.
    /// In our case, will set [Cpu::is_halted] to true and end the [Cpu::execute] cycle.
    Halt,
}

impl Instruction {
    /// Convert a byte stored in memory into an Instruction.
    /// If `prefixed` is set, the byte will be interpreted as the start of a prefix instruction.
    /// Returns [`None`] if the opcode is invalid.
    pub(super) fn from_byte(byte: u8, prefixed: bool) -> Option<Self> {
        match prefixed {
            true => opcodes::get_opcode_unprefixed(byte),
            false => Some(opcodes::get_opcode_prefixed(byte)),
        }
    }
}

impl Cpu {
    /// Execute an instruction on the CPU
    pub(super) fn execute(&mut self, instruction: Instruction) -> u16 {
        if self.is_halted {
            return 0;
        }

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
            Instruction::Jp(condition) => return self.jump(condition),
            Instruction::Ld(load_type) => return self.load(load_type),
            Instruction::Push(r16) => self.push(self.get_stack_target_value(r16)),
            Instruction::Pop(r16) => {
                let res = self.pop();
                self.set_stack_target_value(r16, res)
            }
            Instruction::Call(condition) => return self.call(condition),
            Instruction::Ret(condition) => return self.ret(condition),
            Instruction::Nop => (),
            Instruction::Halt => self.is_halted = true,
        };
        // Increment the program counter by one.
        // Instructions that modify the PC differently return early.
        self.pc.wrapping_add(1)
    }

    /// Executes [`Instruction::Add`].
    fn add_a(&mut self, target: TargetRegister8) {
        let value = self.get_r8_value(target);
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.a = new_value;
    }

    /// Executes [`Instruction::AddHl`].
    fn add_hl(&mut self, target: TargetRegister16) {
        let value = self.get_r16_value(target);
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.get_hl() & 0xF) + (value & 0xF) > 0xF;
        self.registers.set_hl(new_value);
    }

    /// Executes [`Instruction::Adc`].
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

    /// Executes [`Instruction::Cp`].
    /// Returns the value so the implementation can be reused by [Instruction::Sub].
    fn compare(&mut self, target: TargetRegister8) -> u8 {
        let value = self.get_r8_value(target);
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (new_value & 0xF) >= 0xF;
        new_value
    }

    /// Executes [`Instruction::Sub`].
    fn sub(&mut self, target: TargetRegister8) {
        // Call compare logic and set the result
        self.registers.a = self.compare(target);
    }

    /// Executes [`Instruction::Sbc`].
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

    /// Executes [`Instruction::And`].
    fn and(&mut self, target: TargetRegister8) {
        let value = self.get_r8_value(target);
        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;
        self.registers.a &= value;
    }

    /// Executes [`Instruction::Or`].
    fn or(&mut self, target: TargetRegister8) {
        let value = self.get_r8_value(target);
        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        self.registers.a |= value;
    }

    /// Executes [`Instruction::Xor`].
    fn xor(&mut self, target: TargetRegister8) {
        let value = self.get_r8_value(target);
        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        self.registers.a ^= value;
    }

    /// Executes [`Instruction::Inc`].
    fn increment(&mut self, target: TargetRegister8) {
        let register = self.get_r8_ref(target);

        *register += 1;
        let new_value = *register;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = ((new_value - 1) & 0xF) + 1 > 0xF;
    }

    /// Executes [`Instruction::Dec`].
    fn decrement(&mut self, target: TargetRegister8) {
        let register = self.get_r8_ref(target);

        *register -= 1;
        let new_value = *register;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (new_value & 0xF) + ((new_value + 1) & 0xF) >= 0xF;
    }

    /// Executes [`Instruction::Ccf`].
    fn invert_carry_flag(&mut self) {
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = !self.registers.f.carry;
    }

    /// Executes [`Instruction::Scf`].
    fn set_carry_flag(&mut self) {
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = true;
    }

    /// Executes [`Instruction::Cpl`].
    fn complement_a(&mut self) {
        self.registers.f.subtract = true;
        self.registers.f.half_carry = true;
        self.registers.a = !self.registers.a;
    }

    /// Executes [`Instruction::Bit`].
    fn test_bit(&mut self, index: U3, target: TargetRegister8) {
        let value = self.get_r8_value(target);
        let mask = 1 << index;
        let is_bit_set = ((value & mask) >> index) == 0;

        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.zero = !is_bit_set;
    }

    /// Executes [`Instruction::Res`].
    fn unset_bit(&mut self, index: U3, target: TargetRegister8) {
        let register = self.get_r8_ref(target);
        let zero_bit = !(1 << index);
        *register &= zero_bit;
    }

    /// Executes [`Instruction::Set`].
    fn set_bit(&mut self, index: U3, target: TargetRegister8) {
        let register = self.get_r8_ref(target);
        let one_bit = 1 << index;
        *register |= one_bit;
    }

    /// Executes [`Instruction::Srl`].
    fn shift_right_logically(&mut self, target: TargetRegister8) {
        let register = self.get_r8_ref(target);
        let lsb = *register & 0b0000_0001;

        *register >>= 1;
        self.registers.f.zero = *register == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = lsb == 1;
    }

    /// Executes [`Instruction::Rr`].
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

    /// Executes [`Instruction::Rla`].
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

    /// Executes [`Instruction::Rrc`].
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

    /// Executes [`Instruction::Rlc`].
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

    /// Executes [`Instruction::Sra`].
    fn shift_right_arithmetically(&mut self, target: TargetRegister8) {
        let register = self.get_r8_ref(target);
        let lsb = *register & 0b0000_0001;

        *register >>= 1;
        self.registers.f.zero = *register == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = lsb == 1;
    }

    /// Executes [`Instruction::Sla`].
    fn shift_left_arithmetically(&mut self, target: TargetRegister8) {
        let register = self.get_r8_ref(target);
        let msb = *register & 0b1000_0000;

        *register <<= 1;
        self.registers.f.zero = *register == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = msb == 128;
    }

    /// Executes [`Instruction::Swap`].
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

    /// Executes [`Instruction::Jp`].
    fn jump(&mut self, condition: JumpTest) -> u16 {
        let should_jump = self.get_jump_test_result(condition);
        if should_jump {
            // The Game Boy is little endian: Read lsb first
            let lsb = self.bus.read_byte(self.pc + 1) as u16;
            let msb = self.bus.read_byte(self.pc + 2) as u16;
            (msb << 8) | lsb
        } else {
            // Condition not met, move to the next instruction
            // A jump instruction is 3 bytes wide (1 byte tag, 2 bytes jump address)
            self.pc.wrapping_add(3)
        }
    }

    /// Executes [`Instruction::Ld`].
    fn load(&mut self, load_type: LoadType) -> u16 {
        match load_type {
            LoadType::Byte(target, source) => {
                let source_value = match source {
                    LoadByteSource::A => self.registers.a,
                    LoadByteSource::B => self.registers.b,
                    LoadByteSource::C => self.registers.c,
                    LoadByteSource::D => self.registers.d,
                    LoadByteSource::E => self.registers.e,
                    LoadByteSource::H => self.registers.h,
                    LoadByteSource::L => self.registers.l,
                    LoadByteSource::D8 => self.read_next_byte(),
                    LoadByteSource::Hli => self.bus.read_byte(self.registers.get_hl()),
                };

                match target {
                    LoadByteTarget::A => self.registers.a = source_value,
                    LoadByteTarget::B => self.registers.b = source_value,
                    LoadByteTarget::C => self.registers.c = source_value,
                    LoadByteTarget::D => self.registers.d = source_value,
                    LoadByteTarget::E => self.registers.e = source_value,
                    LoadByteTarget::H => self.registers.h = source_value,
                    LoadByteTarget::L => self.registers.l = source_value,
                    LoadByteTarget::Hli => {
                        self.bus.write_byte(self.registers.get_hl(), source_value)
                    }
                }

                match source {
                    LoadByteSource::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1),
                }
            }
            _ => todo!(),
        }
    }

    /// Executes [`Instruction::Push`].
    fn push(&mut self, value: u16) {
        // Decrease the SP and write MSB of value into memory at location of SP
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        // Decrease the SP and write LSB of value into memory at location of SP
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0x00FF) as u8);
    }

    /// Executes [`Instruction::Pop`].
    fn pop(&mut self) -> u16 {
        // Read LSB of value from memory at location of SP and increase the SP
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        // Read MSB of value from memory at location of SP and increase the SP
        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

    /// Executes [`Instruction::Call`].
    fn call(&mut self, condition: JumpTest) -> u16 {
        let should_jump = self.get_jump_test_result(condition);
        // Set the PC to the instruction after the 3-byte wide `Call` instruction
        let next_pc = self.sp.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            self.read_next_word()
        } else {
            next_pc
        }
    }

    /// Executes [`Instruction::Ret`].
    fn ret(&mut self, condition: JumpTest) -> u16 {
        let should_jump = self.get_jump_test_result(condition);
        if should_jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }
}

#[cfg(test)]
mod tests;
