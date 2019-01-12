use crate::cpu::Cpu;
use crate::mmu;
use crate::registers::Registers;

pub type LoadFromFn<S> = Fn(&Cpu) -> mmu::Result<S>;
pub type StoreToFn<S> = Fn(&mut Cpu, S) -> mmu::Result<()>;

pub type LoadFromRegFn<S> = Fn(&Registers) -> S;
pub type StoreToRegFn<S> = Fn(&mut Registers, S);

pub type LoadByteFromRegFn = LoadFromRegFn<u8>;
pub type StoreByteToRegFn = StoreToRegFn<u8>;

#[inline]
pub fn read_byte_from_pc_offset(offset: usize) -> impl Fn(&Cpu) -> mmu::Result<u8> {
    move |cpu| {
        let pc = cpu.registers.pc() as usize;

        cpu.mmu.read_byte(pc + offset)
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
pub fn store_to_reg<S>(
    to_reg: &'static StoreToRegFn<S>,
) -> impl Fn(&mut Cpu, S) -> mmu::Result<()> {
    move |cpu: &mut Cpu, v: S| {
        to_reg(&mut cpu.registers, v);

        Ok(())
    }
}

#[inline]
pub fn ld<S>(cpu: &mut Cpu, load_from: &LoadFromFn<S>, store_to: &StoreToFn<S>) {
    load_from(cpu)
        .and_then(|value| store_to(cpu, value))
        .expect("occur failure while performing ld op code");
}
