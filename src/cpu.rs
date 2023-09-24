use std::fs;

use crate::isa::{Instruction, InstructionType, RV32I};

pub struct CPU {
    pub regs: [u32; 32],
    pub pc: usize,
    memory: [u8; 0x10000],
    pub exit_on_nop: bool,
    pub last_inst: Option<Instruction>,
}

trait RV32ISA {
    // Add Immediate: Adds an immediate value to rs1 and stores the result in rd.
    fn addi(&mut self, rd: u8, rs1: u8, imm: i16);
}

pub trait Interface {
    fn load(&mut self, instructions: &[u8]);

    fn run(&mut self) -> u8;

    fn boot(&mut self, path: &str, radix: u8) -> u8 {
        let instructions_str = fs::read_to_string(path).expect("Unable to read file");
        let mut instructions_bytes = Vec::new();

        for line in instructions_str.lines() {
            let instruction = u32::from_str_radix(line, radix as u32).expect("Invalid number");
            let bytes = instruction.to_le_bytes();
            instructions_bytes.extend_from_slice(&bytes);
        }

        self.load(&instructions_bytes);
        self.run()
    }

    fn from_inst(&mut self, instruction: Vec<u32>) {
        let bytes = instruction
            .iter()
            .flat_map(|inst| inst.to_le_bytes().to_vec())
            .collect::<Vec<u8>>();

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

    pub fn print_state(&self) {
        println!("PC:  0x{:08X}", self.pc);
        for (i, reg) in self.regs.iter().enumerate() {
            println!("x{:02}: 0x{:08}", i, reg);
        }
    }

    fn execute(&mut self, inst: Instruction) -> u8 {
        match inst.inst {
            RV32I::ADDI => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for ADDI")
                };

                self.addi(args.rd, args.rs1, args.imm);
            }

            _ => {
                println!("Unimplemented instruction: {:?}", inst);
                return 1;
            }
        }

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

impl RV32ISA for CPU {
    fn addi(&mut self, rd: u8, rs1: u8, imm: i16) {
        self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_add(imm as u32);
    }
}

trait TwelveBitWrap {
    fn wrapping_12bit_add(&self, rhs: Self) -> i16;
}

impl TwelveBitWrap for i16 {
    fn wrapping_12bit_add(&self, rhs: Self) -> i16 {
        let result = self.wrapping_add(rhs) & 0xFFF;

        if result >= 2048 {
            return -2048;
        }
        result
    }
}
