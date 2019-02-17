pub struct CarryTestResult<T> {
    pub val: T,
    pub half_carry: bool,
    pub carry: bool,
}

pub trait CarryTest: Sized {
    fn carry_add(self, rhs: Self) -> CarryTestResult<Self>;
    fn carry_sub(self, rhs: Self) -> CarryTestResult<Self>;
}

impl CarryTest for u8 {
    fn carry_add(self, rhs: Self) -> CarryTestResult<Self> {
        let (val, carry) = self.overflowing_add(rhs);
        let (_, half_carry) = (self << 4).overflowing_add(rhs << 4);

        CarryTestResult {
            val,
            half_carry,
            carry,
        }
    }

    fn carry_sub(self, rhs: Self) -> CarryTestResult<Self> {
        let (val, carry) = self.overflowing_sub(rhs);
        let (_, half_carry) = (self & 0xF).overflowing_sub(rhs & 0xF);

        CarryTestResult {
            val,
            half_carry,
            carry,
        }
    }
}
