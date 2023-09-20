use std::fs;

use crate::isa::Instruction;

pub struct CPU {
    regs: [u32; 32],
    pc: usize,
    memory: [u8; 0x10000],
    pub exit_on_nop: bool,
    pub last_inst: Option<Instruction>,
}

pub trait Interface {
    fn load(&mut self, instructions: &[u8]);

    fn run(&mut self) -> u8;

    fn boot(&mut self, path: &str) -> u8 {
        let instructions = fs::read(path).expect("Unable to read file");
        self.load(&instructions);
        self.run()
    }

    fn from_inst(&mut self, instruction: u32) {
        let bytes = instruction.to_le_bytes();
        self.load(&bytes);
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            regs: [0; 32],
            pc: 0,
            memory: [0; 0x10000],
            exit_on_nop: false,
            last_inst: None,
        }
    }

    fn fetch(&mut self) -> u32 {
        let inst = u32::from_le_bytes([
            self.memory[self.pc],
            self.memory[self.pc + 1],
            self.memory[self.pc + 2],
            self.memory[self.pc + 3],
        ]);
        self.pc += 4;
        inst
    }

    fn decode(&self, inst: u32) -> Instruction {
        Instruction::from(inst)
    }

    // for now, just print the instruction
    fn execute(&mut self, inst: Instruction) -> u8 {
        println!("{:?}", inst);
        0
    }
}

impl Interface for CPU {
    fn load(&mut self, instructions: &[u8]) {
        for (i, inst) in instructions.iter().enumerate() {
            self.memory[i + self.pc] = *inst;
        }
    }

    fn run(&mut self) -> u8 {
        loop {
            let inst = self.fetch();
            let inst = self.decode(inst);
            self.execute(inst);
            self.last_inst = Some(inst);
            if self.exit_on_nop && inst.is_nop() {
                return 0;
            }
        }
    }
}
