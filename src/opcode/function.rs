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

pub fn pop_af(cpu: &mut Cpu) -> InstructionResult {
    pop_reg(cpu, &Registers::set_af)
}

pub fn pop_bc(cpu: &mut Cpu) -> InstructionResult {
    pop_reg(cpu, &Registers::set_bc)
}

pub fn pop_de(cpu: &mut Cpu) -> InstructionResult {
    pop_reg(cpu, &Registers::set_de)
}

pub fn pop_hl(cpu: &mut Cpu) -> InstructionResult {
    pop_reg(cpu, &Registers::set_hl)
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

#[inline]
fn pop(cpu: &mut Cpu) -> u16 {
    let old_sp = cpu.registers.sp();

    let new_sp = old_sp.wrapping_add(2);
    cpu.registers.set_sp(new_sp);

    cpu.mmu.read_word(old_sp)
}

#[inline]
fn pop_reg(cpu: &mut Cpu, set_reg: &Fn(&mut Registers, u16)) -> InstructionResult {
    let value = pop(cpu);
    set_reg(&mut cpu.registers, value);

    (Cycle(12), OpLength(1))
}

#[cfg(test)]
mod test {
    use crate::cpu::Cpu;

    use super::pop;
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

    #[test]
    fn run_pop() {
        let init_sp = 0x42;
        let expected_sp = init_sp + 2;

        let expected_popped_value = 0x4242;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_sp(init_sp);
        actual_cpu
            .mmu
            .write_word(init_sp, expected_popped_value)
            .unwrap();

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_sp(expected_sp);

        let actual_popped_value = pop(&mut actual_cpu);

        assert_eq!(actual_popped_value, expected_popped_value);
        assert_eq!(actual_cpu, expected_cpu);
    }
}
