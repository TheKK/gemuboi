use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::opcode::types::InstructionResult;

pub fn jp_nn(cpu: &mut Cpu) -> InstructionResult {
    let nn = cpu.read_word_argument(1);
    cpu.registers.set_pc(nn);

    (Cycle(12), OpLength(3))
}

pub fn jp_hl_dref(cpu: &mut Cpu) -> InstructionResult {
    let hl = cpu.registers.hl();
    let new_pc = cpu.mmu.read_word(hl);

    cpu.registers.set_pc(new_pc);

    (Cycle(4), OpLength(1))
}

#[cfg(test)]
mod test {
    use crate::cpu::Cpu;

    use super::*;

    #[test]
    fn run_jp_nn() {
        let init_pc = 0xcc;
        let nn = 0x42;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(init_pc);
        actual_cpu.mmu.write_byte(init_pc, 0xc3).unwrap();
        actual_cpu.mmu.write_word(init_pc + 1, nn).unwrap();

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_pc(nn);

        jp_nn(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }

    #[test]
    fn run_jp_hl_dref() {
        let init_pc = 0xcc;
        let init_hl = 0x42;

        let expected_pc = 0x4242;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(init_pc);
        actual_cpu.registers.set_hl(init_hl);
        actual_cpu.mmu.write_word(init_hl, expected_pc).unwrap();

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_pc(expected_pc);

        jp_hl_dref(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }
}
