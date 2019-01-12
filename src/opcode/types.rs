use crate::cpu::{Cpu, Cycle, OpLength};

pub type Operation = Fn(&mut Cpu) -> (Cycle, OpLength);
