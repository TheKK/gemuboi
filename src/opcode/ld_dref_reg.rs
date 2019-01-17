use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::registers::Registers;

use super::ld_utils::{ld, load_from_reg, store_to_reg_dref};

macro_rules! ld_dref_reg_fn {
    ($fn_name:ident, $addr_reg_getter:ident, $val_reg_getter:ident) => {
        pub fn $fn_name(cpu: &mut Cpu) -> (Cycle, OpLength) {
            ld(
                cpu,
                &load_from_reg(&Registers::$val_reg_getter),
                &store_to_reg_dref(&Registers::$addr_reg_getter),
            );

            (Cycle(8), OpLength(1))
        }
    };
}

ld_dref_reg_fn!(ld_bc_dref_a, bc, a);
ld_dref_reg_fn!(ld_de_dref_a, de, a);

ld_dref_reg_fn!(ld_hl_dref_a, hl, a);
ld_dref_reg_fn!(ld_hl_dref_b, hl, b);
ld_dref_reg_fn!(ld_hl_dref_c, hl, c);
ld_dref_reg_fn!(ld_hl_dref_d, hl, d);
ld_dref_reg_fn!(ld_hl_dref_e, hl, e);
ld_dref_reg_fn!(ld_hl_dref_h, hl, h);
ld_dref_reg_fn!(ld_hl_dref_l, hl, l);

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! ld_dref_reg_test {
        ($test_name:ident, $ins_to_test:ident, $addr_reg_setter:ident, $val_reg_setter:ident) => {
            #[test]
            fn $test_name() {
                // Arrange: prepare cpu.
                // The value is for instruction like "ld_hl_dref_h" and "ld_hl_dref_h" which
                // overlaped. Repeated value avoid unexpected behaviour.
                let the_addr = 0x4242;
                let the_value = 0x42;

                let mut actual_cpu = Cpu::default();
                actual_cpu.registers.$addr_reg_setter(the_addr);
                actual_cpu.registers.$val_reg_setter(the_value);

                let mut expected_cpu = actual_cpu.clone();
                expected_cpu
                    .mmu
                    .write_byte(the_addr, the_value)
                    .unwrap();

                // Action.cpu
                $ins_to_test(&mut actual_cpu);

                // Assert: check cpu state.
                assert_eq!(actual_cpu, expected_cpu);
            }
        };
    }

    ld_dref_reg_test!(run_ld_bc_dref_a, ld_bc_dref_a, set_bc, set_a);
    ld_dref_reg_test!(run_ld_de_dref_a, ld_de_dref_a, set_de, set_a);

    ld_dref_reg_test!(run_ld_hl_dref_a, ld_hl_dref_a, set_hl, set_a);
    ld_dref_reg_test!(run_ld_hl_dref_b, ld_hl_dref_b, set_hl, set_b);
    ld_dref_reg_test!(run_ld_hl_dref_c, ld_hl_dref_c, set_hl, set_c);
    ld_dref_reg_test!(run_ld_hl_dref_d, ld_hl_dref_d, set_hl, set_d);
    ld_dref_reg_test!(run_ld_hl_dref_e, ld_hl_dref_e, set_hl, set_e);
    ld_dref_reg_test!(run_ld_hl_dref_h, ld_hl_dref_h, set_hl, set_h);
    ld_dref_reg_test!(run_ld_hl_dref_l, ld_hl_dref_l, set_hl, set_l);

    // ld_dref_reg_test!(run_ld_c_dref_a, ld_c_dref_a, set_c, set_a);

    // ld_dref_reg_test!(run_ld_a16_dref_a, ld_a16_dref_a, set_a);
}
