use crate::cpu::{Cpu, Cycle, OpLength};

pub type Instruction = Fn(&mut Cpu) -> (Cycle, OpLength);
