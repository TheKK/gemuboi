use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::opcode::types::InstructionResult;

fn carry_add_u8(l: u8, r: u8) -> (u8, bool, bool) {
    let (val, carry) = l.overflowing_add(r);
    let half_carry = (val & (1 << 3)) < (l & (1 << 3));

    (val, half_carry, carry)
}

macro_rules! add_a_instruction {
    ($ins_name: ident, $from: ident) => {
        // Z 0 H C
        pub fn $ins_name(cpu: &mut Cpu) -> InstructionResult {
            let a = cpu.registers.a();
            let val = cpu.registers.$from();

            let (result, half_carry, carry) = carry_add_u8(a, val);

            cpu.registers.set_a(result);

            cpu.registers.flag.set_zero(result == 0);
            cpu.registers.flag.set_sub(false);
            cpu.registers.flag.set_half_carry(half_carry);
            cpu.registers.flag.set_carry(carry);

            (Cycle(4), OpLength(1))
        }
    };
}

add_a_instruction!(add_a_a, a);
add_a_instruction!(add_a_b, b);
add_a_instruction!(add_a_c, c);
add_a_instruction!(add_a_d, d);
add_a_instruction!(add_a_e, e);
add_a_instruction!(add_a_h, h);
add_a_instruction!(add_a_l, l);

#[cfg(test)]
mod tests {
    macro_rules! test_add_a_instruction {
        ($inst_name: ident, $reg_setter: ident) => {
            mod $inst_name {
                use super::super::*;

                use crate::cpu::Cpu;

                fn run_test(with_carry: bool, with_half_carry: bool) {
                    let (reg_a, reg_other) = match (with_carry, with_half_carry) {
                        (false, false) => (0b00000001_u8, 0b00000001_u8),
                        (true, false) => (0b10000001, 0b10000001),
                        (false, true) => (0b00001001, 0b00001001),
                        (true, true) => (0b10001001, 0b10001001),
                    };

                    let (result, _) = reg_a.overflowing_add(reg_other);

                    let mut actual_cpu = Cpu::default();
                    actual_cpu.registers.set_a(reg_a);
                    actual_cpu.registers.$reg_setter(reg_other);

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

                    $inst_name(&mut actual_cpu);

                    assert_eq!(actual_cpu, expected_cpu);
                }

                #[test]
                fn run_test_zero() {
                    let reg_a = 0b00000000;
                    let reg_other = 0b00000000;

                    let mut actual_cpu = Cpu::default();
                    actual_cpu.registers.set_a(reg_a);
                    actual_cpu.registers.$reg_setter(reg_other);

                    actual_cpu.registers.flag.set_zero(false);
                    actual_cpu.registers.flag.set_sub(true);
                    actual_cpu.registers.flag.set_carry(true);
                    actual_cpu.registers.flag.set_half_carry(true);

                    let mut expected_cpu = actual_cpu.clone();
                    expected_cpu.registers.set_a(reg_a + reg_other);

                    expected_cpu.registers.flag.set_zero(true);
                    expected_cpu.registers.flag.set_sub(false);
                    expected_cpu.registers.flag.set_carry(false);
                    expected_cpu.registers.flag.set_half_carry(false);

                    $inst_name(&mut actual_cpu);

                    assert_eq!(actual_cpu, expected_cpu);
                }

                #[test]
                fn run_test_without_carry_without_half_carry() {
                    run_test(false, false);
                }

                #[test]
                fn run_test_with_carry_with_half_carry() {
                    run_test(true, true);
                }

                #[test]
                fn run_test_with_carry_without_half_carry() {
                    run_test(true, false);
                }

                #[test]
                fn run_test_without_carry_with_half_carry() {
                    run_test(false, true);
                }
            }
        };
    }

    test_add_a_instruction!(add_a_a, set_a);
    test_add_a_instruction!(add_a_b, set_b);
    test_add_a_instruction!(add_a_c, set_c);
    test_add_a_instruction!(add_a_d, set_d);
    test_add_a_instruction!(add_a_e, set_e);
    test_add_a_instruction!(add_a_h, set_h);
    test_add_a_instruction!(add_a_l, set_l);
}
