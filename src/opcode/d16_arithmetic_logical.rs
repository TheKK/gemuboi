use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::registers::Registers;

use super::arithmetic_logical_utils::inc_d16;

macro_rules! instruction {
    (inc, $fn_name: ident, $reg_getter:ident, $reg_setter:ident) => {
        pub fn $fn_name(cpu: &mut Cpu) -> (Cycle, OpLength) {
            inc_d16(cpu, &Registers::$reg_getter, &Registers::$reg_setter);

            (Cycle(8), OpLength(1))
        }
    };
}

instruction!(inc, inc_bc, bc, set_bc);
instruction!(inc, inc_de, de, set_de);
instruction!(inc, inc_hl, hl, set_hl);
instruction!(inc, inc_sp, sp, set_sp);

#[cfg(test)]
mod test {
    use super::*;

    use crate::cpu::Cpu;

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

    test_inc_d16!(run_inc_bc, inc_bc, bc, set_bc);
    test_inc_d16!(run_inc_de, inc_de, de, set_de);
    test_inc_d16!(run_inc_hl, inc_hl, hl, set_hl);
    test_inc_d16!(run_inc_sp, inc_sp, sp, set_sp);
}
