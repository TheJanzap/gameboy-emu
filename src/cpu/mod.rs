use crate::memory_bus::MemoryBus;
use instructions::{
    Instruction,
    parameter::{TargetRegister8, TargetRegister16},
};
use registers::Registers;

mod instructions;
mod registers;

/// Byte that indicates a prefix instruction.
const PREFIX_BYTE: u8 = 0xCB;

#[derive(Default)]
struct Cpu {
    registers: Registers,
    /// The program counter of the CPU.
    pc: u16,
    bus: MemoryBus,
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
}
