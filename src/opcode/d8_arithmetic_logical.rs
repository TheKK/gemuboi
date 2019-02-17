use crate::carry_test::{CarryTest, CarryTestResult};
use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::opcode::types::InstructionResult;

macro_rules! add_a_instruction {
    ($ins_name: ident, $from: ident) => {
        // Z 0 H C
        pub fn $ins_name(cpu: &mut Cpu) -> InstructionResult {
            let a = cpu.registers.a();
            let val = cpu.registers.$from();

            let CarryTestResult {
                val: result,
                half_carry,
                carry,
            } = a.carry_add(val);

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

// SUB REG
// 1  4
// Z 1 H C
macro_rules! sub_reg_instruction {
    ($ins_name: ident, $reg_getter: ident) => {
        pub fn $ins_name(cpu: &mut Cpu) -> InstructionResult {
            let a = cpu.registers.a();
            let reg = cpu.registers.$reg_getter();

            let CarryTestResult {
                val: result,
                half_carry,
                carry,
            } = a.carry_sub(reg);

            cpu.registers.set_a(result);
            cpu.registers.flag.set_zero(result == 0);
            cpu.registers.flag.set_sub(true);
            cpu.registers.flag.set_half_carry(half_carry);
            cpu.registers.flag.set_carry(carry);

            (Cycle(4), OpLength(1))
        }
    };
}

sub_reg_instruction!(sub_b, b);

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

    macro_rules! test_sub_reg_instruction {
        ($inst_name: ident, $reg_setter: ident) => {
            mod $inst_name {
                use super::super::*;

                use crate::cpu::Cpu;

                fn run_with_non_zero_result_or_not(is_zero: bool) {
                    let init_a = 42;
                    let init_reg = if is_zero { 42 } else { 10 };

                    let expected_result = init_a - init_reg;

                    let mut actual_cpu = Cpu::default();
                    actual_cpu.registers.set_a(init_a);
                    actual_cpu.registers.$reg_setter(init_reg);
                    actual_cpu.registers.flag.set_zero(!is_zero);
                    actual_cpu.registers.flag.set_sub(false);

                    let mut expected_cpu = actual_cpu.clone();
                    expected_cpu.registers.set_a(expected_result);
                    expected_cpu.registers.flag.set_zero(is_zero);
                    expected_cpu.registers.flag.set_sub(true);

                    $inst_name(&mut actual_cpu);

                    assert_eq!(actual_cpu, expected_cpu);
                }

                #[test]
                fn run_with_zero_result() {
                    run_with_non_zero_result_or_not(true);
                }

                #[test]
                fn run_with_non_zero_result() {
                    run_with_non_zero_result_or_not(false);
                }

                fn run_with_carry_or_not(with_carry: bool) {
                    let init_a = 0b00010000_u8;
                    let init_reg = if with_carry { 0b11110000 } else { 0b00000000 };

                    let (expected_result, _) = init_a.overflowing_sub(init_reg);
                    let expected_zero = false;
                    let expected_half_carry = false;

                    let mut actual_cpu = Cpu::default();
                    actual_cpu.registers.set_a(init_a);
                    actual_cpu.registers.$reg_setter(init_reg);
                    actual_cpu.registers.flag.set_zero(!expected_zero);
                    actual_cpu.registers.flag.set_sub(false);
                    actual_cpu
                        .registers
                        .flag
                        .set_half_carry(!expected_half_carry);
                    actual_cpu.registers.flag.set_carry(!with_carry);

                    let mut expected_cpu = actual_cpu.clone();
                    expected_cpu.registers.set_a(expected_result);
                    expected_cpu.registers.flag.set_zero(expected_zero);
                    expected_cpu.registers.flag.set_sub(true);
                    expected_cpu
                        .registers
                        .flag
                        .set_half_carry(expected_half_carry);
                    expected_cpu.registers.flag.set_carry(with_carry);

                    $inst_name(&mut actual_cpu);

                    assert_eq!(actual_cpu, expected_cpu);
                }

                #[test]
                fn run_with_carry() {
                    run_with_carry_or_not(true);
                }

                #[test]
                fn run_without_carry() {
                    run_with_carry_or_not(false);
                }

                fn run_with_half_carry_or_not(with_half_carry: bool) {
                    let init_a = 0b11110001_u8;
                    let init_reg = if with_half_carry {
                        0b00001111
                    } else {
                        0b00000000
                    };

                    let (expected_result, _) = init_a.overflowing_sub(init_reg);
                    let expected_zero = false;

                    let expected_carry = false;

                    let mut actual_cpu = Cpu::default();
                    actual_cpu.registers.set_a(init_a);
                    actual_cpu.registers.$reg_setter(init_reg);
                    actual_cpu.registers.flag.set_zero(!expected_zero);
                    actual_cpu.registers.flag.set_sub(false);
                    actual_cpu.registers.flag.set_carry(!expected_carry);
                    actual_cpu.registers.flag.set_half_carry(!with_half_carry);

                    let mut expected_cpu = actual_cpu.clone();
                    expected_cpu.registers.set_a(expected_result);
                    expected_cpu.registers.flag.set_zero(expected_zero);
                    expected_cpu.registers.flag.set_sub(true);
                    expected_cpu.registers.flag.set_carry(expected_carry);
                    expected_cpu.registers.flag.set_half_carry(with_half_carry);

                    sub_b(&mut actual_cpu);

                    assert_eq!(actual_cpu, expected_cpu);
                }

                #[test]
                fn run_with_half_carry() {
                    run_with_half_carry_or_not(true);
                }

                #[test]
                fn run_without_half_carry() {
                    run_with_half_carry_or_not(false);
                }
            }
        };
    }

    test_sub_reg_instruction!(sub_b, set_b);
}
