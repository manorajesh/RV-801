use crate::cpu::{Interface, CPU};

#[cfg(test)]
mod tests {
    use crate::init_cpu_test;

    use super::*;

    #[test]
    fn addi() {
        let mut cpu = init_cpu_test();
        cpu.from_inst(vec![
            0x06408093, 0x00a08113, 0xfff10193, 0x7ff20213, 0x80020293, 0x80130313, 0x80130313,
        ]);
        cpu.run();

        assert_eq!(cpu.regs[1], 100);
        assert_eq!(cpu.regs[2], 110);
        assert_eq!(cpu.regs[3], 109);
        assert_eq!(cpu.regs[4], 2047);
        assert_eq!(cpu.regs[5] as i32, -1); // -1
        assert_eq!(cpu.regs[6] as i32, -4094); // 2047
    }

    #[test]
    fn slti() {
        let mut cpu = init_cpu_test();
        cpu.from_inst(vec![0x06402093, 0xfff02113]);

        cpu.run();

        assert_eq!(cpu.regs[1], 1);
        assert_eq!(cpu.regs[2], 0);
    }

    #[test]
    fn sltiu() {
        let mut cpu = init_cpu_test();
        cpu.from_inst(vec![0x06403093, 0xfff03113]);

        cpu.run();

        assert_eq!(cpu.regs[1], 1);
        assert_eq!(cpu.regs[2], 1);
    }
}
