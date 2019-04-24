use crate::cpu::Cpu;
use crate::opcode::control;
use crate::opcode::d16_arithmetic_logical;
use crate::opcode::d8_arithmetic_logical;
use crate::opcode::ld_dref_reg;
use crate::opcode::ld_reg_d16;
use crate::opcode::ld_reg_d8;
use crate::opcode::ld_reg_dref;
use crate::opcode::ld_reg_reg;
use crate::opcode::load_16_bit;

pub type OpFn = Fn(&mut Cpu) -> (Cycle, OpLength);

fn unimplement_op_fn(_: &mut Cpu) -> (Cycle, OpLength) {
    unimplemented!("Op code is not implemented yet");
}

pub struct Cycle(pub u8);
pub struct OpLength(pub u8);

pub fn op_table(op_code: u8) -> &'static OpFn {
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

        // ld reg dref
        0x0A => &ld_reg_dref::ld_a_bc_dref,
        0x1A => &ld_reg_dref::ld_a_de_dref,

        0x46 => &ld_reg_dref::ld_b_hl_dref,
        0x4E => &ld_reg_dref::ld_c_hl_dref,
        0x56 => &ld_reg_dref::ld_d_hl_dref,
        0x5E => &ld_reg_dref::ld_e_hl_dref,
        0x66 => &ld_reg_dref::ld_h_hl_dref,
        0x6E => &ld_reg_dref::ld_l_hl_dref,
        0x7E => &ld_reg_dref::ld_a_hl_dref,

        // ldh a hl dref & friends
        0x2A => &ld_reg_dref::ldi_a_hl_dref,
        0x3A => &ld_reg_dref::ldd_a_hl_dref,

        0xF0 => &ld_reg_dref::ldh_a_a8_dref,
        0xFA => &ld_reg_dref::ld_a_a16_dref,
        0xF2 => &ld_reg_dref::ld_a_c_dref,

        // ld dref reg
        0x02 => &ld_dref_reg::ld_bc_dref_a,
        0x12 => &ld_dref_reg::ld_de_dref_a,

        0x77 => &ld_dref_reg::ld_hl_dref_a,
        0x70 => &ld_dref_reg::ld_hl_dref_b,
        0x71 => &ld_dref_reg::ld_hl_dref_c,
        0x72 => &ld_dref_reg::ld_hl_dref_d,
        0x73 => &ld_dref_reg::ld_hl_dref_e,
        0x74 => &ld_dref_reg::ld_hl_dref_h,
        0x75 => &ld_dref_reg::ld_hl_dref_l,

        0x22 => &ld_dref_reg::ldi_hl_dref_a,
        0x32 => &ld_dref_reg::ldd_hl_dref_a,

        0xEA => &ld_dref_reg::ld_a16_dref_a,

        // d16 arithmetic/logical
        0x03 => &d16_arithmetic_logical::inc_bc,
        0x13 => &d16_arithmetic_logical::inc_de,
        0x23 => &d16_arithmetic_logical::inc_hl,
        0x33 => &d16_arithmetic_logical::inc_sp,

        0x0B => &d16_arithmetic_logical::dec_bc,
        0x1B => &d16_arithmetic_logical::dec_de,
        0x2B => &d16_arithmetic_logical::dec_hl,
        0x3B => &d16_arithmetic_logical::dec_sp,

        0x09 => &d16_arithmetic_logical::add_hl_bc,
        0x19 => &d16_arithmetic_logical::add_hl_de,
        0x29 => &d16_arithmetic_logical::add_hl_hl,
        0x39 => &d16_arithmetic_logical::add_hl_sp,

        0xE8 => &d16_arithmetic_logical::add_sp_r8,

        // d8 arithmetic/logical
        0x80 => &d8_arithmetic_logical::add_a_b,
        0x81 => &d8_arithmetic_logical::add_a_c,
        0x82 => &d8_arithmetic_logical::add_a_d,
        0x83 => &d8_arithmetic_logical::add_a_e,
        0x84 => &d8_arithmetic_logical::add_a_h,
        0x85 => &d8_arithmetic_logical::add_a_l,
        0x86 => &d8_arithmetic_logical::add_a_hl_dref,
        0x87 => &d8_arithmetic_logical::add_a_a,
        0xC6 => &d8_arithmetic_logical::add_a_d8,

        0x90 => &d8_arithmetic_logical::sub_a_b,
        0x91 => &d8_arithmetic_logical::sub_a_c,
        0x92 => &d8_arithmetic_logical::sub_a_d,
        0x93 => &d8_arithmetic_logical::sub_a_e,
        0x94 => &d8_arithmetic_logical::sub_a_h,
        0x95 => &d8_arithmetic_logical::sub_a_l,
        0x96 => &d8_arithmetic_logical::sub_hl_dref,
        0x97 => &d8_arithmetic_logical::sub_a_a,
        0xD6 => &d8_arithmetic_logical::sub_d8,

        0xA0 => &d8_arithmetic_logical::and_b,
        0xA1 => &d8_arithmetic_logical::and_c,
        0xA2 => &d8_arithmetic_logical::and_d,
        0xA3 => &d8_arithmetic_logical::and_e,
        0xA4 => &d8_arithmetic_logical::and_h,
        0xA5 => &d8_arithmetic_logical::and_l,
        0xA6 => &d8_arithmetic_logical::and_hl_dref,
        0xA7 => &d8_arithmetic_logical::and_a,
        0xE6 => &d8_arithmetic_logical::and_d8,

        0xB0 => &d8_arithmetic_logical::or_b,
        0xB1 => &d8_arithmetic_logical::or_c,
        0xB2 => &d8_arithmetic_logical::or_d,
        0xB3 => &d8_arithmetic_logical::or_e,
        0xB4 => &d8_arithmetic_logical::or_h,
        0xB5 => &d8_arithmetic_logical::or_l,
        0xB6 => &d8_arithmetic_logical::or_hl_dref,
        0xB7 => &d8_arithmetic_logical::or_a,
        0xF6 => &d8_arithmetic_logical::or_d8,

        0x88 => &d8_arithmetic_logical::adc_a_b,
        0x89 => &d8_arithmetic_logical::adc_a_c,
        0x8A => &d8_arithmetic_logical::adc_a_d,
        0x8B => &d8_arithmetic_logical::adc_a_e,
        0x8C => &d8_arithmetic_logical::adc_a_h,
        0x8D => &d8_arithmetic_logical::adc_a_l,
        0x8E => &d8_arithmetic_logical::adc_a_hl_dref,
        0x8F => &d8_arithmetic_logical::adc_a_a,
        0xCE => &d8_arithmetic_logical::adc_a_d8,

        0x98 => &d8_arithmetic_logical::sbc_a_b,
        0x99 => &d8_arithmetic_logical::sbc_a_c,
        0x9A => &d8_arithmetic_logical::sbc_a_d,
        0x9B => &d8_arithmetic_logical::sbc_a_e,
        0x9C => &d8_arithmetic_logical::sbc_a_h,
        0x9D => &d8_arithmetic_logical::sbc_a_l,
        0x9E => &d8_arithmetic_logical::sbc_a_hl_dref,
        0x9F => &d8_arithmetic_logical::sbc_a_a,
        0xDE => &d8_arithmetic_logical::sbc_a_d8,

        0xA8 => &d8_arithmetic_logical::xor_b,
        0xA9 => &d8_arithmetic_logical::xor_c,
        0xAA => &d8_arithmetic_logical::xor_d,
        0xAB => &d8_arithmetic_logical::xor_e,
        0xAC => &d8_arithmetic_logical::xor_h,
        0xAD => &d8_arithmetic_logical::xor_l,
        0xAE => &d8_arithmetic_logical::xor_hl_dref,
        0xAF => &d8_arithmetic_logical::xor_a,
        0xEE => &d8_arithmetic_logical::xor_d8,

        0xB8 => &d8_arithmetic_logical::cp_b,
        0xB9 => &d8_arithmetic_logical::cp_c,
        0xBA => &d8_arithmetic_logical::cp_d,
        0xBB => &d8_arithmetic_logical::cp_e,
        0xBC => &d8_arithmetic_logical::cp_h,
        0xBD => &d8_arithmetic_logical::cp_l,
        0xBE => &d8_arithmetic_logical::cp_hl_dref,
        0xBF => &d8_arithmetic_logical::cp_a,
        0xFE => &d8_arithmetic_logical::cp_d8,

        0x04 => &d8_arithmetic_logical::inc_b,
        0x0C => &d8_arithmetic_logical::inc_c,
        0x14 => &d8_arithmetic_logical::inc_d,
        0x1C => &d8_arithmetic_logical::inc_e,
        0x24 => &d8_arithmetic_logical::inc_h,
        0x2C => &d8_arithmetic_logical::inc_l,
        0x34 => &d8_arithmetic_logical::inc_hl_dref,
        0x3C => &d8_arithmetic_logical::inc_a,

        0x05 => &d8_arithmetic_logical::dec_b,
        0x0D => &d8_arithmetic_logical::dec_c,
        0x15 => &d8_arithmetic_logical::dec_d,
        0x1D => &d8_arithmetic_logical::dec_e,
        0x25 => &d8_arithmetic_logical::dec_h,
        0x2D => &d8_arithmetic_logical::dec_l,
        0x35 => &d8_arithmetic_logical::dec_hl_dref,
        0x3D => &d8_arithmetic_logical::dec_a,

        0x2F => &d8_arithmetic_logical::cpl,

        0x37 => &d8_arithmetic_logical::scf,

        0x3F => &d8_arithmetic_logical::ccf,

        // ld reg d16.
        0x01 => &ld_reg_d16::ld_bc_d16,
        0x11 => &ld_reg_d16::ld_de_d16,
        0x21 => &ld_reg_d16::ld_hl_d16,
        0x31 => &ld_reg_d16::ld_sp_d16,

        // 16 bit load.
        0xF9 => &load_16_bit::ld_sp_hl,
        0x08 => &load_16_bit::ld_a16_sp,

        _ => &unimplement_op_fn,
    }
}
