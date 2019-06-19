use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::opcode::types::InstructionResult;
use crate::registers::Registers;

pub fn jp_nn(cpu: &mut Cpu) -> InstructionResult {
    let nn = cpu.read_word_argument(1);
    cpu.registers.set_pc(nn);

    (Cycle(12), OpLength(3))
}

pub fn jp_hl(cpu: &mut Cpu) -> InstructionResult {
    let hl = cpu.registers.hl();

    cpu.registers.set_pc(hl);

    (Cycle(4), OpLength(1))
}

pub fn jp_nz(cpu: &mut Cpu) -> InstructionResult {
    jp_if(cpu, &|registers: &Registers| !registers.flag.zero())
}

pub fn jp_z(cpu: &mut Cpu) -> InstructionResult {
    jp_if(cpu, &|registers: &Registers| registers.flag.zero())
}

pub fn jp_nc(cpu: &mut Cpu) -> InstructionResult {
    jp_if(cpu, &|registers: &Registers| !registers.flag.carry())
}

pub fn jp_c(cpu: &mut Cpu) -> InstructionResult {
    jp_if(cpu, &|registers: &Registers| registers.flag.carry())
}

#[inline]
fn jp_if(cpu: &mut Cpu, cond: &Fn(&Registers) -> bool) -> InstructionResult {
    let new_pc = cpu.read_word_argument(1);

    if cond(&cpu.registers) {
        cpu.registers.set_pc(new_pc);
        (Cycle(16), OpLength(3))
    } else {
        (Cycle(12), OpLength(3))
    }
}

pub fn jr_n(cpu: &mut Cpu) -> InstructionResult {
    jr_if(cpu, &|_| true)
}

pub fn jr_nz(cpu: &mut Cpu) -> InstructionResult {
    jr_if(cpu, &|registers: &Registers| !registers.flag.zero())
}

pub fn jr_z(cpu: &mut Cpu) -> InstructionResult {
    jr_if(cpu, &|registers: &Registers| registers.flag.zero())
}

pub fn jr_nc(cpu: &mut Cpu) -> InstructionResult {
    jr_if(cpu, &|registers: &Registers| !registers.flag.carry())
}

pub fn jr_c(cpu: &mut Cpu) -> InstructionResult {
    jr_if(cpu, &|registers: &Registers| registers.flag.carry())
}

#[inline]
fn jr_if(cpu: &mut Cpu, cond: &Fn(&Registers) -> bool) -> InstructionResult {
    let pc = cpu.registers.pc();
    let pc_offset = cpu.read_byte_argument(1) as i8;

    if cond(&cpu.registers) {
        let new_pc = if pc_offset < 0 {
            pc - (pc_offset.abs() as u16)
        } else {
            pc + (pc_offset as u16)
        };

        cpu.registers.set_pc(new_pc);

        (Cycle(12), OpLength(2))
    } else {
        (Cycle(8), OpLength(2))
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::Cpu;

    use super::*;

    #[test]
    fn run_jp_nn() {
        let init_pc = 0xcc;
        let nn = 0x42;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(init_pc);
        actual_cpu.mmu.write_byte(init_pc, 0xc3).unwrap();
        actual_cpu.mmu.write_word(init_pc + 1, nn).unwrap();

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_pc(nn);

        jp_nn(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }

    #[test]
    fn run_jp_hl() {
        let init_pc = 0xcc;
        let init_hl = 0x42;

        let expected_pc = init_hl;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(init_pc);
        actual_cpu.registers.set_hl(init_hl);

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_pc(expected_pc);

        jp_hl(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }

    #[test]
    fn run_jp_nz_with_zero_flag_set() {
        let init_pc = 0xcc;
        let zero_flag = true;

        let expected_pc = 0x4242;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(init_pc);
        actual_cpu.registers.flag.set_zero(zero_flag);
        actual_cpu.mmu.write_word(init_pc + 1, expected_pc).unwrap();

        let expected_cpu = actual_cpu.clone();

        jp_nz(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }

    #[test]
    fn run_jp_nz_with_zero_flag_unset() {
        let init_pc = 0xcc;
        let zero_flag = false;

        let expected_pc = 0x4242;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(init_pc);
        actual_cpu.registers.flag.set_zero(zero_flag);
        actual_cpu.mmu.write_word(init_pc + 1, expected_pc).unwrap();

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_pc(expected_pc);

        jp_nz(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }

    #[test]
    fn run_jp_z_with_zero_flag_set() {
        let init_pc = 0xcc;
        let zero_flag = true;

        let expected_pc = 0x4242;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(init_pc);
        actual_cpu.registers.flag.set_zero(zero_flag);
        actual_cpu.mmu.write_word(init_pc + 1, expected_pc).unwrap();

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_pc(expected_pc);

        jp_z(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }

    #[test]
    fn run_jp_z_with_zero_flag_unset() {
        let init_pc = 0xcc;
        let zero_flag = false;

        let expected_pc = 0x4242;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(init_pc);
        actual_cpu.registers.flag.set_zero(zero_flag);
        actual_cpu.mmu.write_word(init_pc + 1, expected_pc).unwrap();

        let expected_cpu = actual_cpu.clone();

        jp_z(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }

    #[test]
    fn run_jp_nc_with_carry_flag_set() {
        let init_pc = 0xcc;
        let carry_flag = true;

        let expected_pc = 0x4242;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(init_pc);
        actual_cpu.registers.flag.set_carry(carry_flag);
        actual_cpu.mmu.write_word(init_pc + 1, expected_pc).unwrap();

        let expected_cpu = actual_cpu.clone();

        jp_nc(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }

    #[test]
    fn run_jp_nc_with_carry_flag_unset() {
        let init_pc = 0xcc;
        let carry_flag = false;

        let expected_pc = 0x4242;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(init_pc);
        actual_cpu.registers.flag.set_carry(carry_flag);
        actual_cpu.mmu.write_word(init_pc + 1, expected_pc).unwrap();

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_pc(expected_pc);

        jp_nc(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }

    #[test]
    fn run_jp_c_with_carry_flag_set() {
        let init_pc = 0xcc;
        let carry_flag = true;

        let expected_pc = 0x4242;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(init_pc);
        actual_cpu.registers.flag.set_carry(carry_flag);
        actual_cpu.mmu.write_word(init_pc + 1, expected_pc).unwrap();

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_pc(expected_pc);

        jp_c(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }

    #[test]
    fn run_jp_c_with_carry_flag_unset() {
        let init_pc = 0xcc;
        let carry_flag = false;

        let expected_pc = 0x4242;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(init_pc);
        actual_cpu.registers.flag.set_carry(carry_flag);
        actual_cpu.mmu.write_word(init_pc + 1, expected_pc).unwrap();

        let expected_cpu = actual_cpu.clone();

        jp_c(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }

    #[test]
    fn run_jr_n_with_positive_value() {
        let init_pc = 0xcc;
        let n = 0x10;

        let expected_pc = init_pc + u16::from(n);

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(init_pc);
        actual_cpu.mmu.write_byte(init_pc + 1, n).unwrap();

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_pc(expected_pc);

        jr_n(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }

    #[test]
    fn run_jr_n_with_negative_value() {
        let init_pc = 0xcc;
        let negative_n = -10_i8;
        let n = negative_n as u8;

        let expected_pc = init_pc - 10;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(init_pc);
        actual_cpu.mmu.write_byte(init_pc + 1, n).unwrap();

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_pc(expected_pc);

        jr_n(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }
}
