//! Contains all parameter types for the CPU instructions.

/// Which 8-bit register an instruction should affect. Note that F is missing.
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
