use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::opcode::types::InstructionResult;
use crate::registers::Registers;

pub fn ld_bc_d16(cpu: &mut Cpu) -> InstructionResult {
    ld_reg_d16(cpu, &Registers::set_bc)
}

pub fn ld_de_d16(cpu: &mut Cpu) -> InstructionResult {
    ld_reg_d16(cpu, &Registers::set_de)
}

pub fn ld_hl_d16(cpu: &mut Cpu) -> InstructionResult {
    ld_reg_d16(cpu, &Registers::set_hl)
}

pub fn ld_sp_d16(cpu: &mut Cpu) -> InstructionResult {
    ld_reg_d16(cpu, &Registers::set_sp)
}

fn ld_reg_d16(cpu: &mut Cpu, reg_setter: &Fn(&mut Registers, u16)) -> InstructionResult {
    let d16 = cpu.read_word_argument(1);
    reg_setter(&mut cpu.registers, d16);

    (Cycle(12), OpLength(3))
}

#[cfg(test)]
mod test {
    macro_rules! test_ld_reg_d16 {
        ($inst: ident, $reg_setter: ident) => {
            mod $inst {
                use super::super::*;

                use crate::cpu::Cpu;

                #[test]
                fn run() {
                    let pc = 0x42;
                    let expected_bc = 0x1234;

                    let mut actual_cpu = Cpu::default();
                    actual_cpu.registers.set_pc(pc);
                    actual_cpu.mmu.write_word(pc + 1, expected_bc).unwrap();

                    let mut expected_cpu = actual_cpu.clone();
                    expected_cpu.registers.$reg_setter(expected_bc);

                    $inst(&mut actual_cpu);

                    assert_eq!(actual_cpu, expected_cpu);
                }
            }
        };
    }

    test_ld_reg_d16!(ld_bc_d16, set_bc);
    test_ld_reg_d16!(ld_de_d16, set_de);
    test_ld_reg_d16!(ld_hl_d16, set_hl);
    test_ld_reg_d16!(ld_sp_d16, set_sp);
}
