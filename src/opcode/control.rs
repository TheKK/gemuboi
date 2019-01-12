use crate::cpu::Cpu;
use crate::opcode::table::{Cycle, OpLength};

pub fn nop(_: &mut Cpu) -> (Cycle, OpLength) {
    (Cycle(4), OpLength(1))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_nop() {
        let cpu = Cpu::default();

        let mut new_cpu = cpu.clone();
        nop(&mut new_cpu);

        assert!(cpu == new_cpu);
    }
}
