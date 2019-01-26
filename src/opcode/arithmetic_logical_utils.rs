use crate::cpu::Cpu;
use crate::opcode::types::{LoadWordFromRegFn, StoreWordToRegFn};

pub fn inc_d16(cpu: &mut Cpu, load_from_reg: &LoadWordFromRegFn, store_to_reg: &StoreWordToRegFn) {
    let value = load_from_reg(&cpu.registers);

    // TODO Handle overflow.
    store_to_reg(&mut cpu.registers, value + 1);
}

pub fn dec_d16(cpu: &mut Cpu, load_from_reg: &LoadWordFromRegFn, store_to_reg: &StoreWordToRegFn) {
    let value = load_from_reg(&cpu.registers);

    // TODO Handle underflow.
    store_to_reg(&mut cpu.registers, value - 1);
}
