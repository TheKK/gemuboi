use crate::cpu::Cpu;
use crate::opcode::control;
use crate::opcode::ld_reg_d8;
use crate::opcode::ld_reg_reg;

type OpFn = &'static Fn(&mut Cpu) -> (Cycle, OpLength);

fn unimplement_op_fn(_: &mut Cpu) -> (Cycle, OpLength) {
    unimplemented!("Op code is not implemented yet");
}

pub struct Cycle(pub u8);
pub struct OpLength(pub u8);

pub fn op_table(op_code: u8) -> OpFn {
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

        // ld reg reg.
        0x7F => &ld_reg_reg::ld_a_a,
        0x78 => &ld_reg_reg::ld_a_b,
        0x79 => &ld_reg_reg::ld_a_c,
        0x7A => &ld_reg_reg::ld_a_d,
        0x7B => &ld_reg_reg::ld_a_e,
        0x7C => &ld_reg_reg::ld_a_h,
        0x7D => &ld_reg_reg::ld_a_l,

        0x47 => &ld_reg_reg::ld_b_a,
        0x40 => &ld_reg_reg::ld_b_b,
        0x41 => &ld_reg_reg::ld_b_c,
        0x42 => &ld_reg_reg::ld_b_d,
        0x43 => &ld_reg_reg::ld_b_e,
        0x44 => &ld_reg_reg::ld_b_h,
        0x45 => &ld_reg_reg::ld_b_l,

        0x4F => &ld_reg_reg::ld_c_a,
        0x48 => &ld_reg_reg::ld_c_b,
        0x49 => &ld_reg_reg::ld_c_c,
        0x4A => &ld_reg_reg::ld_c_d,
        0x4B => &ld_reg_reg::ld_c_e,
        0x4C => &ld_reg_reg::ld_c_h,
        0x4D => &ld_reg_reg::ld_c_l,

        0x57 => &ld_reg_reg::ld_d_a,
        0x50 => &ld_reg_reg::ld_d_b,
        0x51 => &ld_reg_reg::ld_d_c,
        0x52 => &ld_reg_reg::ld_d_d,
        0x53 => &ld_reg_reg::ld_d_e,
        0x54 => &ld_reg_reg::ld_d_h,
        0x55 => &ld_reg_reg::ld_d_l,

        0x5F => &ld_reg_reg::ld_e_a,
        0x58 => &ld_reg_reg::ld_e_b,
        0x59 => &ld_reg_reg::ld_e_c,
        0x5A => &ld_reg_reg::ld_e_d,
        0x5B => &ld_reg_reg::ld_e_e,
        0x5C => &ld_reg_reg::ld_e_h,
        0x5D => &ld_reg_reg::ld_e_l,

        0x67 => &ld_reg_reg::ld_h_a,
        0x60 => &ld_reg_reg::ld_h_b,
        0x61 => &ld_reg_reg::ld_h_c,
        0x62 => &ld_reg_reg::ld_h_d,
        0x63 => &ld_reg_reg::ld_h_e,
        0x64 => &ld_reg_reg::ld_h_h,
        0x65 => &ld_reg_reg::ld_h_l,

        0x6F => &ld_reg_reg::ld_l_a,
        0x68 => &ld_reg_reg::ld_l_b,
        0x69 => &ld_reg_reg::ld_l_c,
        0x6A => &ld_reg_reg::ld_l_d,
        0x6B => &ld_reg_reg::ld_l_e,
        0x6C => &ld_reg_reg::ld_l_h,
        0x6D => &ld_reg_reg::ld_l_l,

        _ => &unimplement_op_fn,
    }
}