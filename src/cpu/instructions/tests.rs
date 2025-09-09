use super::*;
#[test]
fn add_no_overflow() {
    let mut cpu = Cpu::default();
    let a_input = 12;
    let to_add = 32;
    let result = 44;

    cpu.registers.a = a_input;
    cpu.registers.d = to_add;
    let instruction = Instruction::Add(TargetRegister8::D);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, result);
    assert_eq!(cpu.registers.f.zero, false);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, false);
    assert_eq!(cpu.registers.f.carry, false);
}

#[test]
fn add_overflow() {
    let mut cpu = Cpu::default();
    let a_input = 0b1111_1111;
    let to_add = 2;
    let result = 0b0000_0001;

    cpu.registers.a = a_input;
    cpu.registers.c = to_add;
    let instruction = Instruction::Add(TargetRegister8::C);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, result);
    assert_eq!(cpu.registers.f.zero, false);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, true);
    assert_eq!(cpu.registers.f.carry, true);
}

#[test]
fn add_half_carry() {
    let mut cpu = Cpu::default();
    let a_input = 0b1000_1111;
    let to_add = 1;
    let result = 0b1001_0000;

    cpu.registers.a = a_input;
    cpu.registers.e = to_add;
    let instruction = Instruction::Add(TargetRegister8::E);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, result);
    assert_eq!(cpu.registers.f.zero, false);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, true);
    assert_eq!(cpu.registers.f.carry, false);
}

#[test]
fn add_hl_no_overflow() {
    let mut cpu = Cpu::default();
    let hl_input = 0x5511;
    let to_add = 0x1111;
    let result = 0x6622;

    cpu.registers.set_hl(hl_input);
    cpu.registers.set_de(to_add);
    let instruction = Instruction::AddHl(TargetRegister16::DE);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get_hl(), result);
    assert_eq!(cpu.registers.f.zero, false);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, false);
    assert_eq!(cpu.registers.f.carry, false);
}

#[test]
fn add_hl_overflow() {
    let mut cpu = Cpu::default();
    let hl_input = 0xFFFF;
    let to_add = 2;
    let result = 0x0001;

    cpu.registers.set_hl(hl_input);
    cpu.registers.set_bc(to_add);
    let instruction = Instruction::AddHl(TargetRegister16::BC);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get_hl(), result);
    assert_eq!(cpu.registers.f.zero, false);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, true);
    assert_eq!(cpu.registers.f.carry, true);
}

#[test]
fn add_hl_half_carry() {
    let mut cpu = Cpu::default();
    let hl_input = 0b0000_1111_1111_1111;
    let to_add = 1;
    let result = 0b0001_0000_0000_0000;

    cpu.registers.set_hl(hl_input);
    cpu.registers.set_de(to_add);
    let instruction = Instruction::AddHl(TargetRegister16::DE);
    cpu.execute(instruction);
    assert_eq!(cpu.registers.get_hl(), result);
    assert_eq!(cpu.registers.f.zero, false);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, true);
    assert_eq!(cpu.registers.f.carry, false);
}

#[test]
fn adc_no_carry() {
    let mut cpu = Cpu::default();
    let a_input = 12;
    let to_add = 32;
    let result = 44;
    let old_carry = false;
    let new_carry = false;

    cpu.registers.a = a_input;
    cpu.registers.l = to_add;
    cpu.registers.f.carry = old_carry;
    let instruction = Instruction::Adc(TargetRegister8::L);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, result);
    assert_eq!(cpu.registers.f.carry, new_carry);
}

#[test]
fn adc_with_carry() {
    let mut cpu = Cpu::default();
    let a_input = 12;
    let old_carry = true;
    let to_add = 32;
    let result = 45;
    let new_carry = false;

    cpu.registers.a = a_input;
    cpu.registers.l = to_add;
    cpu.registers.f.carry = old_carry;
    let instruction = Instruction::Adc(TargetRegister8::L);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, result);
    assert_eq!(cpu.registers.f.carry, new_carry);
}

