use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};
use crate::registers::Registers;

use super::ld_utils::{
    ld, load_byte_from_reg_dref, read_byte_from_pc_offset, read_word_from_pc_offset, store_to_reg,
};

macro_rules! ld_reg_dref_fn {
    ($fn_name:ident, ($addr_reg:ident) > $store_to:ident) => {
        pub fn $fn_name(cpu: &mut Cpu) -> (Cycle, OpLength) {
            ld(
                cpu,
                &load_byte_from_reg_dref(&Registers::$addr_reg),
                &store_to_reg(&Registers::$store_to),
            );

            (Cycle(8), OpLength(1))
        }
    };
}

ld_reg_dref_fn!(ld_a_bc_dref, (bc) > set_a);
ld_reg_dref_fn!(ld_a_de_dref, (de) > set_a);

ld_reg_dref_fn!(ld_a_hl_dref, (hl) > set_a);
ld_reg_dref_fn!(ld_b_hl_dref, (hl) > set_b);
ld_reg_dref_fn!(ld_c_hl_dref, (hl) > set_c);
ld_reg_dref_fn!(ld_d_hl_dref, (hl) > set_d);
ld_reg_dref_fn!(ld_e_hl_dref, (hl) > set_e);
ld_reg_dref_fn!(ld_h_hl_dref, (hl) > set_h);
ld_reg_dref_fn!(ld_l_hl_dref, (hl) > set_l);

#[inline]
pub fn ldi_a_hl_dref(cpu: &mut Cpu) -> (Cycle, OpLength) {
    ld(
        cpu,
        &load_byte_from_reg_dref(&Registers::hl),
        &store_to_reg(&Registers::set_a),
    );

    let hl = cpu.registers.hl();
    cpu.registers.set_hl(hl + 1);

    (Cycle(8), OpLength(1))
}

#[inline]
pub fn ldd_a_hl_dref(cpu: &mut Cpu) -> (Cycle, OpLength) {
    ld(
        cpu,
        &load_byte_from_reg_dref(&Registers::hl),
        &store_to_reg(&Registers::set_a),
    );

    let hl = cpu.registers.hl();
    cpu.registers.set_hl(hl - 1);

    (Cycle(8), OpLength(1))
}

#[inline]
pub fn ldh_a_a8_dref(cpu: &mut Cpu) -> (Cycle, OpLength) {
    ld(
        cpu,
        &|cpu| {
            let addr = 0xFF00 + u16::from(read_byte_from_pc_offset(1)(cpu).unwrap());
            cpu.mmu.read_byte(addr)
        },
        &store_to_reg(&Registers::set_a),
    );

    (Cycle(12), OpLength(2))
}

#[inline]
pub fn ld_a_a16_dref(cpu: &mut Cpu) -> (Cycle, OpLength) {
    ld(
        cpu,
        &|cpu| {
            let addr = read_word_from_pc_offset(1)(cpu).unwrap();
            cpu.mmu.read_byte(addr)
        },
        &store_to_reg(&Registers::set_a),
    );

    (Cycle(16), OpLength(3))
}

