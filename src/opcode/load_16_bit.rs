use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::opcode::types::InstructionResult;

pub fn ld_sp_hl(cpu: &mut Cpu) -> InstructionResult {
    cpu.registers.set_sp(cpu.registers.hl());

    (Cycle(8), OpLength(1))
}

pub fn ld_a16_sp(cpu: &mut Cpu) -> InstructionResult {
    let arg = cpu.read_word_argument(1);
    let reg_sp = cpu.registers.sp();

    cpu.mmu.write_word(arg, reg_sp).unwrap();

    (Cycle(20), OpLength(3))
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

    #[test]
    fn test_ld_a16_sp() {
        let reg_pc = 0x12;
        let reg_sp = 0x87;

        let a16 = 0x42;

        let expected_value = reg_sp;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(reg_pc);
        actual_cpu.registers.set_sp(reg_sp);

        actual_cpu.mmu.write_word(reg_pc + 1, a16).unwrap();

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.mmu.write_word(a16, expected_value).unwrap();

        ld_a16_sp(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }
}
