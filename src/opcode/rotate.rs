use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::opcode::types::InstructionResult;
use crate::registers::Flag;

macro_rules! cb {
    (rlc => $fn_name:ident, $getter:ident, $setter:ident) => {
        pub fn $fn_name(cpu: &mut Cpu) -> InstructionResult {
            let (new_reg, new_flag) = rlc(cpu.registers.$getter());

            cpu.registers.$setter(new_reg);
            cpu.registers.flag = new_flag;

            (Cycle(8), OpLength(1))
        }
    };

    (rl => $fn_name:ident, $getter:ident, $setter:ident) => {
        pub fn $fn_name(cpu: &mut Cpu) -> InstructionResult {
            let (new_reg, new_flag) = rl(cpu.registers.flag.carry(), cpu.registers.$getter());

            cpu.registers.$setter(new_reg);
            cpu.registers.flag = new_flag;

            (Cycle(8), OpLength(1))
        }
    };
}

cb!(rlc => cb_rlca, a, set_a);
cb!(rlc => cb_rlcb, b, set_b);
cb!(rlc => cb_rlcc, c, set_c);
cb!(rlc => cb_rlcd, d, set_d);
cb!(rlc => cb_rlce, e, set_e);
cb!(rlc => cb_rlch, h, set_h);
cb!(rlc => cb_rlcl, l, set_l);

pub fn cb_rlc_hl_dref(cpu: &mut Cpu) -> InstructionResult {
    let hl = cpu.registers.hl();
    let (new_value, new_flag) = rlc(cpu.read_hl_dref());

    cpu.mmu.write_byte(hl, new_value).unwrap();
    cpu.registers.flag = new_flag;

    (Cycle(16), OpLength(1))
}

cb!(rl => cb_rla, a, set_a);
cb!(rl => cb_rlb, b, set_b);
cb!(rl => cb_rlc, c, set_c);
cb!(rl => cb_rld, d, set_d);
cb!(rl => cb_rle, e, set_e);
cb!(rl => cb_rlh, h, set_h);
cb!(rl => cb_rll, l, set_l);

pub fn cb_rl_hl_dref(cpu: &mut Cpu) -> InstructionResult {
    let hl = cpu.registers.hl();
    let (new_value, new_flag) = rl(cpu.registers.flag.carry(), cpu.read_hl_dref());

    cpu.mmu.write_byte(hl, new_value).unwrap();
    cpu.registers.flag = new_flag;

    (Cycle(16), OpLength(1))
}

pub fn rlca(cpu: &mut Cpu) -> InstructionResult {
    let (new_a, new_flag) = rlc(cpu.registers.a());

    cpu.registers.set_a(new_a);
    cpu.registers.flag = new_flag;

    (Cycle(4), OpLength(1))
}

pub fn rla(cpu: &mut Cpu) -> InstructionResult {
    let (new_a, new_flags) = rl(cpu.registers.flag.carry(), cpu.registers.a());

    cpu.registers.set_a(new_a);
    cpu.registers.flag = new_flags;

    (Cycle(4), OpLength(1))
}

pub fn rrca(cpu: &mut Cpu) -> InstructionResult {
    let (new_a, new_flag) = rrc(cpu.registers.a());

    cpu.registers.set_a(new_a);
    cpu.registers.flag = new_flag;

    (Cycle(4), OpLength(1))
}

pub fn rra(cpu: &mut Cpu) -> InstructionResult {
    let (new_a, new_flags) = rr(cpu.registers.flag.carry(), cpu.registers.a());

    cpu.registers.set_a(new_a);
    cpu.registers.flag = new_flags;

    (Cycle(4), OpLength(1))
}

#[inline]
fn rlc(input: u8) -> (u8, Flag) {
    let carry = 0b1000_0000 & input != 0;
    let new_value = input.rotate_left(1);

    (new_value, Flag::new(new_value == 0, false, false, carry))
}

#[inline]
fn rl(carry: bool, input: u8) -> (u8, Flag) {
    let carry_bit = if carry { 1 } else { 0 };

    let new_carry = 0b1000_0000 & input != 0;
    let new_value = (input.rotate_left(1) & 0b1111_1110) + carry_bit;

    (
        new_value,
        Flag::new(new_value == 0, false, false, new_carry),
    )
}

#[inline]
fn rrc(input: u8) -> (u8, Flag) {
    let carry = 0b0000_0001 & input != 0;
    let new_value = input.rotate_right(1);

    (new_value, Flag::new(new_value == 0, false, false, carry))
}

#[inline]
fn rr(carry: bool, input: u8) -> (u8, Flag) {
    let carry_bit = if carry { 1 } else { 0 };

    let new_carry = 0b0000_0001 & input != 0;
    let new_value = (input.rotate_right(1) & 0b0111_1111) + (carry_bit << 7);

    (
        new_value,
        Flag::new(new_value == 0, false, false, new_carry),
    )
}

#[cfg(test)]
mod test {
    use crate::registers::Flag;

    pub use super::*;

    #[test]
    fn run_rlc() {
        assert_eq!(
            rlc(0b00000000),
            (0b00000000, Flag::new(true, false, false, false))
        );
        assert_eq!(
            rlc(0b10000000),
            (0b00000001, Flag::new(false, false, false, true))
        );
        assert_eq!(
            rlc(0b00000001),
            (0b00000010, Flag::new(false, false, false, false))
        );
    }

    #[test]
    fn run_rl() {
        assert_eq!(
            rl(true, 0b00000000),
            (0b00000001, Flag::new(false, false, false, false))
        );
        assert_eq!(
            rl(false, 0b00000000),
            (0b00000000, Flag::new(true, false, false, false))
        );
        assert_eq!(
            rl(true, 0b10000000),
            (0b00000001, Flag::new(false, false, false, true))
        );
        assert_eq!(
            rl(false, 0b10000000),
            (0b00000000, Flag::new(true, false, false, true))
        );
    }

    #[test]
    fn run_rrc() {
        assert_eq!(
            rrc(0b00000000),
            (0b00000000, Flag::new(true, false, false, false))
        );
        assert_eq!(
            rrc(0b10000000),
            (0b01000000, Flag::new(false, false, false, false))
        );
        assert_eq!(
            rrc(0b00000001),
            (0b10000000, Flag::new(false, false, false, true))
        );
    }

    #[test]
    fn run_rr() {
        assert_eq!(
            rr(true, 0b00000000),
            (0b10000000, Flag::new(false, false, false, false))
        );
        assert_eq!(
            rr(false, 0b00000000),
            (0b00000000, Flag::new(true, false, false, false))
        );
        assert_eq!(
            rr(true, 0b00000001),
            (0b10000000, Flag::new(false, false, false, true))
        );
        assert_eq!(
            rr(false, 0b00000001),
            (0b00000000, Flag::new(true, false, false, true))
        );
    }
}