#[test]
fn sub_no_overflow() {
    let mut cpu = Cpu::default();
    let a_input = 44;
    let to_sub = 12;
    let result = 32;

    cpu.registers.a = a_input;
    cpu.registers.d = to_sub;
    let instruction = Instruction::Sub(TargetRegister8::D);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, result);
    assert_eq!(cpu.registers.f.zero, false);
    assert_eq!(cpu.registers.f.subtract, true);
    assert_eq!(cpu.registers.f.half_carry, false);
    assert_eq!(cpu.registers.f.carry, false);
}

#[test]
fn sub_underflow() {
    let mut cpu = Cpu::default();
    let a_input = 0b0000_0001;
    let to_sub = 2;
    let result = 0b1111_1111;

    cpu.registers.a = a_input;
    cpu.registers.c = to_sub;
    let instruction = Instruction::Sub(TargetRegister8::C);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, result);
    assert_eq!(cpu.registers.f.zero, false);
    assert_eq!(cpu.registers.f.subtract, true);
    assert_eq!(cpu.registers.f.half_carry, true);
    assert_eq!(cpu.registers.f.carry, true);
}

#[test]
fn sub_half_carry() {
    let mut cpu = Cpu::default();
    let a_input = 0b1001_0000;
    let to_sub = 1;
    let result = 0b1000_1111;

    cpu.registers.a = a_input;
    cpu.registers.e = to_sub;
    let instruction = Instruction::Sub(TargetRegister8::E);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, result);
    assert_eq!(cpu.registers.f.zero, false);
    assert_eq!(cpu.registers.f.subtract, true);
    assert_eq!(cpu.registers.f.half_carry, true);
    assert_eq!(cpu.registers.f.carry, false);
}

#[test]
fn sbc_no_carry() {
    let mut cpu = Cpu::default();
    let a_input = 44;
    let old_carry = false;
    let to_add = 32;
    let result = 12;
    let new_carry = false;

    cpu.registers.a = a_input;
    cpu.registers.l = to_add;
    cpu.registers.f.carry = old_carry;
    let instruction = Instruction::Sbc(TargetRegister8::L);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, result);
    assert_eq!(cpu.registers.f.carry, new_carry);
}

#[test]
fn sbc_with_carry() {
    let mut cpu = Cpu::default();
    let a_input = 44;
    let old_carry = true;
    let to_sub = 32;
    let result = 11;
    let new_carry = false;

    cpu.registers.a = a_input;
    cpu.registers.l = to_sub;
    cpu.registers.f.carry = old_carry;
    let instruction = Instruction::Sbc(TargetRegister8::L);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, result);
    assert_eq!(cpu.registers.f.carry, new_carry);
}

#[test]
fn cp() {
    let mut cpu = Cpu::default();
    let a_input = 44;
    let to_sub = 12;
    let _result = 32; // Must never appear

    cpu.registers.a = a_input;
    cpu.registers.d = to_sub;
    let instruction = Instruction::Cp(TargetRegister8::D);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, a_input);
    assert_eq!(cpu.registers.f.zero, false);
    assert_eq!(cpu.registers.f.subtract, true);
    assert_eq!(cpu.registers.f.half_carry, false);
    assert_eq!(cpu.registers.f.carry, false);
}

#[test]
fn and() {
    let mut cpu = Cpu::default();
    let a_input = 0b1100_1111;
    let to_and = 0b1010_1000;
    let result = 0b1000_1000;

    cpu.registers.a = a_input;
    cpu.registers.c = to_and;
    let instruction = Instruction::And(TargetRegister8::C);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, result);
}

#[test]
fn or() {
    let mut cpu = Cpu::default();
    let a_input = 0b1100_1111;
    let to_or = 0b1010_1000;
    let result = 0b1110_1111;

    cpu.registers.a = a_input;
    cpu.registers.l = to_or;
    let instruction = Instruction::Or(TargetRegister8::L);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, result);
}

#[test]
fn xor() {
    let mut cpu = Cpu::default();
    let a_input = 0b1100_1111;
    let to_or = 0b1010_1000;
    let result = 0b0110_0111;

    cpu.registers.a = a_input;
    cpu.registers.e = to_or;
    let instruction = Instruction::Xor(TargetRegister8::E);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, result);
}

