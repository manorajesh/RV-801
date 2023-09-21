use std::fs;

use crate::isa::{Instruction, RV32I, InstructionType};

pub struct CPU {
    regs: [u32; 32],
    pc: usize,
    memory: [u8; 0x10000],
    pub exit_on_nop: bool,
    pub last_inst: Option<Instruction>,
}

trait RV32ISA {
    // Load Upper Immediate: Loads the immediate value into rd.
    fn lui(&mut self, rd: u8, imm: u32);
    
    // Add Upper Immediate to PC: Adds the immediate value to the program counter.
    fn auipc(&mut self, rd: u8, imm: u32);
    
    // Jump And Link: Performs a jump and saves the return address in rd.
    fn jal(&mut self, rd: u8, imm: u32);
    
    // Jump And Link Register: Jumps to address in rs1 + immediate and saves return address in rd.
    fn jalr(&mut self, rd: u8, rs1: u8, imm: u32);
    
    // Branch if Equal: Branches if rs1 is equal to rs2.
    fn beq(&mut self, rs1: u8, rs2: u8, imm: u32);
    
    // Branch if Not Equal: Branches if rs1 is not equal to rs2.
    fn bne(&mut self, rs1: u8, rs2: u8, imm: u32);
    
    // Branch if Less Than: Branches if rs1 is less than rs2.
    fn blt(&mut self, rs1: u8, rs2: u8, imm: u32);
    
    // Branch if Greater or Equal: Branches if rs1 is greater or equal to rs2.
    fn bge(&mut self, rs1: u8, rs2: u8, imm: u32);
    
    // Branch if Less Than (Unsigned): Branches if rs1 is less than rs2, unsigned comparison.
    fn bltu(&mut self, rs1: u8, rs2: u8, imm: u32);
    
    // Branch if Greater or Equal (Unsigned): Branches if rs1 is greater or equal to rs2, unsigned comparison.
    fn bgeu(&mut self, rs1: u8, rs2: u8, imm: u32);
    
    // Load Byte: Loads a byte from memory into rd.
    fn lb(&mut self, rd: u8, rs1: u8, imm: u32);
    
    // Load Half-word: Loads a half-word from memory into rd.
    fn lh(&mut self, rd: u8, rs1: u8, imm: u32);
    
    // Load Word: Loads a word from memory into rd.
    fn lw(&mut self, rd: u8, rs1: u8, imm: u32);
    
    // Load Byte Unsigned: Loads a byte from memory into rd, zero-extended.
    fn lbu(&mut self, rd: u8, rs1: u8, imm: u32);
    
    // Load Half-word Unsigned: Loads a half-word from memory into rd, zero-extended.
    fn lhu(&mut self, rd: u8, rs1: u8, imm: u32);
    
    // Store Byte: Stores a byte to memory.
    fn sb(&mut self, rs1: u8, rs2: u8, imm: u32);
    
    // Store Half-word: Stores a half-word to memory.
    fn sh(&mut self, rs1: u8, rs2: u8, imm: u32);
    
    // Store Word: Stores a word to memory.
    fn sw(&mut self, rs1: u8, rs2: u8, imm: u32);
    
    // Add Immediate: Adds an immediate value to rs1 and stores the result in rd.
    fn addi(&mut self, rd: u8, rs1: u8, imm: u32);
    
    // Set if Less Than Immediate: Sets rd if rs1 is less than immediate.
    fn slti(&mut self, rd: u8, rs1: u8, imm: u32);
    
    // Set if Less Than Immediate Unsigned: Sets rd if rs1 is less than immediate, unsigned comparison.
    fn sltiu(&mut self, rd: u8, rs1: u8, imm: u32);
    
    // Exclusive OR Immediate: XORs rs1 and an immediate value, stores in rd.
    fn xori(&mut self, rd: u8, rs1: u8, imm: u32);
    
    // OR Immediate: ORs rs1 and an immediate value, stores in rd.
    fn ori(&mut self, rd: u8, rs1: u8, imm: u32);
    
