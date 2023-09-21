use cpu::Interface;

mod cpu;
mod isa;

fn main() {
    let mut cpu = cpu::CPU::new();
    cpu.from_inst(0x00A30213);
    cpu.exit_on_nop = true;
    cpu.run();

    cpu.print_state();
}
