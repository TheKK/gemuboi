#[derive(Debug, PartialEq)]
pub enum Error {
    OutOfBound,
}

pub type Result<T> = std::result::Result<T, Error>;

pub const MEM_SIZE: usize = 1024;

#[derive(Clone)]
pub struct Mmu {
    memory: [u8; MEM_SIZE],
}

impl Default for Mmu {
    fn default() -> Self {
        Self { memory: [0; 1024] }
    }
}

impl PartialEq for Mmu {
    fn eq(&self, rhs: &Self) -> bool {
        self.memory[..] == rhs.memory[..]
    }
}

impl Mmu {
    #[inline]
    pub fn read_byte(&self, addr: usize) -> Result<u8> {
        self.memory.get(addr).cloned().ok_or(Error::OutOfBound)
    }

    #[inline]
    pub fn read_word(&self, addr: usize) -> Result<u16> {
        let l = u16::from(self.memory.get(addr).cloned().ok_or(Error::OutOfBound)?);
        let h = u16::from(
            self.memory
                .get(addr + 1)
                .cloned()
                .ok_or(Error::OutOfBound)?,
        );

        Ok((l << 8) + h)
    }

    #[inline]
    pub fn write_byte(&mut self, addr: usize, value: u8) -> Result<()> {
        let byte = self.memory.get_mut(addr).ok_or(Error::OutOfBound)?;
        *byte = value;

        Ok(())
    }

    #[inline]
    pub fn write_word(&mut self, addr: usize, value: u16) -> Result<()> {
        self.memory.get(addr).ok_or(Error::OutOfBound)?;
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
    fn write_byte_with_correct_address() {
        let test = |addr| {
            let mut mmu = Mmu::default();

            let value = 42;

            assert!(mmu.write_byte(addr, value).is_ok());
            assert_eq!(mmu.read_byte(addr).unwrap(), value);
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
        test(MEM_SIZE + 1);
        test(MEM_SIZE + 10);
    }

    #[test]
    fn write_word_with_correct_address() {
        let test = |addr| {
            let mut mmu = Mmu::default();

            let value = 1024;

            assert!(mmu.write_word(addr, value).is_ok());
            assert_eq!(mmu.read_word(addr).unwrap(), value);
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
        test(MEM_SIZE + 1);
        test(MEM_SIZE + 2);
        test(MEM_SIZE + 10);
    }
}
