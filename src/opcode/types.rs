use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};

pub type Instruction = Fn(&mut Cpu) -> (Cycle, OpLength);
