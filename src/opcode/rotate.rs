use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::opcode::types::InstructionResult;
use crate::registers::Flag;

pub fn rlca(cpu: &mut Cpu) -> InstructionResult {
    let (new_a, new_flag) = rlc(cpu.registers.a());

    cpu.registers.set_a(new_a);
    cpu.registers.flag = new_flag;

    (Cycle(4), OpLength(1))
}

pub fn rla(cpu: &mut Cpu) -> InstructionResult {
    let (new_a, new_flags) = rl(cpu.registers.flag.carry(), cpu.registers.a());

    cpu.registers.set_a(new_a);
    cpu.registers.flag = new_flags;

    (Cycle(4), OpLength(1))
}

#[inline]
fn rlc(input: u8) -> (u8, Flag) {
    let carry = 0b1000_0000 & input != 0;
    let new_value = input.rotate_left(1);

    (new_value, Flag::new(new_value == 0, false, false, carry))
}

#[inline]
fn rl(carry: bool, input: u8) -> (u8, Flag) {
    let carry_bit = if carry { 1 } else { 0 };

    let new_carry = 0b1000_0000 & input != 0;
    let result = (input.rotate_left(1) & 0b1111_1110) + carry_bit;

    let mut new_flags = Flag::default();
    new_flags.set_zero(result == 0);
    new_flags.set_carry(new_carry);

    (result, new_flags)
}

fn rrc() {}

fn rr() {}

#[cfg(test)]
mod test {
    use crate::registers::Flag;

    pub use super::*;

    #[test]
    fn run_rlc() {
        assert_eq!(
            rlc(0b00000000),
            (0b00000000, Flag::new(true, false, false, false))
        );
        assert_eq!(
            rlc(0b10000000),
            (0b00000001, Flag::new(false, false, false, true))
        );
        assert_eq!(
            rlc(0b00000001),
            (0b00000010, Flag::new(false, false, false, false))
        );
    }

    #[test]
    fn run_rl() {
        assert_eq!(
            rl(true, 0b00000000),
            (0b00000001, Flag::new(false, false, false, false))
        );
        assert_eq!(
            rl(false, 0b00000000),
            (0b00000000, Flag::new(true, false, false, false))
        );
        assert_eq!(
            rl(true, 0b10000000),
            (0b00000001, Flag::new(false, false, false, true))
        );
        assert_eq!(
            rl(false, 0b10000000),
            (0b00000000, Flag::new(true, false, false, true))
        );
    }
}
