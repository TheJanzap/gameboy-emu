use crate::memory_bus::MemoryBus;
use instructions::{
    Instruction,
    parameter::{JumpTest, StackTarget, TargetRegister8, TargetRegister16},
};
use registers::Registers;

mod instructions;
mod registers;

/// Byte that indicates a prefix instruction.
const PREFIX_BYTE: u8 = 0xCB;

struct Cpu {
    registers: Registers,
    /// The program counter of the CPU.
    pc: u16,
    /// The stack pointer of the CPU.
    sp: u16,
    bus: MemoryBus,
    /// Set by [`Instruction::Halt`]. Is checked every cycle.
    is_halted: bool,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            registers: Registers::default(),
            pc: u16::default(),
            sp: u16::MAX,
            bus: MemoryBus::default(),
            is_halted: bool::default(),
        }
    }
}

impl Cpu {
    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let is_prefixed = instruction_byte == PREFIX_BYTE;
        if is_prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc =
            if let Some(instruction) = Instruction::from_byte(instruction_byte, is_prefixed) {
                self.execute(instruction)
            } else {
                let description = format!(
                    "0x{}{instruction_byte:x}",
                    if is_prefixed { "cb" } else { "" }
                );
                panic!("Unknown instruction found for: {}", description)
            };
        self.pc = next_pc;
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

    /// Gets the value of an 16-bit register.
    fn get_r16_value(&self, target: TargetRegister16) -> u16 {
        match target {
            TargetRegister16::BC => self.registers.get_bc(),
            TargetRegister16::DE => self.registers.get_de(),
            TargetRegister16::HL => self.registers.get_hl(),
        }
    }

    /// Reads the next byte in memory.
    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc.wrapping_add(1))
    }

    /// Reads the next two bytes in memory and combines them to a 16-bit value.
    fn read_next_word(&self) -> u16 {
        ((self.bus.read_byte(self.pc + 2) as u16) << 8) | (self.bus.read_byte(self.pc + 1) as u16)
    }

    /// Gets the value associated with each [`Instruction`]s [JumpTest].
    fn get_jump_test_result(&self, condition: JumpTest) -> bool {
        match condition {
            JumpTest::Zero => self.registers.f.zero,
            JumpTest::NotZero => !self.registers.f.zero,
            JumpTest::Carry => self.registers.f.carry,
            JumpTest::NotCarry => !self.registers.f.carry,
            JumpTest::Always => true,
        }
    }

    /// Gets the value that should be pushed on the stack from the specified register.
    fn get_stack_target_value(&self, target: StackTarget) -> u16 {
        match target {
            StackTarget::AF => self.registers.get_af(),
            StackTarget::BC => self.registers.get_bc(),
            StackTarget::DE => self.registers.get_de(),
            StackTarget::HL => self.registers.get_hl(),
        }
    }

    /// Sets the value that is popped of the stack into the specified register.
    fn set_stack_target_value(&mut self, target: StackTarget, value: u16) {
        match target {
            StackTarget::AF => self.registers.set_af(value),
            StackTarget::BC => self.registers.set_bc(value),
            StackTarget::DE => self.registers.set_de(value),
            StackTarget::HL => self.registers.set_hl(value),
        }
    }
}
