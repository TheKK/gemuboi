use crate::mmu::Mmu;
use crate::opcode::table::{op_table, Cycle, OpLength};
use crate::registers::Registers;

#[derive(Default, PartialEq, Clone)]
pub struct Cpu {
    pub(crate) registers: Registers,
    pub(crate) mmu: Mmu,
}

impl Cpu {
    fn execute_instruction(&mut self, op_code: u8) -> u8 {
        let (Cycle(cycle), OpLength(_len)) = op_table(op_code)(self);

        cycle
    }
}
