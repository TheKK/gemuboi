use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::opcode::types::InstructionResult;
use crate::registers::Registers;

fn ld_reg_d16(cpu: &mut Cpu, reg_setter: &Fn(&mut Registers, u16)) -> InstructionResult {
    let d16 = cpu.read_word_argument(1);
    reg_setter(&mut cpu.registers, d16);

    (Cycle(12), OpLength(3))
}

pub fn ld_bc_d16(cpu: &mut Cpu) -> InstructionResult {
    ld_reg_d16(cpu, &Registers::set_bc)
}

#[cfg(test)]
mod test {
    mod ld_bc_d16 {
        use super::super::*;

        use crate::cpu::Cpu;

        #[test]
        fn run() {
            let pc = 0x42;
            let expected_bc = 0x1234;

            let mut actual_cpu = Cpu::default();
            actual_cpu.registers.set_pc(pc);
            actual_cpu.mmu.write_word(pc + 1, expected_bc).unwrap();

            let mut expected_cpu = actual_cpu.clone();
            expected_cpu.registers.set_bc(expected_bc);

            ld_bc_d16(&mut actual_cpu);

            assert_eq!(actual_cpu.registers.bc(), expected_cpu.registers.bc());
        }
    }
}
