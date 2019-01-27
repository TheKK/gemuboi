use crate::cpu::Cpu;
use crate::mmu;
use crate::opcode::table::{Cycle, OpLength};
use crate::registers::Registers;

pub type InstructionResult = (Cycle, OpLength);
pub type Instruction = Fn(&mut Cpu) -> InstructionResult;

pub type LoadFromFn<S> = Fn(&Cpu) -> mmu::Result<S>;
pub type StoreToFn<S> = Fn(&mut Cpu, S) -> mmu::Result<()>;

pub type LoadFromRegFn<S> = Fn(&Registers) -> S;
pub type StoreToRegFn<S> = Fn(&mut Registers, S);

pub type LoadByteFromRegFn = LoadFromRegFn<u8>;
pub type StoreByteToRegFn = StoreToRegFn<u8>;

pub type LoadWordFromRegFn = LoadFromRegFn<u16>;
pub type StoreWordToRegFn = StoreToRegFn<u16>;
