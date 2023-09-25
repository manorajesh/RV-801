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
    fn addi(&mut self, rd: u8, rs1: u8, imm: u32);

    // Set Less Than Immediate: If rs1 is less than the immediate value, set rd to 1, otherwise set rd to 0.
    fn slti(&mut self, rd: u8, rs1: u8, imm: u32);

    // Set Less Than Immediate Unsigned: If rs1 is less than the immediate value, set rd to 1, otherwise set rd to 0.
    fn sltiu(&mut self, rd: u8, rs1: u8, imm: u32);

    // XOR Immediate: Bitwise XOR rs1 and the immediate value and store the result in rd.
    fn xori(&mut self, rd: u8, rs1: u8, imm: u32);

    // OR Immediate: Bitwise OR rs1 and the immediate value and store the result in rd.
    fn ori(&mut self, rd: u8, rs1: u8, imm: u32);

    // AND Immediate: Bitwise AND rs1 and the immediate value and store the result in rd.
    fn andi(&mut self, rd: u8, rs1: u8, imm: u32);

    // Shift Left Logical Immediate: Shift rs1 left by the immediate value and store the result in rd.
    fn slli(&mut self, rd: u8, rs1: u8, imm: u32);

    // Shift Right Logical Immediate: Shift rs1 right by the immediate value and store the result in rd.
    fn srli(&mut self, rd: u8, rs1: u8, imm: u32);

    // Shift Right Arithmetic Immediate: Shift rs1 right by the immediate value and store the result in rd. The sign bit is preserved.
    fn srai(&mut self, rd: u8, rs1: u8, imm: u32);
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

    fn execute(&mut self, inst: Instruction) -> Result<u8, String> {
        match inst.inst {
            RV32I::ADDI => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for ADDI")
                };

                self.addi(args.rd, args.rs1, args.imm);
            }

            RV32I::SLTI => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for SLTI")
                };

                self.slti(args.rd, args.rs1, args.imm);
            }

            RV32I::SLTIU => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for SLTIU")
                };

                self.sltiu(args.rd, args.rs1, args.imm);
            }

            RV32I::XORI => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for XORI")
                };

                self.xori(args.rd, args.rs1, args.imm);
            }

            RV32I::ORI => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for ORI")
                };

                self.ori(args.rd, args.rs1, args.imm);
            }

            RV32I::ANDI => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for ANDI")
                };

                self.andi(args.rd, args.rs1, args.imm);
            }

            RV32I::SLLI => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for SLLI")
                };

                self.slli(args.rd, args.rs1, args.imm);
            }

            RV32I::SRLI => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for SRLI")
                };

                self.srli(args.rd, args.rs1, args.imm);
            }

            RV32I::SRAI => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for SRAI")
                };

                self.srai(args.rd, args.rs1, args.imm);
            }

            _ => {
                return Err(format!("Unimplemented instruction: {:?}", inst));
            }
        }

        Ok(0)
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
            self.execute(inst).expect("Failed to execute instruction");
            self.last_inst = Some(inst);
            if self.exit_on_nop && inst.is_nop() {
                return 0;
            }
        }
    }
}

impl RV32ISA for CPU {
    fn addi(&mut self, rd: u8, rs1: u8, imm: u32) {
        let imm = sext(imm);
        self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_add_signed(imm);
    }

    fn slti(&mut self, rd: u8, rs1: u8, imm: u32) {
        let imm = sext(imm);
        self.regs[rd as usize] = if (self.regs[rs1 as usize] as i32) < imm {
            1
        } else {
            0
        };
    }

    fn sltiu(&mut self, rd: u8, rs1: u8, imm: u32) {
        let imm = sext(imm);
        self.regs[rd as usize] = if self.regs[rs1 as usize] < imm as u32 {
            1
        } else {
            0
        };
    }

    fn xori(&mut self, rd: u8, rs1: u8, imm: u32) {
        let imm = sext(imm);
        self.regs[rd as usize] = (self.regs[rs1 as usize] as i32 ^ imm) as u32;
    }

    fn ori(&mut self, rd: u8, rs1: u8, imm: u32) {
        let imm = sext(imm);
        self.regs[rd as usize] = (self.regs[rs1 as usize] as i32 | imm) as u32;
    }

    fn andi(&mut self, rd: u8, rs1: u8, imm: u32) {
        let imm = sext(imm);
        self.regs[rd as usize] = (self.regs[rs1 as usize] as i32 & imm) as u32;
    }

    fn slli(&mut self, rd: u8, rs1: u8, imm: u32) {
        let shamt = imm & 0x1F;
        self.regs[rd as usize] = self.regs[rs1 as usize] << shamt;
    }
    
    fn srli(&mut self, rd: u8, rs1: u8, imm: u32) {
        let shamt = imm & 0x1F;
        self.regs[rd as usize] = self.regs[rs1 as usize] >> shamt;
    }
    
    fn srai(&mut self, rd: u8, rs1: u8, imm: u32) {
        let shamt = imm & 0x1F;
        self.regs[rd as usize] = (self.regs[rs1 as usize] as i32 >> shamt) as u32;
    }    
}

fn sext(x: u32) -> i32 {
    // Shift left to bring the sign bit to the leftmost position
    let shifted = x << 20;
    // Arithmetic shift right to sign-extend and bring back to original position
    (shifted as i32) >> 20
}