    // AND Immediate: ANDs rs1 and an immediate value, stores in rd.
    fn andi(&mut self, rd: u8, rs1: u8, imm: u32);
    
    // Shift Left Logical Immediate: Shifts rs1 left by immediate bits, stores in rd.
    fn slli(&mut self, rd: u8, rs1: u8, imm: u32);
    
    // Shift Right Logical Immediate: Shifts rs1 right by immediate bits, zero-filled, stores in rd.
    fn srli(&mut self, rd: u8, rs1: u8, imm: u32);
    
    // Shift Right Arithmetic Immediate: Shifts rs1 right by immediate bits, sign-extended, stores in rd.
    fn srai(&mut self, rd: u8, rs1: u8, imm: u32);
    
    // Add: Adds rs1 and rs2, stores in rd.
    fn add(&mut self, rd: u8, rs1: u8, rs2: u8);
    
    // Subtract: Subtracts rs2 from rs1, stores in rd.
    fn sub(&mut self, rd: u8, rs1: u8, rs2: u8);
    
    // Shift Left Logical: Shifts rs1 left by rs2 bits, stores in rd.
    fn sll(&mut self, rd: u8, rs1: u8, rs2: u8);
    
    // Set if Less Than: Sets rd if rs1 is less than rs2.
    fn slt(&mut self, rd: u8, rs1: u8, rs2: u8);
    
    // Set if Less Than Unsigned: Sets rd if rs1 is less than rs2, unsigned comparison.
    fn sltu(&mut self, rd: u8, rs1: u8, rs2: u8);
    
    // Exclusive OR: XORs rs1 and rs2, stores in rd.
    fn xor(&mut self, rd: u8, rs1: u8, rs2: u8);
    
    // Shift Right Logical: Shifts rs1 right by rs2 bits, zero-filled, stores in rd.
    fn srl(&mut self, rd: u8, rs1: u8, rs2: u8);
    
    // Shift Right Arithmetic: Shifts rs1 right by rs2 bits, sign-extended, stores in rd.
    fn sra(&mut self, rd: u8, rs1: u8, rs2: u8);
    
    // OR: ORs rs1 and rs2, stores in rd.
    fn or(&mut self, rd: u8, rs1: u8, rs2: u8);
    
    // AND: ANDs rs1 and rs2, stores in rd.
    fn and(&mut self, rd: u8, rs1: u8, rs2: u8);
    
    // Fence: Memory ordering instruction.
    fn fence(&mut self, rd: u8, rs1: u8, imm: u32);
    
    // Environment Call: Makes a call to the environment.
    fn ecall(&mut self, rd: u8, rs1: u8, imm: u32);
    
    // Environment Break: Breaks to the debugger.
    fn ebreak(&mut self, rd: u8, rs1: u8, imm: u32);
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

    pub fn print_state(&self) {
        println!("PC: {:08X}", self.pc);
        for (i, reg) in self.regs.iter().enumerate() {
            println!("x{:02}: {:08}", i, reg);
        }
    }

    fn execute(&mut self, inst: Instruction) -> u8 {
        match inst.inst {
            RV32I::LUI => {
                let args = if let InstructionType::U(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for LUI")
                };

                self.lui(args.rd, args.imm);
            }

            RV32I::AUIPC => {
                let args = if let InstructionType::U(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for AUIPC")
                };

                self.auipc(args.rd, args.imm);
            }

            RV32I::JAL => {
                let args = if let InstructionType::J(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for JAL")
                };

                self.jal(args.rd, args.imm);
            }

            RV32I::JALR => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for JALR")
                };

                self.jalr(args.rd, args.rs1, args.imm as u32);
            }

            RV32I::BEQ => {
                let args = if let InstructionType::B(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for BEQ")
                };

                self.beq(args.rs1, args.rs2, args.imm as u32);
            }

            RV32I::BNE => {
                let args = if let InstructionType::B(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for BNE")
                };

                self.bne(args.rs1, args.rs2, args.imm as u32);
            }

