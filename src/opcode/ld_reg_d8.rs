use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::registers::Registers;

use super::ld_utils::{ld, read_byte_from_pc_offset, store_to_reg};

macro_rules! ld_d8_fn {
    ($fn_name:ident, $reg_setter:ident) => {
        pub fn $fn_name(cpu: &mut Cpu) -> (Cycle, OpLength) {
            ld(
                cpu,
                &read_byte_from_pc_offset(1),
                &store_to_reg(&Registers::$reg_setter),
            );

            (Cycle(8), OpLength(2))
        }
    };
}

ld_d8_fn!(ld_a_d8, set_a);
ld_d8_fn!(ld_b_d8, set_b);
ld_d8_fn!(ld_c_d8, set_c);
ld_d8_fn!(ld_d_d8, set_d);
ld_d8_fn!(ld_e_d8, set_e);
ld_d8_fn!(ld_h_d8, set_h);
ld_d8_fn!(ld_l_d8, set_l);

#[cfg(test)]
mod test {
    use super::*;

    use crate::opcode::ld_utils::{LoadByteFromRegFn, StoreByteToRegFn};
    use crate::opcode::types::Instruction;

    fn test_ld_d8(
        op_to_test: &Instruction,
        load_from_reg: &LoadByteFromRegFn,
        store_to_reg: &StoreByteToRegFn,
    ) {
        // Arrange: prepare CPU and its clone.
        let init_pc = 0x00;
        let the_value = 0x42;

        let mut cpu = Cpu::default();
        cpu.mmu.write_byte(0x00, init_pc).unwrap();
        cpu.mmu.write_byte(0x01, the_value).unwrap();
        cpu.registers.set_pc(init_pc as u16);

        let mut new_cpu = cpu.clone();

        // Action: performing operation.
        op_to_test(&mut new_cpu);

        // Assert: test register X's value.
        let actual_value = load_from_reg(&new_cpu.registers);
        assert_eq!(actual_value, the_value);

        // Assert: make sure only CPU only different in register X's value.
        store_to_reg(&mut cpu.registers, the_value);
        assert!(cpu == new_cpu);
    }

    macro_rules! ld_d8_test {
        ($test_name:ident, $op_to_test:ident, $reg:ident, $reg_setter:ident) => {
            #[test]
            fn $test_name() {
                test_ld_d8(&$op_to_test, &Registers::$reg, &Registers::$reg_setter);
            }
        };
    }

    ld_d8_test!(run_ld_a_d8, ld_a_d8, a, set_a);
    ld_d8_test!(run_ld_b_d8, ld_b_d8, b, set_b);
    ld_d8_test!(run_ld_c_d8, ld_c_d8, c, set_c);
    ld_d8_test!(run_ld_d_d8, ld_d_d8, d, set_d);
    ld_d8_test!(run_ld_e_d8, ld_e_d8, e, set_e);
    ld_d8_test!(run_ld_h_d8, ld_h_d8, h, set_h);
    ld_d8_test!(run_ld_l_d8, ld_l_d8, l, set_l);
}
