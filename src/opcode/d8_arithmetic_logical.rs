use crate::carry_test::{CarryTest, CarryTestResult};
use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::opcode::types::InstructionResult;

// Z 0 H C
fn add(cpu: &mut Cpu, val: u8) {
    let a = cpu.registers.a();

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
}

// Z 0 H C
fn adc(cpu: &mut Cpu, val: u8) {
    let a = cpu.registers.a();
    let cy = if cpu.registers.flag.carry() { 1 } else { 0 };

    let CarryTestResult {
        val,
        half_carry: half_carry_a,
        carry: carry_a,
    } = a.carry_add(val);

    let CarryTestResult {
        val: result,
        half_carry: half_carry_b,
        carry: carry_b,
    } = val.carry_add(cy);

    // Since (a + b + 1) could only carry once, we use 'or' here.
    let half_carry = half_carry_a || half_carry_b;
    let carry = carry_a || carry_b;

    cpu.registers.set_a(result);

    cpu.registers.flag.set_zero(result == 0);
    cpu.registers.flag.set_sub(false);
    cpu.registers.flag.set_half_carry(half_carry);
    cpu.registers.flag.set_carry(carry);
}

// Z 1 H C
fn sub(cpu: &mut Cpu, val: u8) {
    let a = cpu.registers.a();

    let CarryTestResult {
        val: result,
        half_carry,
        carry,
    } = a.carry_sub(val);

    cpu.registers.set_a(result);
    cpu.registers.flag.set_zero(result == 0);
    cpu.registers.flag.set_sub(true);
    cpu.registers.flag.set_half_carry(half_carry);
    cpu.registers.flag.set_carry(carry);
}

// Z 1 H C
fn sbc(cpu: &mut Cpu, val: u8) {
    let a = cpu.registers.a();
    let cy = if cpu.registers.flag.carry() { 1 } else { 0 };

    let CarryTestResult {
        val,
        half_carry: half_carry_a,
        carry: carry_a,
    } = a.carry_sub(val);

    let CarryTestResult {
        val: result,
        half_carry: half_carry_b,
        carry: carry_b,
    } = val.carry_sub(cy);

    // Since (a - b - 1) could only carry once, we use 'or' here.
    let half_carry = half_carry_a || half_carry_b;
    let carry = carry_a || carry_b;

    cpu.registers.set_a(result);

    cpu.registers.flag.set_zero(result == 0);
    cpu.registers.flag.set_sub(true);
    cpu.registers.flag.set_half_carry(half_carry);
    cpu.registers.flag.set_carry(carry);
}

// Z 0 1 0
fn and(cpu: &mut Cpu, val: u8) {
    let a = cpu.registers.a();

    let result = a & val;

    cpu.registers.set_a(result);

    cpu.registers.flag.set_zero(result == 0);
    cpu.registers.flag.set_sub(false);
    cpu.registers.flag.set_half_carry(true);
    cpu.registers.flag.set_carry(false);
}

// Z 0 0 0
fn xor(cpu: &mut Cpu, val: u8) {
    let a = cpu.registers.a();

    let result = a ^ val;

    cpu.registers.set_a(result);

    cpu.registers.flag.set_zero(result == 0);
    cpu.registers.flag.set_sub(false);
    cpu.registers.flag.set_half_carry(false);
    cpu.registers.flag.set_carry(false);
}

// Z 1 H C
fn cp(cpu: &mut Cpu, val: u8) {
    let a = cpu.registers.a();

    let CarryTestResult {
        val: result,
        half_carry,
        carry,
    } = a.carry_sub(val);

    cpu.registers.flag.set_zero(result == 0);
    cpu.registers.flag.set_sub(true);
    cpu.registers.flag.set_half_carry(half_carry);
    cpu.registers.flag.set_carry(carry);
}

// Z 0 0 0
fn or(cpu: &mut Cpu, val: u8) {
    let a = cpu.registers.a();

    let result = a | val;

    cpu.registers.set_a(result);

    cpu.registers.flag.set_zero(result == 0);
    cpu.registers.flag.set_sub(false);
    cpu.registers.flag.set_half_carry(false);
    cpu.registers.flag.set_carry(false);
}

