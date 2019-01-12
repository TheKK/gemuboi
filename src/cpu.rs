use crate::registers::Registers;
use crate::mmu::Mmu;

use crate::opcode::control;
use crate::opcode::ld_reg_d8;

pub struct Cycle(pub u8);
pub struct OpLength(pub u8);

type OpFn = &'static Fn(&mut Cpu) -> (Cycle, OpLength);

fn unimplement_op_fn(_: &mut Cpu) -> (Cycle, OpLength) {
  unimplemented!("Op code is not implemented yet");
}

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

fn op_table(op_code: u8) -> OpFn {
  match op_code {
    0x00 => &control::nop,

    // ld reg d8.
    0x06 => &ld_reg_d8::ld_b_d8,
    0x0E => &ld_reg_d8::ld_c_d8,
    0x16 => &ld_reg_d8::ld_d_d8,
    0x1E => &ld_reg_d8::ld_e_d8,
    0x26 => &ld_reg_d8::ld_h_d8,
    0x2E => &ld_reg_d8::ld_l_d8,
    0x3E => &ld_reg_d8::ld_a_d8,

    _ => &unimplement_op_fn,
  }
}