#[test]
fn inc() {
    let mut cpu = Cpu::default();
    let input = 0b1100_1111;
    let result = 0b1101_0000;

    cpu.registers.e = input;
    let instruction = Instruction::Inc(TargetRegister8::E);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.e, result);
    assert_eq!(cpu.registers.f.zero, false);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, true);
    assert_eq!(cpu.registers.f.carry, false);
}

#[test]
fn dec() {
    let mut cpu = Cpu::default();
    let input = 0b1101_0000;
    let result = 0b1100_1111;

    cpu.registers.e = input;
    let instruction = Instruction::Dec(TargetRegister8::E);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.e, result);
    assert_eq!(cpu.registers.f.zero, false);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, true);
    assert_eq!(cpu.registers.f.carry, false);
}

#[test]
fn cff() {
    let mut cpu = Cpu::default();
    let instruction = Instruction::Ccf;
    cpu.registers.f.carry = true;
    cpu.execute(instruction);
    assert_eq!(cpu.registers.f.carry, false);
}

#[test]
fn scf() {
    let mut cpu = Cpu::default();
    let instruction = Instruction::Scf;
    cpu.registers.f.carry = false;
    cpu.execute(instruction);
    assert_eq!(cpu.registers.f.carry, true);
}

#[test]
fn cpl() {
    let mut cpu = Cpu::default();
    let a_input = 0b1100_1111;
    let result = 0b0011_0000;

    cpu.registers.a = a_input;
    let instruction = Instruction::Cpl;
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, result);
    assert_eq!(cpu.registers.f.subtract, true);
    assert_eq!(cpu.registers.f.half_carry, true);
}

#[test]
fn bit_set() {
    let mut cpu = Cpu::default();
    let input = 0b1101_0111;
    let index = U3::wrap(4);
    let result = true;

    cpu.registers.h = input;
    let instruction = Instruction::Bit(index, TargetRegister8::H);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.h, input);
    assert_eq!(cpu.registers.f.zero, result);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, true);
}

#[test]
fn bit_unset() {
    let mut cpu = Cpu::default();
    let input = 0b1100_0111;
    let index = U3::wrap(4);
    let result = false;

    cpu.registers.c = input;
    let instruction = Instruction::Bit(index, TargetRegister8::C);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.f.zero, result);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, true);
}

#[test]
fn res() {
    let mut cpu = Cpu::default();
    let input = 0b1101_0111;
    let index = U3::wrap(4);
    let result = 0b1100_0111;

    cpu.registers.b = input;
    let instruction = Instruction::Res(index, TargetRegister8::B);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.b, result);
}

#[test]
fn set() {
    let mut cpu = Cpu::default();
    let input = 0b1101_0111;
    let index = U3::wrap(3);
    let result = 0b1101_1111;

    cpu.registers.e = input;
    let instruction = Instruction::Set(index, TargetRegister8::E);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.e, result);
}

#[test]
fn rr() {
    let mut cpu = Cpu::default();
    let input = 0b1101_0111;
    let old_carry = true;
    let result = 0b1110_1011;
    let new_carry = true;

    cpu.registers.d = input;
    cpu.registers.f.carry = old_carry;
    let instruction = Instruction::Rr(TargetRegister8::D);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.d, result);
    assert_eq!(cpu.registers.f.zero, result == 0);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, false);
    assert_eq!(cpu.registers.f.carry, new_carry);
}

#[test]
fn rl() {
    let mut cpu = Cpu::default();
    let input = 0b1101_0111;
    let old_carry = false;
    let result = 0b1010_1110;
    let new_carry = true;

    cpu.registers.d = input;
    cpu.registers.f.carry = old_carry;
    let instruction = Instruction::Rl(TargetRegister8::D);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.d, result);
    assert_eq!(cpu.registers.f.zero, result == 0);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, false);
    assert_eq!(cpu.registers.f.carry, new_carry);
}

#[test]
fn rrc() {
    let mut cpu = Cpu::default();
    let a_input = 0b1100_1111;
    let old_carry = false;
    let result = 0b1110_0111;
    let new_carry = true;

    cpu.registers.h = a_input;
    cpu.registers.f.carry = old_carry;
    let instruction = Instruction::Rrc(TargetRegister8::H);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.h, result);
    assert_eq!(cpu.registers.f.zero, false);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, false);
    assert_eq!(cpu.registers.f.carry, new_carry);
}

