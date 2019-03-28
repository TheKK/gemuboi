use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::opcode::types::InstructionResult;

pub fn ld_sp_hl(cpu: &mut Cpu) -> InstructionResult {
    cpu.registers.set_sp(cpu.registers.hl());

    (Cycle(8), OpLength(1))
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::cpu::Cpu;

    #[test]
    fn test_ld_sp_hl() {
        let reg_hl = 0x12;

        let expected_sp = reg_hl;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_hl(reg_hl);

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_sp(expected_sp);

        ld_sp_hl(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }
}