#[inline]
pub fn ld_a_c_dref(cpu: &mut Cpu) -> (Cycle, OpLength) {
    ld(
        cpu,
        &|cpu| {
            let addr = 0xFF00 + u16::from(cpu.registers.c());
            cpu.mmu.read_byte(addr)
        },
        &store_to_reg(&Registers::set_a),
    );

    // TODO length 2?
    (Cycle(8), OpLength(2))
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::cpu::Cpu;

    macro_rules! ld_reg_dref_test {
        ($test_name:ident, $ins_to_test:ident, $addr_reg_setter:ident, $reg_getter:ident, $reg_setter:ident) => {
            #[test]
            fn $test_name() {
                // Arrange: prepare cpu.
                let the_addr = 0x0101;
                let the_value = 0x42;

                let mut init_cpu = Cpu::default();
                init_cpu.mmu.write_byte(the_addr, the_value).unwrap();
                init_cpu.registers.$addr_reg_setter(the_addr as u16);

                let mut modified_cpu = init_cpu.clone();

                // Action.
                $ins_to_test(&mut modified_cpu);

                // Assert: check register value.
                let actual_value = modified_cpu.registers.$reg_getter();
                assert_eq!(actual_value, the_value);

                // Assert: other state.
                init_cpu.registers.$reg_setter(the_value);
                assert!(init_cpu == modified_cpu);
            }
        };
    }

    ld_reg_dref_test!(run_ld_a_bc_dref, ld_a_bc_dref, set_bc, a, set_a);
    ld_reg_dref_test!(run_ld_a_de_dref, ld_a_de_dref, set_de, a, set_a);

    ld_reg_dref_test!(run_ld_a_hl_dref, ld_a_hl_dref, set_hl, a, set_a);
    ld_reg_dref_test!(run_ld_b_hl_dref, ld_b_hl_dref, set_hl, b, set_b);
    ld_reg_dref_test!(run_ld_c_hl_dref, ld_c_hl_dref, set_hl, c, set_c);
    ld_reg_dref_test!(run_ld_d_hl_dref, ld_d_hl_dref, set_hl, d, set_d);
    ld_reg_dref_test!(run_ld_e_hl_dref, ld_e_hl_dref, set_hl, e, set_e);
    ld_reg_dref_test!(run_ld_h_hl_dref, ld_h_hl_dref, set_hl, h, set_h);
    ld_reg_dref_test!(run_ld_l_hl_dref, ld_l_hl_dref, set_hl, l, set_l);

    #[test]
    fn run_ldi_a_hl_dref() {
        // Arrange: prepare cpu.
        let the_addr = 0x0101;
        let the_value = 0x42;

        let mut init_cpu = Cpu::default();
        init_cpu.mmu.write_byte(the_addr, the_value).unwrap();
        init_cpu.registers.set_hl(the_addr as u16);

        let mut modified_cpu = init_cpu.clone();

        // Action.
        ldi_a_hl_dref(&mut modified_cpu);

        // Assert: check register value.
        let actual_value = modified_cpu.registers.a();
        assert_eq!(actual_value, the_value);

        // Assert: other state.
        init_cpu.registers.set_a(the_value);
        init_cpu.registers.set_hl((the_addr + 1) as u16);
        assert!(init_cpu == modified_cpu);
    }

    #[test]
    fn run_ldd_a_hl_dref() {
        // Arrange: prepare cpu.
        let the_addr = 0x0101;
        let the_value = 0x42;

        let mut init_cpu = Cpu::default();
        init_cpu.mmu.write_byte(the_addr, the_value).unwrap();
        init_cpu.registers.set_hl(the_addr as u16);

        let mut modified_cpu = init_cpu.clone();

        // Action.
        ldd_a_hl_dref(&mut modified_cpu);

        // Assert: check register value.
        let actual_value = modified_cpu.registers.a();
        assert_eq!(actual_value, the_value);

        // Assert: other state.
        init_cpu.registers.set_a(the_value);
        init_cpu.registers.set_hl((the_addr - 1) as u16);
        assert!(init_cpu == modified_cpu);
    }

    #[test]
    fn run_ldh_a_a8_dref() {
        // Arrange: prepare cpu.
        let the_pc = 0x00;

        let the_higher_addr: u16 = 0xFF00;
        let the_lower_addr: u8 = 0x09;
        let the_addr = the_higher_addr + (the_lower_addr as u16);

        let the_value = 0x42;

        let mut init_cpu = Cpu::default();
        init_cpu.registers.set_pc(the_pc);
        init_cpu.mmu.write_byte(0x00, 0xf0).unwrap();
        init_cpu.mmu.write_byte(0x01, the_lower_addr).unwrap();
        init_cpu.mmu.write_byte(the_addr, the_value).unwrap();

        let mut modified_cpu = init_cpu.clone();

        // Action.
        ldh_a_a8_dref(&mut modified_cpu);

        // Assert: check register value.
        let actual_value = modified_cpu.registers.a();
        assert_eq!(actual_value, the_value);

        // Assert: other state.
        init_cpu.registers.set_a(the_value);
        assert!(init_cpu == modified_cpu);
    }

    #[test]
    fn run_ld_a_a16_dref() {
        // Arrange: prepare cpu.
        let the_pc = 0x00;

        let the_higher_addr = 0x33;
        let the_lower_addr = 0x09;
        let the_addr = ((the_higher_addr as u16) << 8) + (the_lower_addr as u16);

        let the_value = 0x42;

        let mut init_cpu = Cpu::default();
        init_cpu.registers.set_pc(the_pc);
        init_cpu.mmu.write_byte(0x00, 0xf0).unwrap();
        init_cpu.mmu.write_byte(0x01, the_higher_addr).unwrap();
        init_cpu.mmu.write_byte(0x02, the_lower_addr).unwrap();
        init_cpu.mmu.write_byte(the_addr, the_value).unwrap();

        let mut modified_cpu = init_cpu.clone();

        // Action.
        ld_a_a16_dref(&mut modified_cpu);

        // Assert: check register value.
        let actual_value = modified_cpu.registers.a();
        assert_eq!(actual_value, the_value);

        // Assert: other state.
        init_cpu.registers.set_a(the_value);
        assert!(init_cpu == modified_cpu);
    }

    #[test]
    fn run_ld_a_c_dref() {
        // Arrange: prepare cpu.
        let the_pc = 0x00;

        let the_higher_addr: u16 = 0xFF00;
        let the_lower_addr: u8 = 0x09;
        let the_addr = the_higher_addr + (the_lower_addr as u16);

        let the_value = 0x42;

        let mut init_cpu = Cpu::default();
        init_cpu.registers.set_pc(the_pc);
        init_cpu.registers.set_c(the_lower_addr);
        init_cpu.mmu.write_byte(0x00, 0xf0).unwrap();
        init_cpu.mmu.write_byte(the_addr, the_value).unwrap();

        let mut modified_cpu = init_cpu.clone();

        // Action.
        ld_a_c_dref(&mut modified_cpu);

        // Assert: check register value.
        let actual_value = modified_cpu.registers.a();
        assert_eq!(actual_value, the_value);

        // Assert: other state.
        init_cpu.registers.set_a(the_value);
        assert!(init_cpu == modified_cpu);
    }
}
