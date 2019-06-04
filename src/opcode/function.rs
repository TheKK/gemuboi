use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::opcode::types::InstructionResult;
use crate::registers::Registers;

pub fn push_af(cpu: &mut Cpu) -> InstructionResult {
    push_reg(cpu, &Registers::af)
}

pub fn push_bc(cpu: &mut Cpu) -> InstructionResult {
    push_reg(cpu, &Registers::bc)
}

pub fn push_de(cpu: &mut Cpu) -> InstructionResult {
    push_reg(cpu, &Registers::de)
}

pub fn push_hl(cpu: &mut Cpu) -> InstructionResult {
    push_reg(cpu, &Registers::hl)
}

#[inline]
fn push(cpu: &mut Cpu, val: u16) {
    let old_sp = cpu.registers.sp();
    let new_sp = old_sp.wrapping_sub(2);

    cpu.mmu.write_word(old_sp, val).unwrap();
    cpu.registers.set_sp(new_sp);
}

#[inline]
fn push_reg(cpu: &mut Cpu, reg: &Fn(&Registers) -> u16) -> InstructionResult {
    push(cpu, reg(&cpu.registers));

    (Cycle(16), OpLength(1))
}

#[cfg(test)]
mod test {
    use crate::cpu::Cpu;

    use super::push;

    #[test]
    fn run_push() {
        let init_sp = 0x42;
        let expected_sp = init_sp - 2;

        let pushed_value = 0x4242;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_sp(init_sp);

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_sp(expected_sp);
        expected_cpu.mmu.write_word(init_sp, pushed_value).unwrap();

        push(&mut actual_cpu, pushed_value);

        assert_eq!(actual_cpu, expected_cpu);
    }
}