macro_rules! add_a_instruction {
    ($ins_name: ident, $from: ident) => {
        // Z 0 H C
        pub fn $ins_name(cpu: &mut Cpu) -> InstructionResult {
            add(cpu, cpu.registers.$from());

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

// ADD A,(HL)
// 1  8
pub fn add_a_hl_dref(cpu: &mut Cpu) -> InstructionResult {
    add(cpu, cpu.read_hl_dref());

    (Cycle(8), OpLength(1))
}

// ADD A,d8
// 2  8
pub fn add_a_d8(cpu: &mut Cpu) -> InstructionResult {
    add(cpu, cpu.read_byte_argument(1));

    (Cycle(8), OpLength(2))
}

// ADC A REG
// 1  4
macro_rules! adc_a_instruction {
    ($ins_name: ident, $from: ident) => {
        pub fn $ins_name(cpu: &mut Cpu) -> InstructionResult {
            adc(cpu, cpu.registers.$from());

            (Cycle(4), OpLength(1))
        }
    };
}

adc_a_instruction!(adc_a_b, b);
adc_a_instruction!(adc_a_c, c);
adc_a_instruction!(adc_a_d, d);
adc_a_instruction!(adc_a_e, e);
adc_a_instruction!(adc_a_h, h);
adc_a_instruction!(adc_a_l, l);
adc_a_instruction!(adc_a_a, a);

// ADC A,(HL)
// 1  8
pub fn adc_a_hl_dref(cpu: &mut Cpu) -> InstructionResult {
    adc(cpu, cpu.read_hl_dref());

    (Cycle(8), OpLength(1))
}

// ADC A,d8
// 2  8
pub fn adc_a_d8(cpu: &mut Cpu) -> InstructionResult {
    adc(cpu, cpu.read_byte_argument(1));

    (Cycle(8), OpLength(2))
}

// SUB REG
// 1  4
macro_rules! sub_reg_instruction {
    ($ins_name: ident, $from: ident) => {
        pub fn $ins_name(cpu: &mut Cpu) -> InstructionResult {
            sub(cpu, cpu.registers.$from());

            (Cycle(4), OpLength(1))
        }
    };
}

sub_reg_instruction!(sub_a_b, b);
sub_reg_instruction!(sub_a_c, c);
sub_reg_instruction!(sub_a_d, d);
sub_reg_instruction!(sub_a_e, e);
sub_reg_instruction!(sub_a_h, h);
sub_reg_instruction!(sub_a_l, l);
sub_reg_instruction!(sub_a_a, a);

// SUB (HL)
// 1  8
pub fn sub_hl_dref(cpu: &mut Cpu) -> InstructionResult {
    sub(cpu, cpu.read_hl_dref());

    (Cycle(8), OpLength(1))
}

// SUB d8
// 2  8
pub fn sub_d8(cpu: &mut Cpu) -> InstructionResult {
    sub(cpu, cpu.read_byte_argument(1));

    (Cycle(8), OpLength(2))
}

// SBC REG
// 1  4
macro_rules! sbc_reg_instruction {
    ($ins_name: ident, $from: ident) => {
        pub fn $ins_name(cpu: &mut Cpu) -> InstructionResult {
            sbc(cpu, cpu.registers.$from());

            (Cycle(4), OpLength(1))
        }
    };
}

sbc_reg_instruction!(sbc_a_b, b);
sbc_reg_instruction!(sbc_a_c, c);
sbc_reg_instruction!(sbc_a_d, d);
sbc_reg_instruction!(sbc_a_e, e);
sbc_reg_instruction!(sbc_a_h, h);
sbc_reg_instruction!(sbc_a_l, l);
sbc_reg_instruction!(sbc_a_a, a);

// SBC A,(HL)
// 1  8
pub fn sbc_a_hl_dref(cpu: &mut Cpu) -> InstructionResult {
    sbc(cpu, cpu.read_hl_dref());

    (Cycle(8), OpLength(1))
}

// SBC A,d8
// 2  8
pub fn sbc_a_d8(cpu: &mut Cpu) -> InstructionResult {
    sbc(cpu, cpu.read_byte_argument(1));

    (Cycle(8), OpLength(2))
}

// AND REG
// 1  4
macro_rules! and_reg_instruction {
    ($ins_name: ident, $from: ident) => {
        pub fn $ins_name(cpu: &mut Cpu) -> InstructionResult {
            and(cpu, cpu.registers.$from());

            (Cycle(4), OpLength(1))
        }
    };
}

and_reg_instruction!(and_b, b);
and_reg_instruction!(and_c, c);
and_reg_instruction!(and_d, d);
and_reg_instruction!(and_e, e);
and_reg_instruction!(and_h, h);
and_reg_instruction!(and_l, l);
and_reg_instruction!(and_a, a);

// AND (HL)
// 1  8
pub fn and_hl_dref(cpu: &mut Cpu) -> InstructionResult {
    and(cpu, cpu.read_hl_dref());

    (Cycle(8), OpLength(1))
}

// AND d8
// 2  8
pub fn and_d8(cpu: &mut Cpu) -> InstructionResult {
    and(cpu, cpu.read_byte_argument(1));

    (Cycle(8), OpLength(2))
}

// XOR REG
// 1  4
macro_rules! xor_reg_instruction {
    ($ins_name: ident, $from: ident) => {
        pub fn $ins_name(cpu: &mut Cpu) -> InstructionResult {
            xor(cpu, cpu.registers.$from());

            (Cycle(4), OpLength(1))
        }
    };
}

xor_reg_instruction!(xor_b, b);
xor_reg_instruction!(xor_c, c);
xor_reg_instruction!(xor_d, d);
xor_reg_instruction!(xor_e, e);
xor_reg_instruction!(xor_h, h);
xor_reg_instruction!(xor_l, l);
xor_reg_instruction!(xor_a, a);

// XOR (HL)
// 1  8
pub fn xor_hl_dref(cpu: &mut Cpu) -> InstructionResult {
    xor(cpu, cpu.read_hl_dref());

    (Cycle(8), OpLength(1))
}

// XOR d8
// 2  8
pub fn xor_d8(cpu: &mut Cpu) -> InstructionResult {
    xor(cpu, cpu.read_byte_argument(1));

    (Cycle(8), OpLength(2))
}

// OR REG
// 1  4
macro_rules! or_reg_instruction {
    ($ins_name: ident, $from: ident) => {
        pub fn $ins_name(cpu: &mut Cpu) -> InstructionResult {
            or(cpu, cpu.registers.$from());

            (Cycle(4), OpLength(1))
        }
    };
}

or_reg_instruction!(or_b, b);
or_reg_instruction!(or_c, c);
or_reg_instruction!(or_d, d);
or_reg_instruction!(or_e, e);
or_reg_instruction!(or_h, h);
or_reg_instruction!(or_l, l);
or_reg_instruction!(or_a, a);

// OR (HL)
// 1  8
pub fn or_hl_dref(cpu: &mut Cpu) -> InstructionResult {
    or(cpu, cpu.read_hl_dref());

    (Cycle(8), OpLength(1))
}

// OR d8
// 2  8
pub fn or_d8(cpu: &mut Cpu) -> InstructionResult {
    or(cpu, cpu.read_byte_argument(1));

    (Cycle(8), OpLength(2))
}

#[cfg(test)]
mod tests {
    mod add_a {
        use super::super::add;

        use crate::cpu::Cpu;

        fn run_test(with_carry: bool, with_half_carry: bool) {
            let (reg_a, val) = match (with_carry, with_half_carry) {
                (false, false) => (0b00000001_u8, 0b00000001_u8),
                (true, false) => (0b10000001, 0b10000001),
                (false, true) => (0b00001001, 0b00001001),
                (true, true) => (0b10001001, 0b10001001),
            };

            let (result, _) = reg_a.overflowing_add(val);

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(reg_a);

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

            add(&mut actual_cpu, val);

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

        #[test]
        fn run_test_zero() {
            let reg_a = 0b00000000;
            let val = 0b00000000;

            let expected_reg_a = reg_a + val;

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(reg_a);

            actual_cpu.registers.flag.set_zero(false);
            actual_cpu.registers.flag.set_sub(true);
            actual_cpu.registers.flag.set_carry(true);
            actual_cpu.registers.flag.set_half_carry(true);

            let mut expected_cpu = actual_cpu.clone();
            expected_cpu.registers.set_a(expected_reg_a);

            expected_cpu.registers.flag.set_zero(true);
            expected_cpu.registers.flag.set_sub(false);
            expected_cpu.registers.flag.set_carry(false);
            expected_cpu.registers.flag.set_half_carry(false);

            add(&mut actual_cpu, val);

            assert_eq!(actual_cpu, expected_cpu);
        }
    }

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

    mod adc_a {
        use super::super::adc;

        use crate::cpu::Cpu;

        fn run_test(with_carry: bool, with_half_carry: bool) {
            let (reg_a, val) = match (with_carry, with_half_carry) {
                (false, false) => (0b00000001_u8, 0b00000001_u8),
                (true, false) => (0b10000001, 0b10000001),
                (false, true) => (0b00001001, 0b00001001),
                (true, true) => (0b10001001, 0b10001001),
            };

            let (result, _) = reg_a.overflowing_add(val);

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(reg_a);

            actual_cpu.registers.flag.set_zero(true);
            actual_cpu.registers.flag.set_sub(true);
            actual_cpu.registers.flag.set_carry(false);
            actual_cpu.registers.flag.set_half_carry(!with_half_carry);

            let mut expected_cpu = actual_cpu.clone();
            expected_cpu.registers.set_a(result);

            expected_cpu.registers.flag.set_zero(false);
            expected_cpu.registers.flag.set_sub(false);
            expected_cpu.registers.flag.set_carry(with_carry);
            expected_cpu.registers.flag.set_half_carry(with_half_carry);

            adc(&mut actual_cpu, val);

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

        #[test]
        fn run_test_zero() {
            let reg_a = 0b00000000;
            let val = 0b00000000;

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(reg_a);

            actual_cpu.registers.flag.set_zero(false);
            actual_cpu.registers.flag.set_sub(true);
            actual_cpu.registers.flag.set_carry(false);
            actual_cpu.registers.flag.set_half_carry(true);

            let mut expected_cpu = actual_cpu.clone();
            expected_cpu.registers.set_a(reg_a + val);

            expected_cpu.registers.flag.set_zero(true);
            expected_cpu.registers.flag.set_sub(false);
            expected_cpu.registers.flag.set_carry(false);
            expected_cpu.registers.flag.set_half_carry(false);

            adc(&mut actual_cpu, val);

            assert_eq!(actual_cpu, expected_cpu);
        }

        #[test]
        fn run_test_with_old_carry() {
            let reg_a = 0b00000000;
            let val = 0b00000000;

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(reg_a);

            actual_cpu.registers.flag.set_zero(true);
            actual_cpu.registers.flag.set_sub(true);
            actual_cpu.registers.flag.set_carry(true);
            actual_cpu.registers.flag.set_half_carry(true);

            let mut expected_cpu = actual_cpu.clone();
            expected_cpu.registers.set_a(1);

            expected_cpu.registers.flag.set_zero(false);
            expected_cpu.registers.flag.set_sub(false);
            expected_cpu.registers.flag.set_carry(false);
            expected_cpu.registers.flag.set_half_carry(false);

            adc(&mut actual_cpu, val);

            assert_eq!(actual_cpu, expected_cpu);
        }
    }

    macro_rules! test_adc_a_instruction {
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
                    actual_cpu.registers.flag.set_carry(false);
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
                    actual_cpu.registers.flag.set_carry(false);
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

                #[test]
                fn run_test_with_old_carry() {
                    let reg_a = 0b00000000;
                    let reg_other = 0b00000000;

                    let mut actual_cpu = Cpu::default();
                    actual_cpu.registers.set_a(reg_a);
                    actual_cpu.registers.$reg_setter(reg_other);

                    actual_cpu.registers.flag.set_zero(true);
                    actual_cpu.registers.flag.set_sub(true);
                    actual_cpu.registers.flag.set_carry(true);
                    actual_cpu.registers.flag.set_half_carry(true);

                    let mut expected_cpu = actual_cpu.clone();
                    expected_cpu.registers.set_a(1);

                    expected_cpu.registers.flag.set_zero(false);
                    expected_cpu.registers.flag.set_sub(false);
                    expected_cpu.registers.flag.set_carry(false);
                    expected_cpu.registers.flag.set_half_carry(false);

                    $inst_name(&mut actual_cpu);

                    assert_eq!(actual_cpu, expected_cpu);
                }
            }
        };
    }

    test_adc_a_instruction!(adc_a_a, set_a);
    test_adc_a_instruction!(adc_a_b, set_b);
    test_adc_a_instruction!(adc_a_c, set_c);
    test_adc_a_instruction!(adc_a_d, set_d);
    test_adc_a_instruction!(adc_a_e, set_e);
    test_adc_a_instruction!(adc_a_h, set_h);
    test_adc_a_instruction!(adc_a_l, set_l);

    mod sub_a {
        use super::super::sub;

        use crate::cpu::Cpu;

        fn run_with_non_zero_result_or_not(is_zero: bool) {
            let init_a = 42;
            let val = if is_zero { 42 } else { 10 };

            let expected_result = init_a - val;

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(init_a);
            actual_cpu.registers.flag.set_zero(!is_zero);
            actual_cpu.registers.flag.set_sub(false);

            let mut expected_cpu = actual_cpu.clone();
            expected_cpu.registers.set_a(expected_result);
            expected_cpu.registers.flag.set_zero(is_zero);
            expected_cpu.registers.flag.set_sub(true);

            sub(&mut actual_cpu, val);

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
            let val = if with_carry { 0b11110000 } else { 0b00000000 };

            let (expected_result, _) = init_a.overflowing_sub(val);
            let expected_zero = false;
            let expected_half_carry = false;

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(init_a);
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

            sub(&mut actual_cpu, val);

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
            let val = if with_half_carry {
                0b00001111
            } else {
                0b00000000
            };

            let (expected_result, _) = init_a.overflowing_sub(val);
            let expected_zero = false;

            let expected_carry = false;

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(init_a);
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

            sub(&mut actual_cpu, val);

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

                    $inst_name(&mut actual_cpu);

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

    test_sub_reg_instruction!(sub_a_b, set_b);
    test_sub_reg_instruction!(sub_a_c, set_c);
    test_sub_reg_instruction!(sub_a_d, set_d);
    test_sub_reg_instruction!(sub_a_e, set_e);
    test_sub_reg_instruction!(sub_a_h, set_h);
    test_sub_reg_instruction!(sub_a_l, set_l);

    mod sbc {
        use super::super::sbc;

        use crate::cpu::Cpu;

        fn run_with_old_carry_or_not(with_old_carry: bool) {
            let init_a = 0;
            let val = 0;

            let expected_result = if with_old_carry { 0xFF } else { 0 };
            let expected_half_carry = if with_old_carry { true } else { false };
            let expected_carry = if with_old_carry { true } else { false };
            let expected_zero = if with_old_carry { false } else { true };

            let is_sub = true;

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(init_a);
            actual_cpu
                .registers
                .flag
                .set_half_carry(!expected_half_carry);
            actual_cpu.registers.flag.set_carry(with_old_carry);
            actual_cpu.registers.flag.set_zero(!expected_zero);
            actual_cpu.registers.flag.set_sub(!is_sub);

            let mut expected_cpu = actual_cpu.clone();
            expected_cpu.registers.set_a(expected_result);
            expected_cpu
                .registers
                .flag
                .set_half_carry(expected_half_carry);
            expected_cpu.registers.flag.set_carry(expected_carry);
            expected_cpu.registers.flag.set_zero(expected_zero);
            expected_cpu.registers.flag.set_sub(is_sub);

            sbc(&mut actual_cpu, val);

            assert_eq!(actual_cpu, expected_cpu);
        }

        #[test]
        fn run_with_old_carry() {
            run_with_old_carry_or_not(true);
        }

        #[test]
        fn run_without_old_carry() {
            run_with_old_carry_or_not(false);
        }

        fn run_with_non_zero_result_or_not(is_zero: bool) {
            let init_a = 42;
            let val = if is_zero { 42 } else { 10 };

            let expected_result = init_a - val;

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(init_a);
            actual_cpu.registers.flag.set_zero(!is_zero);
            actual_cpu.registers.flag.set_sub(false);

            let mut expected_cpu = actual_cpu.clone();
            expected_cpu.registers.set_a(expected_result);
            expected_cpu.registers.flag.set_zero(is_zero);
            expected_cpu.registers.flag.set_sub(true);

            sbc(&mut actual_cpu, val);

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
            let val = if with_carry { 0b11110000 } else { 0b00000000 };

            let (expected_result, _) = init_a.overflowing_sub(val);
            let expected_zero = false;
            let expected_half_carry = false;

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(init_a);
            actual_cpu.registers.flag.set_zero(!expected_zero);
            actual_cpu.registers.flag.set_sub(false);
            actual_cpu
                .registers
                .flag
                .set_half_carry(!expected_half_carry);
            actual_cpu.registers.flag.set_carry(false);

            let mut expected_cpu = actual_cpu.clone();
            expected_cpu.registers.set_a(expected_result);
            expected_cpu.registers.flag.set_zero(expected_zero);
            expected_cpu.registers.flag.set_sub(true);
            expected_cpu
                .registers
                .flag
                .set_half_carry(expected_half_carry);
            expected_cpu.registers.flag.set_carry(with_carry);

            sbc(&mut actual_cpu, val);

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
            let val = if with_half_carry {
                0b00001111
            } else {
                0b00000000
            };

            let (expected_result, _) = init_a.overflowing_sub(val);
            let expected_zero = false;

            let expected_carry = false;

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(init_a);
            actual_cpu.registers.flag.set_zero(!expected_zero);
            actual_cpu.registers.flag.set_sub(false);
            actual_cpu.registers.flag.set_carry(false);
            actual_cpu.registers.flag.set_half_carry(!with_half_carry);

            let mut expected_cpu = actual_cpu.clone();
            expected_cpu.registers.set_a(expected_result);
            expected_cpu.registers.flag.set_zero(expected_zero);
            expected_cpu.registers.flag.set_sub(true);
            expected_cpu.registers.flag.set_carry(expected_carry);
            expected_cpu.registers.flag.set_half_carry(with_half_carry);

            sbc(&mut actual_cpu, val);

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

    mod and {
        use super::super::and;

        use crate::cpu::Cpu;

        #[test]
        fn normal_run() {
            let reg_a = 0b10000001;
            let val = 0b11111110;

            let expected_a = reg_a & val;

            let expected_zero = false;
            let expected_sub = false;
            let expected_half_carry = true;
            let expected_carry = false;

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(reg_a);
            actual_cpu.registers.flag.set_zero(!expected_zero);
            actual_cpu.registers.flag.set_sub(!expected_sub);
            actual_cpu
                .registers
                .flag
                .set_half_carry(!expected_half_carry);
            actual_cpu.registers.flag.set_carry(!expected_carry);

            let mut expected_cpu = actual_cpu.clone();
            expected_cpu.registers.set_a(expected_a);
            expected_cpu.registers.flag.set_zero(expected_zero);
            expected_cpu.registers.flag.set_sub(expected_sub);
            expected_cpu
                .registers
                .flag
                .set_half_carry(expected_half_carry);
            expected_cpu.registers.flag.set_carry(expected_carry);

            and(&mut actual_cpu, val);

            assert_eq!(actual_cpu, expected_cpu);
        }

        #[test]
        fn run_with_zero_result() {
            let reg_a = 0xF0;
            let val = 0x0F;

            let expected_a = reg_a & val;

            let expected_zero = true;
            let expected_sub = false;
            let expected_half_carry = true;
            let expected_carry = false;

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(reg_a);
            actual_cpu.registers.flag.set_zero(!expected_zero);
            actual_cpu.registers.flag.set_sub(!expected_sub);
            actual_cpu
                .registers
                .flag
                .set_half_carry(!expected_half_carry);
            actual_cpu.registers.flag.set_carry(!expected_carry);

            let mut expected_cpu = actual_cpu.clone();
            expected_cpu.registers.set_a(expected_a);
            expected_cpu.registers.flag.set_zero(expected_zero);
            expected_cpu.registers.flag.set_sub(expected_sub);
            expected_cpu
                .registers
                .flag
                .set_half_carry(expected_half_carry);
            expected_cpu.registers.flag.set_carry(expected_carry);

            and(&mut actual_cpu, val);

            assert_eq!(actual_cpu, expected_cpu);
        }
    }

    mod or {
        use super::super::or;

        use crate::cpu::Cpu;

        #[test]
        fn normal_run() {
            let reg_a = 0b10101010;
            let val = 0b01010101;

            let expected_a = reg_a | val;

            let expected_zero = false;
            let expected_sub = false;
            let expected_half_carry = false;
            let expected_carry = false;

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(reg_a);
            actual_cpu.registers.flag.set_zero(!expected_zero);
            actual_cpu.registers.flag.set_sub(!expected_sub);
            actual_cpu
                .registers
                .flag
                .set_half_carry(!expected_half_carry);
            actual_cpu.registers.flag.set_carry(!expected_carry);

            let mut expected_cpu = actual_cpu.clone();
            expected_cpu.registers.set_a(expected_a);
            expected_cpu.registers.flag.set_zero(expected_zero);
            expected_cpu.registers.flag.set_sub(expected_sub);
            expected_cpu
                .registers
                .flag
                .set_half_carry(expected_half_carry);
            expected_cpu.registers.flag.set_carry(expected_carry);

            or(&mut actual_cpu, val);

            assert_eq!(actual_cpu, expected_cpu);
        }

        #[test]
        fn run_with_zero_result() {
            let reg_a = 0;
            let val = 0;

            let expected_a = 0;

            let expected_zero = true;
            let expected_sub = false;
            let expected_half_carry = false;
            let expected_carry = false;

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(reg_a);
            actual_cpu.registers.flag.set_zero(!expected_zero);
            actual_cpu.registers.flag.set_sub(!expected_sub);
            actual_cpu
                .registers
                .flag
                .set_half_carry(!expected_half_carry);
            actual_cpu.registers.flag.set_carry(!expected_carry);

            let mut expected_cpu = actual_cpu.clone();
            expected_cpu.registers.set_a(expected_a);
            expected_cpu.registers.flag.set_zero(expected_zero);
            expected_cpu.registers.flag.set_sub(expected_sub);
            expected_cpu
                .registers
                .flag
                .set_half_carry(expected_half_carry);
            expected_cpu.registers.flag.set_carry(expected_carry);

            or(&mut actual_cpu, val);

            assert_eq!(actual_cpu, expected_cpu);
        }
    }

    mod xor {
        use super::super::xor;

        use crate::cpu::Cpu;

        #[test]
        fn normal_run() {
            let reg_a = 0b11011010;
            let val = 0b11110001;

            let expected_a = reg_a ^ val;

            let expected_zero = false;
            let expected_sub = false;
            let expected_half_carry = false;
            let expected_carry = false;

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(reg_a);
            actual_cpu.registers.flag.set_zero(!expected_zero);
            actual_cpu.registers.flag.set_sub(!expected_sub);
            actual_cpu
                .registers
                .flag
                .set_half_carry(!expected_half_carry);
            actual_cpu.registers.flag.set_carry(!expected_carry);

            let mut expected_cpu = actual_cpu.clone();
            expected_cpu.registers.set_a(expected_a);
            expected_cpu.registers.flag.set_zero(expected_zero);
            expected_cpu.registers.flag.set_sub(expected_sub);
            expected_cpu
                .registers
                .flag
                .set_half_carry(expected_half_carry);
            expected_cpu.registers.flag.set_carry(expected_carry);

            xor(&mut actual_cpu, val);

            assert_eq!(actual_cpu, expected_cpu);
        }

        #[test]
        fn run_with_zero_result() {
            let reg_a = 0b11110000;
            let val = 0b11110000;

            let expected_a = reg_a ^ val;

            let expected_zero = true;
            let expected_sub = false;
            let expected_half_carry = false;
            let expected_carry = false;

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(reg_a);
            actual_cpu.registers.flag.set_zero(!expected_zero);
            actual_cpu.registers.flag.set_sub(!expected_sub);
            actual_cpu
                .registers
                .flag
                .set_half_carry(!expected_half_carry);
            actual_cpu.registers.flag.set_carry(!expected_carry);

            let mut expected_cpu = actual_cpu.clone();
            expected_cpu.registers.set_a(expected_a);
            expected_cpu.registers.flag.set_zero(expected_zero);
            expected_cpu.registers.flag.set_sub(expected_sub);
            expected_cpu
                .registers
                .flag
                .set_half_carry(expected_half_carry);
            expected_cpu.registers.flag.set_carry(expected_carry);

            xor(&mut actual_cpu, val);

            assert_eq!(actual_cpu, expected_cpu);
        }
    }

    mod cp {
        use super::super::cp;

        use crate::cpu::Cpu;

        fn run_with_non_zero_result_or_not(is_zero: bool) {
            let init_a = 42;
            let val = if is_zero { 42 } else { 10 };

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(init_a);
            actual_cpu.registers.flag.set_zero(!is_zero);
            actual_cpu.registers.flag.set_sub(false);

            let mut expected_cpu = actual_cpu.clone();
            expected_cpu.registers.flag.set_zero(is_zero);
            expected_cpu.registers.flag.set_sub(true);

            cp(&mut actual_cpu, val);

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
            let val = if with_carry { 0b11110000 } else { 0b00000000 };

            let expected_zero = false;
            let expected_half_carry = false;

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(init_a);
            actual_cpu.registers.flag.set_zero(!expected_zero);
            actual_cpu.registers.flag.set_sub(false);
            actual_cpu
                .registers
                .flag
                .set_half_carry(!expected_half_carry);
            actual_cpu.registers.flag.set_carry(!with_carry);

            let mut expected_cpu = actual_cpu.clone();
            expected_cpu.registers.flag.set_zero(expected_zero);
            expected_cpu.registers.flag.set_sub(true);
            expected_cpu
                .registers
                .flag
                .set_half_carry(expected_half_carry);
            expected_cpu.registers.flag.set_carry(with_carry);

            cp(&mut actual_cpu, val);

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
            let val = if with_half_carry {
                0b00001111
            } else {
                0b00000000
            };

            let expected_zero = false;

            let expected_carry = false;

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_a(init_a);
            actual_cpu.registers.flag.set_zero(!expected_zero);
            actual_cpu.registers.flag.set_sub(false);
            actual_cpu.registers.flag.set_carry(!expected_carry);
            actual_cpu.registers.flag.set_half_carry(!with_half_carry);

            let mut expected_cpu = actual_cpu.clone();
            expected_cpu.registers.flag.set_zero(expected_zero);
            expected_cpu.registers.flag.set_sub(true);
            expected_cpu.registers.flag.set_carry(expected_carry);
            expected_cpu.registers.flag.set_half_carry(with_half_carry);

            cp(&mut actual_cpu, val);

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
}