            RV32I::BLT => {
                let args = if let InstructionType::B(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for BLT")
                };

                self.blt(args.rs1, args.rs2, args.imm as u32);
            }

            RV32I::BGE => {
                let args = if let InstructionType::B(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for BGE")
                };

                self.bge(args.rs1, args.rs2, args.imm as u32);
            }

            RV32I::BLTU => {
                let args = if let InstructionType::B(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for BLTU")
                };

                self.bltu(args.rs1, args.rs2, args.imm as u32);
            }

            RV32I::BGEU => {
                let args = if let InstructionType::B(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for BGEU")
                };

                self.bgeu(args.rs1, args.rs2, args.imm as u32);
            }

            RV32I::LB => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for LB")
                };

                self.lb(args.rd, args.rs1, args.imm as u32);
            }

            RV32I::LH => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for LH")
                };

                self.lh(args.rd, args.rs1, args.imm as u32);
            }

            RV32I::LW => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for LW")
                };

                self.lw(args.rd, args.rs1, args.imm as u32);
            }

            RV32I::LBU => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for LBU")
                };

                self.lbu(args.rd, args.rs1, args.imm as u32);
            }

            RV32I::LHU => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for LHU")
                };

                self.lhu(args.rd, args.rs1, args.imm as u32);
            }

            RV32I::SB => {
                let args = if let InstructionType::S(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for SB")
                };

                self.sb(args.rs1, args.rs2, args.imm as u32);
            }

            RV32I::SH => {
                let args = if let InstructionType::S(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for SH")
                };

                self.sh(args.rs1, args.rs2, args.imm as u32);
            }

            RV32I::SW => {
                let args = if let InstructionType::S(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for SW")
                };

                self.sw(args.rs1, args.rs2, args.imm as u32);
            }

            RV32I::ADDI => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for ADDI")
                };

                self.addi(args.rd, args.rs1, args.imm as u32);
            }

            RV32I::SLTI => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for SLTI")
                };

                self.slti(args.rd, args.rs1, args.imm as u32);
            }

            RV32I::SLTIU => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for SLTIU")
                };

                self.sltiu(args.rd, args.rs1, args.imm as u32);
            }

            RV32I::XORI => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for XORI")
                };

                self.xori(args.rd, args.rs1, args.imm as u32);
            }

            RV32I::ORI => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for ORI")
                };

                self.ori(args.rd, args.rs1, args.imm as u32);
            }

            RV32I::ANDI => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for ANDI")
                };

                self.andi(args.rd, args.rs1, args.imm as u32);
            }

            RV32I::SLLI => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for SLLI")
                };

                self.slli(args.rd, args.rs1, args.imm as u32);
            }

            RV32I::SRLI => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for SRLI")
                };

                self.srli(args.rd, args.rs1, args.imm as u32);
            }

            RV32I::SRAI => {
                let args = if let InstructionType::I(inst) = inst.inst_type {
                    inst
                } else {
                    panic!("Invalid instruction type for SRAI")
                };

                self.srai(args.rd, args.rs1, args.imm as u32);
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
    fn lui(&mut self, rd: u8, imm: u32) {
        self.regs[rd as usize] = imm;
    }

    fn auipc(&mut self, rd: u8, imm: u32) {
        self.regs[rd as usize] = (self.pc as u32) + imm;
    }

    fn jal(&mut self, rd: u8, imm: u32) {
        let tmp_pc = self.pc;
        let sign_extended_imm = ((imm as i32) << 1 >> 1) as u32; // Sign extend the immediate value.
        self.pc = self.pc.wrapping_add(sign_extended_imm as usize); // Jump to the new address.
        self.regs[rd as usize] = (tmp_pc as u32).wrapping_add(4); // Return PC + 4 to rd.
    }

    fn jalr(&mut self, rd: u8, rs1: u8, imm: u32) {
        let tmp_pc = self.pc;
        let sign_extended_imm = ((imm as i32) << 1 >> 1) as u32; // Sign extend the immediate value.
        self.pc = self.regs[rs1 as usize].wrapping_add(sign_extended_imm) as usize; // Jump to the new address.
        self.regs[rd as usize] = (tmp_pc as u32).wrapping_add(4); // Return PC + 4 to rd.
    }

    fn beq(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1_val = self.regs[rs1 as usize];
        let rs2_val = self.regs[rs2 as usize];
        if rs1_val == rs2_val {
            self.pc = ((self.pc as i32) + ((imm as i32) << 1)) as usize;
        }
    }

    fn bne(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1_val = self.regs[rs1 as usize];
        let rs2_val = self.regs[rs2 as usize];
        if rs1_val != rs2_val {
            self.pc = ((self.pc as i32) + ((imm as i32) << 1)) as usize;
        }
    }

    fn blt(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1_val = self.regs[rs1 as usize] as i32;
        let rs2_val = self.regs[rs2 as usize] as i32;
        if rs1_val < rs2_val {
            self.pc = ((self.pc as i32) + ((imm as i32) << 1)) as usize;
        }
    }

    fn bge(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1_val = self.regs[rs1 as usize] as i32;
        let rs2_val = self.regs[rs2 as usize] as i32;
        if rs1_val >= rs2_val {
            self.pc = ((self.pc as i32) + ((imm as i32) << 1)) as usize;
        }
    }

    fn bltu(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1_val = self.regs[rs1 as usize];
        let rs2_val = self.regs[rs2 as usize];
        if rs1_val < rs2_val {
            self.pc = ((self.pc as i32) + ((imm as i32) << 1)) as usize;
        }
    }

    fn bgeu(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1_val = self.regs[rs1 as usize];
        let rs2_val = self.regs[rs2 as usize];
        if rs1_val >= rs2_val {
            self.pc = ((self.pc as i32) + ((imm as i32) << 1)) as usize;
        }
    }

    fn lb(&mut self, rd: u8, rs1: u8, imm: u32) {
        let addr = self.regs[rs1 as usize].wrapping_add(imm) as usize;
        self.regs[rd as usize] = self.memory[addr] as i8 as i32 as u32;
    }

    fn lh(&mut self, rd: u8, rs1: u8, imm: u32) {
        let addr = self.regs[rs1 as usize].wrapping_add(imm) as usize;
        self.regs[rd as usize] = u16::from_le_bytes([
            self.memory[addr],
            self.memory[addr + 1],
        ]) as i16 as i32 as u32;
    }

    fn lw(&mut self, rd: u8, rs1: u8, imm: u32) {
        let addr = self.regs[rs1 as usize].wrapping_add(imm) as usize;
        self.regs[rd as usize] = u32::from_le_bytes([
            self.memory[addr],
            self.memory[addr + 1],
            self.memory[addr + 2],
            self.memory[addr + 3],
        ]);
    }

    fn lbu(&mut self, rd: u8, rs1: u8, imm: u32) {
        let addr = self.regs[rs1 as usize].wrapping_add(imm) as usize;
        self.regs[rd as usize] = self.memory[addr] as u32;
    }

    fn lhu(&mut self, rd: u8, rs1: u8, imm: u32) {
        let addr = self.regs[rs1 as usize].wrapping_add(imm) as usize;
        self.regs[rd as usize] = u16::from_le_bytes([
            self.memory[addr],
            self.memory[addr + 1],
        ]) as u32;
    }

    fn sb(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let addr = self.regs[rs1 as usize].wrapping_add(imm) as usize;
        self.memory[addr] = self.regs[rs2 as usize] as u8;
    }

    fn sh(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let addr = self.regs[rs1 as usize].wrapping_add(imm) as usize;
        let bytes = self.regs[rs2 as usize].to_le_bytes();
        self.memory[addr] = bytes[0];
        self.memory[addr + 1] = bytes[1];
    }

    fn sw(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let addr = self.regs[rs1 as usize].wrapping_add(imm) as usize;
        let bytes = self.regs[rs2 as usize].to_le_bytes();
        self.memory[addr] = bytes[0];
        self.memory[addr + 1] = bytes[1];
        self.memory[addr + 2] = bytes[2];
        self.memory[addr + 3] = bytes[3];
    }

    fn addi(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_add(imm);
    }

    fn slti(&mut self, rd: u8, rs1: u8, imm: u32) {
        let rs1_val = self.regs[rs1 as usize] as i32;
        if rs1_val < (imm as i32) {
            self.regs[rd as usize] = 1;
        } else {
            self.regs[rd as usize] = 0;
        }
    }

    fn sltiu(&mut self, rd: u8, rs1: u8, imm: u32) {
        let rs1_val = self.regs[rs1 as usize];
        if rs1_val < imm {
            self.regs[rd as usize] = 1;
        } else {
            self.regs[rd as usize] = 0;
        }
    }

    fn xori(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.regs[rd as usize] = self.regs[rs1 as usize] ^ imm;
    }

    fn ori(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.regs[rd as usize] = self.regs[rs1 as usize] | imm;
    }

    fn andi(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.regs[rd as usize] = self.regs[rs1 as usize] & imm;
    }

    fn slli(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.regs[rd as usize] = self.regs[rs1 as usize] << imm;
    }

    fn srli(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.regs[rd as usize] = self.regs[rs1 as usize] >> imm;
    }

    fn srai(&mut self, rd: u8, rs1: u8, imm: u32) {
        let rs1_val = self.regs[rs1 as usize] as i32;
        self.regs[rd as usize] = (rs1_val >> imm) as u32;
    }

    fn add(&mut self, rd: u8, rs1: u8, rs2: u8) {
        self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_add(self.regs[rs2 as usize]);
    }

    fn sub(&mut self, rd: u8, rs1: u8, rs2: u8) {
        self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_sub(self.regs[rs2 as usize]);
    }

    fn sll(&mut self, rd: u8, rs1: u8, rs2: u8) {
        self.regs[rd as usize] = self.regs[rs1 as usize] << (self.regs[rs2 as usize] & 0x1F);
    }

    fn slt(&mut self, rd: u8, rs1: u8, rs2: u8) {
        let rs1_val = self.regs[rs1 as usize] as i32;
        let rs2_val = self.regs[rs2 as usize] as i32;
        if rs1_val < rs2_val {
            self.regs[rd as usize] = 1;
        } else {
            self.regs[rd as usize] = 0;
        }
    }

    fn sltu(&mut self, rd: u8, rs1: u8, rs2: u8) {
        let rs1_val = self.regs[rs1 as usize];
        let rs2_val = self.regs[rs2 as usize];
        if rs1_val < rs2_val {
            self.regs[rd as usize] = 1;
        } else {
            self.regs[rd as usize] = 0;
        }
    }

    fn xor(&mut self, rd: u8, rs1: u8, rs2: u8) {
        self.regs[rd as usize] = self.regs[rs1 as usize] ^ self.regs[rs2 as usize];
    }

    fn srl(&mut self, rd: u8, rs1: u8, rs2: u8) {
        self.regs[rd as usize] = self.regs[rs1 as usize] >> (self.regs[rs2 as usize] & 0x1F);
    }

    fn sra(&mut self, rd: u8, rs1: u8, rs2: u8) {
        let rs1_val = self.regs[rs1 as usize] as i32;
        self.regs[rd as usize] = (rs1_val >> (self.regs[rs2 as usize] & 0x1F)) as u32;
    }

    fn or(&mut self, rd: u8, rs1: u8, rs2: u8) {
        self.regs[rd as usize] = self.regs[rs1 as usize] | self.regs[rs2 as usize];
    }

    fn and(&mut self, rd: u8, rs1: u8, rs2: u8) {
        self.regs[rd as usize] = self.regs[rs1 as usize] & self.regs[rs2 as usize];
    }

    fn fence(&mut self, rd: u8, rs1: u8, imm: u32) {}

    fn ecall(&mut self, rd: u8, rs1: u8, imm: u32) {}

    fn ebreak(&mut self, rd: u8, rs1: u8, imm: u32) {}
}