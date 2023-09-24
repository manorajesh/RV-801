use cpu::Interface;

mod cpu;
mod isa;
mod tests;

fn main() {
    let mut cpu = cpu::CPU::new();
    cpu.exit_on_nop = true;
    // cpu.boot("tests/test.bin", 16);
    cpu.from_inst(vec![0x3e800093, 0x06308113, 0x40208133]);
    cpu.run();

    cpu.print_state();
}
