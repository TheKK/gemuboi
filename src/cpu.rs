use crate::registers::Registers;
use crate::mmu::Mmu;


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
    let (Cycle(cycle), OpLength(len)) = op_table(op_code)(self);

    cycle
  }
}

fn op_table(op_code: u8) -> OpFn {
  match op_code {
    _ => &unimplement_op_fn,
  }
}
