use std::default::Default;
use std::ops::Deref;

macro_rules! register_getter_and_setter {
  (8bits $([$reg:ident, $setter:ident]),*) => {
    $(
      pub fn $reg (&self) -> u8 {
        self.$reg
      }

      pub fn $setter (&mut self, val: u8) {
        self.$reg = val;
      }
    )*
  };

  (16bits $([$reg:ident, $setter:ident]),*) => {
    $(
      pub fn $reg (&self) -> u16 {
        self.$reg
      }

      pub fn $setter (&mut self, val: u16) {
        self.$reg = val;
      }
    )*
  };

  (16bits $([$regH:ident + $regL:ident, $getter:ident, $setter:ident]),*) => {
    $(
      pub fn $getter (&self) -> u16 {
        let h = u16::from(self.$regH) << 8;
        let l = u16::from(self.$regL) << 0;

        h + l
      }

      pub fn $setter(&mut self, val: u16) {
        let h = ((val & 0xFF00) >> 8) as u8;
        let l = ((val & 0x00FF) >> 0) as u8;

        self.$regH = h;
        self.$regL = l;
      }
    )*
  };
}

macro_rules! flag_getter_and_setter {
  ($([$flag:ident, $setter:ident]),*) => {
    $(
      pub fn $flag (&self) -> bool {
        self.$flag
      }

      pub fn $setter (&mut self, val: bool) {
        self.$flag = val;
      }
    )*
  };
}

/// Magic flag
#[derive(Debug, PartialEq, Clone)]
pub struct Flag {
    zero: bool,
    sub: bool,
    half_carry: bool,
    carry: bool,
}

impl Default for Flag {
    fn default() -> Self {
        Self {
            zero: false,
            sub: false,
            half_carry: false,
            carry: false,
        }
    }
}

impl Deref for Flag {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &0
    }
}

impl From<u8> for Flag {
    fn from(val: u8) -> Flag {
        Flag {
            zero: val & 128 > 0,
            sub: val & 64 > 0,
            half_carry: val & 32 > 0,
            carry: val & 16 > 0,
        }
    }
}

impl Flag {
    pub fn new(zero: bool, sub: bool, half_carry: bool, carry: bool) -> Self {
        Self {
            zero,
            sub,
            half_carry,
            carry,
        }
    }

    flag_getter_and_setter![
        [zero, set_zero],
        [sub, set_sub],
        [half_carry, set_half_carry],
        [carry, set_carry]
    ];
}

#[derive(Debug, PartialEq, Clone)]
pub struct Registers {
    a: u8,

    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,

    sp: u16,
    pc: u16,

    pub flag: Flag,
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            a: 0,

            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,

            sp: 0,
            pc: 0,

            flag: Flag::default(),
        }
    }
}

impl Registers {
    register_getter_and_setter![
    8bits
    [a, set_a],
    [b, set_b],
    [c, set_c],
    [d, set_d],
    [e, set_e],
    [h, set_h],
    [l, set_l]
  ];

    register_getter_and_setter![
    16bits
    [sp, set_sp],
    [pc, set_pc]
  ];

    register_getter_and_setter![
    16bits
    [b + c, bc, set_bc],
    [d + e, de, set_de],
    [h + l, hl, set_hl]
  ];
}

impl Registers {
    pub fn af(&self) -> u16 {
        let h = u16::from(self.a) << 8;
        let l = u16::from(*self.flag);

        h + l
    }

    pub fn set_af(&mut self, val: u16) {
        let h = ((val & 0xFF00) >> 8) as u8;
        let l = (val & 0x00FF) as u8;

        self.a = h;
        self.flag = l.into();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type RegisterSetter8 = &'static Fn(&mut Registers, u8);
    type RegisterGetter8 = &'static Fn(&Registers) -> u8;
    type RegisterSetter16 = &'static Fn(&mut Registers, u16);
    type RegisterGetter16 = &'static Fn(&Registers) -> u16;

    fn test_u16_read(
        set_fn: RegisterSetter16,
        get_h_fn: RegisterGetter8,
        get_l_fn: RegisterGetter8,
    ) {
        let mut registers = Registers::default();

        let h = 0x1200;
        let l = 0x0023;

        set_fn(&mut registers, h + l);

        assert_eq!(get_h_fn(&registers), (h >> 8) as u8);
        assert_eq!(get_l_fn(&registers), (l >> 0) as u8);
    }

    fn test_u16_write(
        h_set_fn: RegisterSetter8,
        l_set_fn: RegisterSetter8,
        get_fn: RegisterGetter16,
    ) {
        let mut registers = Registers::default();

        let h = 0x12;
        let l = 0x23;

        h_set_fn(&mut registers, h);
        l_set_fn(&mut registers, l);

        let sum = ((h as u16) << 8) + l as u16;

        assert_eq!(get_fn(&registers), sum);
    }

    #[test]
    fn bc_read() {
        test_u16_read(&Registers::set_bc, &Registers::b, &Registers::c);
    }

    #[test]
    fn bc_write() {
        test_u16_write(&Registers::set_b, &Registers::set_c, &Registers::bc)
    }

    #[test]
    fn de_read() {
        test_u16_read(&Registers::set_de, &Registers::d, &Registers::e);
    }

    #[test]
    fn de_write() {
        test_u16_write(&Registers::set_d, &Registers::set_e, &Registers::de)
    }

    #[test]
    fn hl_read() {
        test_u16_read(&Registers::set_hl, &Registers::h, &Registers::l);
    }

    #[test]
    fn hl_write() {
        test_u16_write(&Registers::set_h, &Registers::set_l, &Registers::hl)
    }
}
