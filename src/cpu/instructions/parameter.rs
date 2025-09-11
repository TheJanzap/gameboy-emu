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
