use crate::carry_test::{CarryTest, CarryTestResult};
use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::opcode::types::InstructionResult;

pub fn ld_sp_hl(cpu: &mut Cpu) -> InstructionResult {
    cpu.registers.set_sp(cpu.registers.hl());

    (Cycle(8), OpLength(1))
}

pub fn ld_a16_sp(cpu: &mut Cpu) -> InstructionResult {
    let arg = cpu.read_word_argument(1);
    let reg_sp = cpu.registers.sp();

    cpu.mmu.write_word(arg, reg_sp).unwrap();

    (Cycle(20), OpLength(3))
}

pub fn ld_hl_sp_n(cpu: &mut Cpu) -> InstructionResult {
    let raw_arg = cpu.read_byte_argument(1);

    let signed_arg = i8::from_ne_bytes([raw_arg]);
    let unsigned_arg = signed_arg.abs() as u16;

    let sp = cpu.registers.sp();

    let CarryTestResult {
        val: new_hl,
        half_carry,
        carry,
    } = if signed_arg.is_positive() {
        sp.carry_add(unsigned_arg)
    } else {
        sp.carry_sub(unsigned_arg)
    };

    cpu.registers.set_hl(new_hl);

    cpu.registers.flag.set_zero(false);
    cpu.registers.flag.set_sub(false);
    cpu.registers.flag.set_half_carry(half_carry);
    cpu.registers.flag.set_carry(carry);

    (Cycle(12), OpLength(2))
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::cpu::Cpu;

    #[test]
    fn test_ld_sp_hl() {
        let reg_hl = 0x12;

        let expected_sp = reg_hl;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_hl(reg_hl);

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_sp(expected_sp);

        ld_sp_hl(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }

    #[test]
    fn test_ld_a16_sp() {
        let reg_pc = 0x12;
        let reg_sp = 0x87;

        let a16 = 0x42;

        let expected_value = reg_sp;

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(reg_pc);
        actual_cpu.registers.set_sp(reg_sp);

        actual_cpu.mmu.write_word(reg_pc + 1, a16).unwrap();

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.mmu.write_word(a16, expected_value).unwrap();

        ld_a16_sp(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }

    fn run_ld_hl_sp_n(
        init_sp: u16,
        signed_arg: i8,
        expected_hl: u16,
        expected_half_carry: Option<bool>,
        expected_carry: Option<bool>,
    ) {
        use std::ops::Not;

        let id = |e| e;

        let init_pc = 0x12;
        let init_hl = 0;

        let arg = u8::from_ne_bytes(signed_arg.to_ne_bytes());

        let mut actual_cpu = Cpu::default();
        actual_cpu.registers.set_pc(init_pc);
        actual_cpu.registers.set_sp(init_sp);
        actual_cpu.registers.set_hl(init_hl);

        actual_cpu.registers.flag.set_zero(true);
        actual_cpu.registers.flag.set_sub(true);

        actual_cpu
            .registers
            .flag
            .set_half_carry(expected_half_carry.map_or(false, &Not::not));
        actual_cpu
            .registers
            .flag
            .set_carry(expected_carry.map_or(false, &Not::not));

        actual_cpu.mmu.write_byte(init_pc + 1, arg).unwrap();

        let mut expected_cpu = actual_cpu.clone();
        expected_cpu.registers.set_hl(expected_hl);

        expected_cpu.registers.flag.set_zero(false);
        expected_cpu.registers.flag.set_sub(false);

        expected_cpu
            .registers
            .flag
            .set_half_carry(expected_half_carry.map_or(false, &id));
        expected_cpu
            .registers
            .flag
            .set_carry(expected_carry.map_or(false, &id));

        ld_hl_sp_n(&mut actual_cpu);

        assert_eq!(actual_cpu, expected_cpu);
    }

    #[test]
    fn test_ld_hl_sp_n_with_zero_arg() {
        run_ld_hl_sp_n(123, 0, 123, None, None);
    }

    #[test]
    fn test_ld_hl_sp_n_with_positive_arg() {
        run_ld_hl_sp_n(123, 10, 133, None, None);
    }

    #[test]
    fn test_ld_hl_sp_n_with_negtive_arg() {
        run_ld_hl_sp_n(123, -1, 122, None, None);
    }

    #[test]
    fn test_ld_hl_sp_n_with_carry_flag() {
        run_ld_hl_sp_n(0xFFFF, 1, 0, Some(true), Some(true));
    }

    #[test]
    fn test_ld_hl_sp_n_with_half_carry_flag() {
        run_ld_hl_sp_n(0x00FF, 1, 0x0100, Some(true), Some(false));
    }
}
