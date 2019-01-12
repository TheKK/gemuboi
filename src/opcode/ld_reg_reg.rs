use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::registers::Registers;

use super::ld_utils::{ld, load_from_reg, store_to_reg};

// macro_rules! ff {
//     () => {};
// }

macro_rules! ld_reg_reg_fn {
    // src and dest are the same.
    ($fn_name:ident) => {
        #[inline]
        pub fn $fn_name(_: &mut Cpu) -> (Cycle, OpLength) {
            (Cycle(4), OpLength(1))
        }
    };

    ($fn_name:ident, $store_lhs:ident, $load_rhs:ident) => {
        pub fn $fn_name(cpu: &mut Cpu) -> (Cycle, OpLength) {
            ld(
                cpu,
                &load_from_reg(&Registers::$load_rhs),
                &store_to_reg(&Registers::$store_lhs),
            );

            (Cycle(4), OpLength(1))
        }
    };
}

ld_reg_reg_fn!(ld_a_a);
ld_reg_reg_fn!(ld_b_b);
ld_reg_reg_fn!(ld_c_c);
ld_reg_reg_fn!(ld_d_d);
ld_reg_reg_fn!(ld_e_e);
ld_reg_reg_fn!(ld_h_h);
ld_reg_reg_fn!(ld_l_l);

ld_reg_reg_fn!(ld_a_b, set_a, b);
ld_reg_reg_fn!(ld_a_c, set_a, c);
ld_reg_reg_fn!(ld_a_d, set_a, d);
ld_reg_reg_fn!(ld_a_e, set_a, e);
ld_reg_reg_fn!(ld_a_h, set_a, h);
ld_reg_reg_fn!(ld_a_l, set_a, l);

ld_reg_reg_fn!(ld_b_a, set_b, a);
ld_reg_reg_fn!(ld_b_c, set_b, c);
ld_reg_reg_fn!(ld_b_d, set_b, d);
ld_reg_reg_fn!(ld_b_e, set_b, e);
ld_reg_reg_fn!(ld_b_h, set_b, h);
ld_reg_reg_fn!(ld_b_l, set_b, l);

ld_reg_reg_fn!(ld_c_a, set_c, a);
ld_reg_reg_fn!(ld_c_b, set_c, b);
ld_reg_reg_fn!(ld_c_d, set_c, d);
ld_reg_reg_fn!(ld_c_e, set_c, e);
ld_reg_reg_fn!(ld_c_h, set_c, h);
ld_reg_reg_fn!(ld_c_l, set_c, l);

ld_reg_reg_fn!(ld_d_a, set_d, a);
ld_reg_reg_fn!(ld_d_b, set_d, b);
ld_reg_reg_fn!(ld_d_c, set_d, c);
ld_reg_reg_fn!(ld_d_e, set_d, e);
ld_reg_reg_fn!(ld_d_h, set_d, h);
ld_reg_reg_fn!(ld_d_l, set_d, l);

ld_reg_reg_fn!(ld_e_a, set_e, a);
ld_reg_reg_fn!(ld_e_b, set_e, b);
ld_reg_reg_fn!(ld_e_c, set_e, c);
ld_reg_reg_fn!(ld_e_d, set_e, d);
ld_reg_reg_fn!(ld_e_h, set_e, h);
ld_reg_reg_fn!(ld_e_l, set_e, l);

ld_reg_reg_fn!(ld_h_a, set_h, a);
ld_reg_reg_fn!(ld_h_b, set_h, b);
ld_reg_reg_fn!(ld_h_c, set_h, c);
ld_reg_reg_fn!(ld_h_d, set_h, d);
ld_reg_reg_fn!(ld_h_e, set_h, e);
ld_reg_reg_fn!(ld_h_l, set_h, l);

