use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::opcode::types::InstructionResult;

fn carry_add_u8(l: u8, r: u8) -> (u8, bool, bool) {
    let (val, carry) = l.overflowing_add(r);
    let half_carry = (val & (1 << 3)) < (l & (1 << 3));

    (val, half_carry, carry)
}

// Z 0 H C
pub fn add_a_b(cpu: &mut Cpu) -> InstructionResult {
    let a = cpu.registers.a();
    let b = cpu.registers.b();

    let (result, half_carry, carry) = carry_add_u8(a, b);

    cpu.registers.set_a(result);

    cpu.registers.flag.set_zero(result == 0);
    cpu.registers.flag.set_sub(false);
    cpu.registers.flag.set_half_carry(half_carry);
    cpu.registers.flag.set_carry(carry);

    (Cycle(4), OpLength(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::cpu::Cpu;

    #[test]
    fn run_add_a_b_zero() {
        let reg_a = 0b00000000;
        let reg_b = 0b00000000;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_a(reg_a);
        actual_cpu.registers.set_b(reg_b);

        actual_cpu.registers.flag.set_zero(false);
        actual_cpu.registers.flag.set_sub(true);
        actual_cpu.registers.flag.set_carry(true);
        actual_cpu.registers.flag.set_half_carry(true);

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_a(reg_a + reg_b);

        expected_cpu.registers.flag.set_zero(true);
        expected_cpu.registers.flag.set_sub(false);
        expected_cpu.registers.flag.set_carry(false);
        expected_cpu.registers.flag.set_half_carry(false);

        add_a_b(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }

    fn run_add_a_b(with_carry: bool, with_half_carry: bool) {
        let (reg_a, reg_b) = match (with_carry, with_half_carry) {
            (false, false) => (0b00000001_u8, 0b00000001_u8),
            (true, false) => (0b10000001, 0b10000001),
            (false, true) => (0b00001001, 0b00001001),
            (true, true) => (0b10001001, 0b10001001),
        };

        let (result, _) = reg_a.overflowing_add(reg_b);

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_a(reg_a);
        actual_cpu.registers.set_b(reg_b);

        actual_cpu.registers.flag.set_zero(true);
        actual_cpu.registers.flag.set_sub(true);
        actual_cpu.registers.flag.set_carry(!with_carry);
        actual_cpu.registers.flag.set_half_carry(!with_half_carry);

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_a(result);

        expected_cpu.registers.flag.set_zero(false);
        expected_cpu.registers.flag.set_sub(false);
        expected_cpu.registers.flag.set_carry(with_carry);
        expected_cpu.registers.flag.set_half_carry(with_half_carry);

        add_a_b(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }

    #[test]
    fn run_add_a_b_without_carry_without_half_carry() {
        run_add_a_b(false, false);
    }

    #[test]
    fn run_add_a_b_with_carry_with_half_carry() {
        run_add_a_b(true, true);
    }

    #[test]
    fn run_add_a_b_with_carry_without_half_carry() {
        run_add_a_b(true, false);
    }

    #[test]
    fn run_add_a_b_without_carry_with_half_carry() {
        run_add_a_b(false, true);
    }
}
