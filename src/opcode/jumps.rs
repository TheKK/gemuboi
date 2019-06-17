use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::opcode::types::InstructionResult;

pub fn jp_nn(cpu: &mut Cpu) -> InstructionResult {
    let nn = cpu.read_word_argument(1);
    cpu.registers.set_pc(nn);

    (Cycle(12), OpLength(3))
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
}