#[test]
fn rlc() {
    let mut cpu = Cpu::default();
    let a_input = 0b1100_1111;
    let old_carry = false;
    let result = 0b1001_1110;
    let new_carry = true;

    cpu.registers.l = a_input;
    cpu.registers.f.carry = old_carry;
    let instruction = Instruction::Rlc(TargetRegister8::L);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.l, result);
    assert_eq!(cpu.registers.f.zero, false);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, false);
    assert_eq!(cpu.registers.f.carry, new_carry);
}
#[test]
fn rra() {
    let mut cpu = Cpu::default();
    let a_input = 0b1100_1111;
    let old_carry = false;
    let result = 0b0110_0111;
    let new_carry = true;

    cpu.registers.a = a_input;
    cpu.registers.f.carry = old_carry;
    let instruction = Instruction::Rra;
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, result);
    assert_eq!(cpu.registers.f.zero, false);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, false);
    assert_eq!(cpu.registers.f.carry, new_carry);
}

#[test]
fn rla() {
    let mut cpu = Cpu::default();
    let a_input = 0b1100_1011;
    let old_carry = true;
    let result = 0b1001_0111;
    let new_carry = true;

    cpu.registers.a = a_input;
    cpu.registers.f.carry = old_carry;
    let instruction = Instruction::Rla;
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, result);
    assert_eq!(cpu.registers.f.zero, false);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, false);
    assert_eq!(cpu.registers.f.carry, new_carry);
}

#[test]
fn rrca() {
    let mut cpu = Cpu::default();
    let a_input = 0b1100_1111;
    let old_carry = false;
    let result = 0b1110_0111;
    let new_carry = true;

    cpu.registers.a = a_input;
    cpu.registers.f.carry = old_carry;
    let instruction = Instruction::Rrca;
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, result);
    assert_eq!(cpu.registers.f.zero, false);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, false);
    assert_eq!(cpu.registers.f.carry, new_carry);
}

#[test]
fn rlca() {
    let mut cpu = Cpu::default();
    let a_input = 0b1100_1111;
    let old_carry = false;
    let result = 0b1001_1110;
    let new_carry = true;

    cpu.registers.a = a_input;
    cpu.registers.f.carry = old_carry;
    let instruction = Instruction::Rlca;
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, result);
    assert_eq!(cpu.registers.f.zero, false);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, false);
    assert_eq!(cpu.registers.f.carry, new_carry);
}

#[test]
fn srl() {
    let mut cpu = Cpu::default();
    let input = 0b1101_0111;
    let result = 0b0110_1011;
    let new_carry = true;

    cpu.registers.a = input;
    let instruction = Instruction::Srl(TargetRegister8::A);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.a, result);
    assert_eq!(cpu.registers.f.zero, result == 0);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, false);
    assert_eq!(cpu.registers.f.carry, new_carry);
}

#[test]
fn sra() {
    let mut cpu = Cpu::default();
    let input = 0b1100_1111;
    let result = 0b0110_0111;
    let new_carry = true;

    cpu.registers.h = input;
    let instruction = Instruction::Sra(TargetRegister8::H);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.h, result);
    assert_eq!(cpu.registers.f.carry, new_carry);
}

#[test]
fn sla() {
    let mut cpu = Cpu::default();
    let input = 0b1100_1111;
    let result = 0b1001_1110;
    let new_carry = true;

    cpu.registers.h = input;
    let instruction = Instruction::Sla(TargetRegister8::H);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.h, result);
    assert_eq!(cpu.registers.f.zero, result == 0);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, false);
    assert_eq!(cpu.registers.f.carry, new_carry);
}

#[test]
fn swap() {
    let mut cpu = Cpu::default();
    let input = 0b1100_1111;
    let result = 0b1111_1100;

    cpu.registers.c = input;
    let instruction = Instruction::Swap(TargetRegister8::C);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.c, result);
    assert_eq!(cpu.registers.f.zero, false);
    assert_eq!(cpu.registers.f.subtract, false);
    assert_eq!(cpu.registers.f.half_carry, false);
    assert_eq!(cpu.registers.f.carry, false);
}
