use crate::cpu::Cpu;
use crate::mmu;
use crate::opcode::table::{Cycle, OpFn, OpLength};
use crate::registers::Registers;

pub type LoadFromFn<S> = Fn(&Cpu) -> mmu::Result<S>;
pub type StoreToFn<S> = Fn(&mut Cpu, S) -> mmu::Result<()>;

pub type LoadFromRegFn<S> = Fn(&Registers) -> S;
pub type StoreToRegFn<S> = Fn(&mut Registers, S);

pub type LoadByteFromRegFn = LoadFromRegFn<u8>;
pub type StoreByteToRegFn = StoreToRegFn<u8>;

pub type LoadWordFromRegFn = LoadFromRegFn<u16>;

pub fn ldi_instruction(cpu: &mut Cpu, op: &OpFn) -> (Cycle, OpLength) {
    let result = op(cpu);
    cpu.registers.set_hl(cpu.registers.hl() + 1);

    result
}

pub fn ldd_instruction(cpu: &mut Cpu, op: &OpFn) -> (Cycle, OpLength) {
    let result = op(cpu);
    cpu.registers.set_hl(cpu.registers.hl() - 1);

    result
}

#[inline]
pub fn read_byte_from_pc_offset(offset: u16) -> impl Fn(&Cpu) -> mmu::Result<u8> {
    move |cpu| {
        let pc = cpu.registers.pc();

        Ok(cpu.mmu.read_byte(pc + offset))
    }
}

#[inline]
pub fn read_word_from_pc_offset(offset: u16) -> impl Fn(&Cpu) -> mmu::Result<u16> {
    move |cpu| {
        let pc = cpu.registers.pc();

        cpu.mmu.read_word(pc + offset)
    }
}

#[inline]
pub fn load_from_reg<S>(from_reg: &'static LoadFromRegFn<S>) -> impl Fn(&Cpu) -> mmu::Result<S> {
    move |cpu: &Cpu| {
        let value = from_reg(&cpu.registers);

        Ok(value)
    }
}

#[inline]
pub fn load_byte_from_reg_dref(
    from_reg: &'static LoadWordFromRegFn,
) -> impl Fn(&Cpu) -> mmu::Result<u8> {
    move |cpu: &Cpu| {
        let addr = from_reg(&cpu.registers);

        Ok(cpu.mmu.read_byte(addr))
    }
}

#[inline]
pub fn store_to_reg<S>(
    to_reg: &'static StoreToRegFn<S>,
) -> impl Fn(&mut Cpu, S) -> mmu::Result<()> {
    move |cpu: &mut Cpu, v: S| {
        to_reg(&mut cpu.registers, v);

        Ok(())
    }
}

#[inline]
pub fn store_to_reg_dref(
    the_reg: &'static LoadWordFromRegFn,
) -> impl Fn(&mut Cpu, u8) -> mmu::Result<()> {
    move |cpu: &mut Cpu, v: u8| {
        let the_addr = the_reg(&mut cpu.registers);

        cpu.mmu.write_byte(the_addr, v)
    }
}

#[inline]
pub fn ld<S>(cpu: &mut Cpu, load_from: &LoadFromFn<S>, store_to: &StoreToFn<S>) {
    load_from(cpu)
        .and_then(|value| store_to(cpu, value))
        .expect("occur failure while performing ld op code");
}
