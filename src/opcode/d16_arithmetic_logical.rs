use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::registers::Registers;

use super::arithmetic_logical_utils::inc_d16;

pub fn inc_bc(cpu: &mut Cpu) -> (Cycle, OpLength) {
    inc_d16(cpu, &Registers::bc, &Registers::set_bc);

    (Cycle(8), OpLength(1))
}

pub fn inc_de(cpu: &mut Cpu) -> (Cycle, OpLength) {
    inc_d16(cpu, &Registers::de, &Registers::set_de);

    (Cycle(8), OpLength(1))
}

pub fn inc_hl(cpu: &mut Cpu) -> (Cycle, OpLength) {
    inc_d16(cpu, &Registers::hl, &Registers::set_hl);

    (Cycle(8), OpLength(1))
}

pub fn inc_sp(cpu: &mut Cpu) -> (Cycle, OpLength) {
    inc_d16(cpu, &Registers::sp, &Registers::set_sp);

    (Cycle(8), OpLength(1))
}

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
