use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::registers::Registers;

use super::arithmetic_logical_utils::{dec_d16, inc_d16};
use crate::opcode::types::LoadWordFromRegFn;

// Affect flags: - 0 H C (Z N H C).
fn add_hl(cpu: &mut Cpu, load_from_reg: &LoadWordFromRegFn) -> (Cycle, OpLength) {
    let hl = cpu.registers.hl();
    let value_to_add = load_from_reg(&cpu.registers);

    let (result_value, carry) = hl.overflowing_add(value_to_add);

    // TODO Figure out if behaviour of half-carry is correct.
    let half_result_value = (0x00FF & result_value) as u8;
    let half_carry = half_result_value < cpu.registers.l();

    cpu.registers.set_hl(result_value);

    cpu.registers.flag.set_sub(false);
    cpu.registers.flag.set_carry(carry);
    cpu.registers.flag.set_half_carry(half_carry);

    (Cycle(8), OpLength(1))
}

macro_rules! instruction {
    (inc, $fn_name: ident, $reg_getter:ident, $reg_setter:ident) => {
        pub fn $fn_name(cpu: &mut Cpu) -> (Cycle, OpLength) {
            inc_d16(cpu, &Registers::$reg_getter, &Registers::$reg_setter);

            (Cycle(8), OpLength(1))
        }
    };

    (dec, $fn_name: ident, $reg_getter:ident, $reg_setter:ident) => {
        pub fn $fn_name(cpu: &mut Cpu) -> (Cycle, OpLength) {
            dec_d16(cpu, &Registers::$reg_getter, &Registers::$reg_setter);

            (Cycle(8), OpLength(1))
        }
    };
}

instruction!(inc, inc_bc, bc, set_bc);
instruction!(inc, inc_de, de, set_de);
instruction!(inc, inc_hl, hl, set_hl);
instruction!(inc, inc_sp, sp, set_sp);

instruction!(dec, dec_bc, bc, set_bc);
instruction!(dec, dec_de, de, set_de);
instruction!(dec, dec_hl, hl, set_hl);
instruction!(dec, dec_sp, sp, set_sp);

pub fn add_hl_bc(cpu: &mut Cpu) -> (Cycle, OpLength) {
    add_hl(cpu, &Registers::bc)
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::cpu::Cpu;
    use crate::opcode::types::{Instruction, StoreWordToRegFn};

    macro_rules! test_inc_d16 {
        ($test_name:ident, $op_to_test:ident, $reg_getter:ident, $reg_setter:ident) => {
            #[test]
            fn $test_name() {
                let mut actual_cpu = Cpu::default();

                let mut expecte_cpu = actual_cpu.clone();
                expecte_cpu
                    .registers
                    .$reg_setter(expecte_cpu.registers.$reg_getter() + 1);

                $op_to_test(&mut actual_cpu);

                assert_eq!(actual_cpu, expecte_cpu);
            }
        };
    }

    macro_rules! test_dec_d16 {
        ($test_name:ident, $op_to_test:ident, $reg_getter:ident, $reg_setter:ident) => {
            #[test]
            fn $test_name() {
                let mut actual_cpu = Cpu::default();
                actual_cpu.registers.$reg_setter(0x42);

                let mut expecte_cpu = actual_cpu.clone();
                expecte_cpu
                    .registers
                    .$reg_setter(expecte_cpu.registers.$reg_getter() - 1);

                $op_to_test(&mut actual_cpu);

                assert_eq!(actual_cpu, expecte_cpu);
            }
        };
    }

    test_inc_d16!(run_inc_bc, inc_bc, bc, set_bc);
    test_inc_d16!(run_inc_de, inc_de, de, set_de);
    test_inc_d16!(run_inc_hl, inc_hl, hl, set_hl);
    test_inc_d16!(run_inc_sp, inc_sp, sp, set_sp);

    test_dec_d16!(run_dec_bc, dec_bc, bc, set_bc);
    test_dec_d16!(run_dec_de, dec_de, de, set_de);
    test_dec_d16!(run_dec_hl, dec_hl, hl, set_hl);
    test_dec_d16!(run_dec_sp, dec_sp, sp, set_sp);

    fn run_add_hl(
        inst_to_test: &Instruction,
        store_to_reg: &StoreWordToRegFn,
        with_carry: bool,
        with_half_carry: bool,
    ) {
        let init_hl = 0x0101;

        let init_value = {
            let high_init_value = if with_carry { 0xFF00 } else { 0x1100 };
            let low_init_value = if with_half_carry { 0x00FF } else { 0x0011 };

            high_init_value + low_init_value
        };

        let result_hl = u16::wrapping_add(init_hl, init_value);

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_hl(init_hl);

        store_to_reg(&mut actual_cpu.registers, init_value);

        actual_cpu.registers.flag.set_zero(true);
        actual_cpu.registers.flag.set_sub(true);
        actual_cpu.registers.flag.set_carry(!with_carry);
        actual_cpu.registers.flag.set_half_carry(!with_half_carry);

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_hl(result_hl);
        // Zero flag remain the same.
        expected_cpu.registers.flag.set_zero(true);
        // Sub flag reset.
        expected_cpu.registers.flag.set_sub(false);
        // Carry flag set when carrys from 16bits.
        expected_cpu.registers.flag.set_carry(with_carry);
        // Half carry flag set when carrys from lower 8bits.
        expected_cpu.registers.flag.set_half_carry(with_half_carry);

        inst_to_test(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }

    // add_hl_bc
    fn run_add_hl_bc_without_carry_without_half_carry() {
        run_add_hl(&add_hl_bc, &Registers::set_bc, false, false);
    }

    #[test]
    fn run_add_hl_bc_with_carry_with_half_carry() {
        run_add_hl(&add_hl_bc, &Registers::set_bc, true, true);
    }

    #[test]
    fn run_add_hl_bc_with_carry_without_half_carry() {
        run_add_hl(&add_hl_bc, &Registers::set_bc, true, false);
    }

    #[test]
    fn run_add_hl_bc_without_carry_with_half_carry() {
        run_add_hl(&add_hl_bc, &Registers::set_bc, false, true);
    }
    }
}
