use std::default::Default;

/// Magic flag
pub struct Flag {
  zero: bool,
  add_sub: bool,
  half_carry: bool,
  carry: bool,
}

impl Default for Flag {
  fn default() -> Self {
    Self {
      zero: false,
      add_sub: false,
      half_carry: false,
      carry: false,
    }
  }
}

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

  f: Flag,
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

      f: Flag::default(),
    }
  }
}

macro_rules! getter_and_setter {
  (8bits $([$reg:ident, $setter:ident]),*) => {
    $(
      fn $reg (&self) -> u8 {
        self.$reg
      }

      fn $setter (&mut self, val: u8) {
        self.$reg = val;
      }
    )*
  };

  (16bits $([$reg:ident, $setter:ident]),*) => {
    $(
      fn $reg (&self) -> u16 {
        self.$reg
      }

      fn $setter (&mut self, val: u16) {
        self.$reg = val;
      }
    )*
  };

  (16bits $([$regL:ident + $regH:ident, $getter:ident, $setter:ident]),*) => {
    $(
      fn $getter (&self) -> u16 {
        let l = (self.$regL as u16) << 8;
        let h = (self.$regH as u16) << 0;

        l + h
      }

      fn $setter(&mut self, val: u16) {
        let l = ((val & 0xff00) >> 8) as u8;
        let h = ((val & 0x00ff) >> 0) as u8;

        self.$regL = l;
        self.$regH = h;
      }
    )*
  };
}

macro_rules! flag_getter_and_setter {
  ($([$flag:ident, $setter:ident]),*) => {
    $(
      fn $flag (&self) -> bool {
        self.f.$flag
      }

      fn $setter (&mut self, val: bool) {
        self.f.$flag = val;
      }
    )*
  };
}

impl Registers {
  getter_and_setter![
    8bits
    [a, set_a],
    [b, set_b],
    [c, set_c],
    [d, set_d],
    [e, set_e],
    [h, set_h],
    [l, set_l]
  ];

  getter_and_setter![
    16bits
    [sp, set_sp],
    [pc, set_pc]
  ];

  getter_and_setter![
    16bits
    [b + c, bc, set_bc],
    [d + e, de, set_de],
    [h + l, hl, set_hl]
  ];

  flag_getter_and_setter![
    [zero, set_zero],
    [add_sub, set_add_sub],
    [half_carry, set_half_carry],
    [carry, set_carry]
  ];
}

#[cfg(test)]
mod test {
  use super::*;

  type RegisterSetter8 = &'static Fn(&mut Registers, u8);
  type RegisterGetter8 = &'static Fn(&Registers) -> u8;
  type RegisterSetter16 = &'static Fn(&mut Registers, u16);
  type RegisterGetter16 = &'static Fn(&Registers) -> u16;

  fn test_u16_read(set_fn: RegisterSetter16, get_l_fn: RegisterGetter8, get_h_fn: RegisterGetter8) {
    let mut registers = Registers::default();

    let l = 0x1200;
    let h = 0x0023;

    set_fn(&mut registers, l + h);

    assert_eq!(get_l_fn(&registers), (l >> 8) as u8);
    assert_eq!(get_h_fn(&registers), (h >> 0) as u8);
  }

  fn test_u16_write(l_set_fn: RegisterSetter8, h_set_fn: RegisterSetter8, get_fn: RegisterGetter16) {
    let mut registers = Registers::default();

    let l = 0x12;
    let h = 0x23;

    l_set_fn(&mut registers, l);
    h_set_fn(&mut registers, h);

    let sum = ((l as u16) << 8) + h as u16;

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
