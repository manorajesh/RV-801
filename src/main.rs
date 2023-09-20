use cpu::Interface;

mod cpu;
mod isa;

fn main() {
    let mut cpu = cpu::CPU::new();
    cpu.from_inst(0x00000013);
    cpu.exit_on_nop = true;
    cpu.run();

    println!("Last instruction: 0x{:08x}", cpu.last_inst.unwrap().raw);
}
