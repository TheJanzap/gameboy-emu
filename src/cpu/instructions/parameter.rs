//! Contains all parameter types for the CPU instructions.

/// Which 8-bit register an instruction should affect.
/// Note that F is missing, as it cannot be the target of an Instruction.
pub(crate) enum TargetRegister8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

/// Combined 16-bit registers
pub(crate) enum TargetRegister16 {
    BC,
    DE,
    HL,
}

/// What flag state a jump should check.
pub(super) enum JumpTest {
    /// Jump if the zero flag is not set.
    NotZero,
    /// Jump if the zero flag is set.
    Zero,
    /// Jump if the carry flag is not set.
    NotCarry,
    /// Jump if the carry flag is set.
    Carry,
    /// Jump unconditionally.
    Always,
}

/// Different ways [crate::cpu::instructions::Instruction] can load data.
pub(super) enum LoadType {
    /// Load 8-bit values from one place to another.
    Byte(LoadByteTarget, LoadByteSource),
    /// Load 16-bit values from one place to another.
    Word,
    /// Load the contents of address into the `A` register.
    AFromIndirect,
    /// Load the contents of the `A` register into the location of address
    IndirectFromA,
    /// Load the contents of the memory address stored at the very last byte of memory
    /// into register `A`.
    AFromByteAddress,
    /// Load the contents of the `A` register into the location of the address stored at the
    /// very last byte of memory.
    ByteAddressFromA,
}

pub(super) enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    /// HL Incremented, the value in HL is incremented after it is accessed.
    /// Sometimes written as `[hl+]`.
    Hli,
}

pub(super) enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    /// Direct 8-bit value, stored directly after instruction.
    D8,
    /// HL Incremented, the value in HL is incremented after it is accessed.
    /// Sometimes written as `[hl+]`.
    Hli,
}
