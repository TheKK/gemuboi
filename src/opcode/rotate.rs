use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::opcode::types::InstructionResult;

pub fn rlca(cpu: &mut Cpu) -> InstructionResult {
    let (new_a, carry) = rlc(cpu.registers.a());

    cpu.registers.set_a(new_a);

    let flag = &mut cpu.registers.flag;
    flag.set_zero(new_a == 0);
    flag.set_sub(false);
    flag.set_half_carry(false);
    flag.set_carry(carry);

    (Cycle(4), OpLength(1))
}

#[inline]
fn rlc(input: u8) -> (u8, bool) {
    let carry = 0b1000_0000 & input != 0;

    (input.rotate_left(1), carry)
}

#[cfg(test)]
mod test {
    pub use super::*;

    #[test]
    fn run_rlc() {
        assert_eq!(rlc(0b01010101), (0b10101010, false));
        assert_eq!(rlc(0b11111110), (0b11111101, true));
    }
}
