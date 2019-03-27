use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    OutOfBound,
}

pub type Result<T> = std::result::Result<T, Error>;
pub type Addr = u16;

pub const MEM_SIZE: u16 = 0xFFFF;
pub const INVALID_READ_DEFAULT_VALUE: u8 = 0;

pub const INVALID_MEM_ACCESS_EXPECT: &str = "Invalid address access";

#[derive(Clone)]
pub struct Mmu {
    memory: [u8; MEM_SIZE as usize],
}

impl Default for Mmu {
    fn default() -> Self {
        Self {
            memory: [0; MEM_SIZE as usize],
        }
    }
}

impl fmt::Debug for Mmu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Mmu")
            .field("memory", &self.memory.to_vec())
            .finish()
    }
}

impl PartialEq for Mmu {
    fn eq(&self, rhs: &Self) -> bool {
        self.memory[..] == rhs.memory[..]
    }
}

impl Mmu {
    #[inline]
    pub fn read_byte(&self, addr: Addr) -> u8 {
        let addr = addr as usize;

        self.memory
            .get(addr)
            .cloned()
            .unwrap_or(INVALID_READ_DEFAULT_VALUE)
    }

    #[inline]
    pub fn read_word(&self, addr: Addr) -> u16 {
        let addr = addr as usize;

        // TODO addr + 1 is possible overflowing.
        let h = u16::from(*self.memory.get(addr).expect(INVALID_MEM_ACCESS_EXPECT));
        let l = u16::from(*self.memory.get(addr + 1).expect(INVALID_MEM_ACCESS_EXPECT));

        (h << 8) + l
    }

    #[inline]
    pub fn write_byte(&mut self, addr: Addr, value: u8) -> Result<()> {
        let addr = addr as usize;

        let byte = self.memory.get_mut(addr).ok_or(Error::OutOfBound)?;
        *byte = value;

        Ok(())
    }

    #[inline]
    pub fn write_word(&mut self, addr: Addr, value: u16) -> Result<()> {
        let addr = addr as usize;

        self.memory.get(addr + 1).ok_or(Error::OutOfBound)?;

        self.memory[addr] = ((value & 0xff00) >> 8) as u8;
        self.memory[addr + 1] = (value & 0x00ff) as u8;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_byte_with_incorrect_address() {
        let mmu = Mmu::default();

        assert_eq!(mmu.read_byte(0xFF), INVALID_READ_DEFAULT_VALUE);
    }

    #[test]
    fn write_byte_with_correct_address() {
        let test = |addr| {
            let mut mmu = Mmu::default();

            let value = 42;

            assert!(mmu.write_byte(addr, value).is_ok());
            assert_eq!(mmu.read_byte(addr), value);
        };

        test(0x00);
        test(0x42);
        test(MEM_SIZE - 1);
    }

    #[test]
    fn write_byte_with_incorrect_address() {
        let test = |addr| {
            let mut mmu = Mmu::default();

            let value = 42;

            assert_eq!(mmu.write_byte(addr, value), Err(Error::OutOfBound));
        };

        test(MEM_SIZE);
    }

    #[test]
    fn write_word_with_correct_address() {
        let test = |addr| {
            let mut mmu = Mmu::default();

            let value = 1024;

            assert!(mmu.write_word(addr, value).is_ok());
            assert_eq!(mmu.read_word(addr), value);
        };

        test(0x00);
        test(0x42);
        test(MEM_SIZE - 2);
    }

    #[test]
    fn write_word_with_incorrect_address() {
        let test = |addr| {
            let mut mmu = Mmu::default();

            let value = 42;

            assert_eq!(mmu.write_word(addr, value), Err(Error::OutOfBound));
        };

        test(MEM_SIZE - 1);
        test(MEM_SIZE);
    }
}
