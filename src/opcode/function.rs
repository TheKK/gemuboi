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

pub fn call_nn(cpu: &mut Cpu) -> InstructionResult {
    call_if(cpu, &|_| true)
}

pub fn call_nz(cpu: &mut Cpu) -> InstructionResult {
    call_if(cpu, &|registers| !registers.flag.zero())
}

pub fn call_z(cpu: &mut Cpu) -> InstructionResult {
    call_if(cpu, &|registers| registers.flag.zero())
}

pub fn call_nc(cpu: &mut Cpu) -> InstructionResult {
    call_if(cpu, &|registers| !registers.flag.carry())
}

pub fn call_c(cpu: &mut Cpu) -> InstructionResult {
    call_if(cpu, &|registers| registers.flag.carry())
}

#[inline]
fn call_if(cpu: &mut Cpu, cond: &Fn(&Registers) -> bool) -> InstructionResult {
    if cond(&cpu.registers) {
        let new_pc = cpu.read_word_argument(1);
        let new_sp = cpu.registers.sp() - 2;
        let ret_pc = cpu.registers.pc() + 3;

        cpu.registers.set_pc(new_pc);
        cpu.registers.set_sp(new_sp);
        cpu.mmu.write_word(new_sp, ret_pc).unwrap();

        (Cycle(24), OpLength(3))
    } else {
        (Cycle(12), OpLength(3))
    }
}

pub fn ret(cpu: &mut Cpu) -> InstructionResult {
    let sp = cpu.registers.sp();
    let ret_pc = cpu.mmu.read_word(sp);

    cpu.registers.set_sp(sp + 2);
    cpu.registers.set_pc(ret_pc);

    (Cycle(16), OpLength(1))
}

#[cfg(test)]
mod test {
    use crate::cpu::Cpu;

    use super::*;

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

    #[test]
    fn run_call_if_with_true() {
        let init_pc = 0x55;
        let init_sp = 0x42;

        let nn = 0x12;

        let expected_pc = nn;
        let expected_sp = init_sp - 2;

        let expected_next_pc = init_pc + 3;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(init_pc);
        actual_cpu.registers.set_sp(init_sp);
        actual_cpu.mmu.write_word(init_pc + 1, nn).unwrap();

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_pc(expected_pc);
        expected_cpu.registers.set_sp(expected_sp);
        expected_cpu
            .mmu
            .write_word(expected_sp, expected_next_pc)
            .unwrap();

        call_if(&mut actual_cpu, &|_| true);

        assert_eq!(actual_cpu, expected_cpu);
    }

    #[test]
    fn run_call_if_with_false() {
        let init_pc = 0x55;
        let init_sp = 0x42;

        let nn = 0x12;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(init_pc);
        actual_cpu.registers.set_sp(init_sp);
        actual_cpu.mmu.write_word(init_pc + 1, nn).unwrap();

        let expected_cpu = actual_cpu.clone();

        call_if(&mut actual_cpu, &|_| false);

        assert_eq!(actual_cpu, expected_cpu);
    }

    #[test]
    fn run_ret() {
        let init_pc = 0x55;
        let init_sp = 0x42;

        let ret_pc = 0x12;

        let expected_pc = ret_pc;
        let expected_sp = init_sp + 2;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(init_pc);
        actual_cpu.registers.set_sp(init_sp);
        actual_cpu.mmu.write_word(init_sp, ret_pc).unwrap();

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_pc(expected_pc);
        expected_cpu.registers.set_sp(expected_sp);

        ret(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }
}
