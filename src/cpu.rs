use crate::mmu::Mmu;
use crate::opcode::table::{op_table, Cycle, OpLength};
use crate::registers::Registers;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Cpu {
    pub(crate) registers: Registers,
    pub(crate) mmu: Mmu,
}

impl Cpu {
    fn execute_instruction(&mut self, op_code: u8) -> u8 {
        let (Cycle(cycle), OpLength(_len)) = op_table(op_code)(self);

        cycle
    }

    pub fn read_byte_argument(&self, index: u16) -> u8 {
        let arg_addr = self.registers.pc().saturating_add(index);

        self.mmu.read_byte(arg_addr)
    }
}

#[cfg(test)]
mod test {
    use super::Cpu;

    #[test]
    fn read_byte_argument_normal() {
        let pc = 0x42;

        let arg_index = 1;
        let arg_value = 0x12;

        let mut cpu = Cpu::default();
        cpu.mmu.write_byte(pc + arg_index, arg_value).unwrap();
        cpu.registers.set_pc(pc);

        assert_eq!(arg_value, cpu.read_byte_argument(arg_index));
    }

    #[test]
    fn read_byte_argument_out_of_bound() {
        use crate::mmu::INVALID_READ_DEFAULT_VALUE;

        let pc = 0x10;
        let arg_index = 0xFFFF;

        let mut cpu = Cpu::default();
        cpu.registers.set_pc(pc);

        assert_eq!(
            cpu.read_byte_argument(arg_index),
            INVALID_READ_DEFAULT_VALUE
        );
    }
}