ld_reg_reg_fn!(ld_l_a, set_l, a);
ld_reg_reg_fn!(ld_l_b, set_l, b);
ld_reg_reg_fn!(ld_l_c, set_l, c);
ld_reg_reg_fn!(ld_l_d, set_l, d);
ld_reg_reg_fn!(ld_l_e, set_l, e);
ld_reg_reg_fn!(ld_l_h, set_l, h);

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! ld_reg_reg_test {
        ($test_name:ident, $ins_to_test:ident, $load:ident, $store:ident) => {
            #[test]
            fn $test_name() {
                // Arrange: prepare CPU and its clone.
                let the_value = 0x42;

                let mut init_cpu = Cpu::default();
                init_cpu.registers.$store(the_value);

                let mut modified_cpu = init_cpu.clone();

                // Action: performing operation.
                $ins_to_test(&mut modified_cpu);

                // Assert: test register lhs's value.
                let actual_value = modified_cpu.registers.$load();
                assert_eq!(actual_value, the_value);

                // Assert: make sure cpu is the same.
                assert!(init_cpu == modified_cpu);
            }
        };

        ($test_name:ident, $ins_to_test:ident, $load_lhs:ident, $store_lhs:ident, $store_rhs:ident) => {
            #[test]
            fn $test_name() {
                // Arrange: prepare CPU and its clone.
                let the_value = 0x42;

                let mut init_cpu = Cpu::default();
                init_cpu.registers.$store_rhs(the_value);

                let mut modified_cpu = init_cpu.clone();

                // Action: performing operation.
                $ins_to_test(&mut modified_cpu);

                // Assert: test register lhs's value.
                let actual_value = modified_cpu.registers.$load_lhs();
                assert_eq!(actual_value, the_value);

                // Assert: make sure only CPU only different in register lhs's value.
                init_cpu.registers.$store_lhs(the_value);
                assert!(init_cpu == modified_cpu);
            }
        };
    }

    ld_reg_reg_test!(run_ld_a_a, ld_a_a, a, set_a);
    ld_reg_reg_test!(run_ld_b_b, ld_b_b, b, set_b);
    ld_reg_reg_test!(run_ld_c_c, ld_c_c, c, set_c);
    ld_reg_reg_test!(run_ld_d_d, ld_d_d, d, set_d);
    ld_reg_reg_test!(run_ld_e_e, ld_e_e, e, set_e);
    ld_reg_reg_test!(run_ld_h_h, ld_h_h, h, set_h);
    ld_reg_reg_test!(run_ld_l_l, ld_l_l, l, set_l);

    ld_reg_reg_test!(run_ld_a_b, ld_a_b, a, set_a, set_b);
    ld_reg_reg_test!(run_ld_a_c, ld_a_c, a, set_a, set_c);
    ld_reg_reg_test!(run_ld_a_d, ld_a_d, a, set_a, set_d);
    ld_reg_reg_test!(run_ld_a_e, ld_a_e, a, set_a, set_e);
    ld_reg_reg_test!(run_ld_a_h, ld_a_h, a, set_a, set_h);
    ld_reg_reg_test!(run_ld_a_l, ld_a_l, a, set_a, set_l);

    ld_reg_reg_test!(run_ld_b_a, ld_b_a, b, set_b, set_a);
    ld_reg_reg_test!(run_ld_b_c, ld_b_c, b, set_b, set_c);
    ld_reg_reg_test!(run_ld_b_d, ld_b_d, b, set_b, set_d);
    ld_reg_reg_test!(run_ld_b_e, ld_b_e, b, set_b, set_e);
    ld_reg_reg_test!(run_ld_b_h, ld_b_h, b, set_b, set_h);
    ld_reg_reg_test!(run_ld_b_l, ld_b_l, b, set_b, set_l);

    ld_reg_reg_test!(run_ld_c_a, ld_c_a, c, set_c, set_a);
    ld_reg_reg_test!(run_ld_c_b, ld_c_b, c, set_c, set_b);
    ld_reg_reg_test!(run_ld_c_d, ld_c_d, c, set_c, set_d);
    ld_reg_reg_test!(run_ld_c_e, ld_c_e, c, set_c, set_e);
    ld_reg_reg_test!(run_ld_c_h, ld_c_h, c, set_c, set_h);
    ld_reg_reg_test!(run_ld_c_l, ld_c_l, c, set_c, set_l);

    ld_reg_reg_test!(run_ld_d_a, ld_d_a, d, set_d, set_a);
    ld_reg_reg_test!(run_ld_d_b, ld_d_b, d, set_d, set_b);
    ld_reg_reg_test!(run_ld_d_c, ld_d_c, d, set_d, set_c);
    ld_reg_reg_test!(run_ld_d_e, ld_d_e, d, set_d, set_e);
    ld_reg_reg_test!(run_ld_d_h, ld_d_h, d, set_d, set_h);
    ld_reg_reg_test!(run_ld_d_l, ld_d_l, d, set_d, set_l);

    ld_reg_reg_test!(run_ld_e_a, ld_e_a, e, set_e, set_a);
    ld_reg_reg_test!(run_ld_e_b, ld_e_b, e, set_e, set_b);
    ld_reg_reg_test!(run_ld_e_c, ld_e_c, e, set_e, set_c);
    ld_reg_reg_test!(run_ld_e_d, ld_e_d, e, set_e, set_d);
    ld_reg_reg_test!(run_ld_e_h, ld_e_h, e, set_e, set_h);
    ld_reg_reg_test!(run_ld_e_l, ld_e_l, e, set_e, set_l);

    ld_reg_reg_test!(run_ld_h_a, ld_h_a, h, set_h, set_a);
    ld_reg_reg_test!(run_ld_h_b, ld_h_b, h, set_h, set_b);
    ld_reg_reg_test!(run_ld_h_c, ld_h_c, h, set_h, set_c);
    ld_reg_reg_test!(run_ld_h_d, ld_h_d, h, set_h, set_d);
    ld_reg_reg_test!(run_ld_h_e, ld_h_e, h, set_h, set_e);
    ld_reg_reg_test!(run_ld_h_l, ld_h_l, h, set_h, set_l);

    ld_reg_reg_test!(run_ld_l_a, ld_l_a, l, set_l, set_a);
    ld_reg_reg_test!(run_ld_l_b, ld_l_b, l, set_l, set_b);
    ld_reg_reg_test!(run_ld_l_c, ld_l_c, l, set_l, set_c);
    ld_reg_reg_test!(run_ld_l_d, ld_l_d, l, set_l, set_d);
    ld_reg_reg_test!(run_ld_l_e, ld_l_e, l, set_l, set_e);
    ld_reg_reg_test!(run_ld_l_h, ld_l_h, l, set_l, set_h);
}
