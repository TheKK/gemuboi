#[inline]
fn rlc(input: u8) -> (u8, bool) {
    let carry = 0b1000_0000 & input != 0;

    (input.rotate_left(1), carry)
}

#[cfg(test)]
mod test {
    pub use super::*;

    #[test]
    fn run_rlc() {
        assert_eq!(rlc(0b01010101), (0b10101010, false));
        assert_eq!(rlc(0b11111110), (0b11111101, true));
    }
}
