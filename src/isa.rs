#[derive(Debug, Clone, Copy)]
pub enum RV32I {
    LUI,    // Load Upper Immediate
    AUIPC,  // Add Upper Immediate to PC
    JAL,    // Jump and Link
    JALR,   // Jump and Link Register
    BEQ,    // Branch if Equal
    BNE,    // Branch if Not Equal
    BLT,    // Branch if Less Than
    BGE,    // Branch if Greater Than or Equal
    BLTU,   // Branch if Less Than Unsigned
    BGEU,   // Branch if Greater Than or Equal Unsigned
    LB,     // Load Byte
    LH,     // Load Halfword
    LW,     // Load Word
    LBU,    // Load Byte Unsigned
    LHU,    // Load Halfword Unsigned
    SB,     // Store Byte
    SH,     // Store Halfword
    SW,     // Store Word
    ADDI,   // Add Immediate
    SLTI,   // Set Less Than Immediate
    SLTIU,  // Set Less Than Immediate Unsigned
    XORI,   // Exclusive OR Immediate
    ORI,    // OR Immediate
    ANDI,   // AND Immediate
    SLLI,   // Shift Left Logical Immediate
    SRLI,   // Shift Right Logical Immediate
    SRAI,   // Shift Right Arithmetic Immediate
    ADD,    // Add
    SUB,    // Subtract
    SLL,    // Shift Left Logical
    SLT,    // Set Less Than
    SLTU,   // Set Less Than Unsigned
    XOR,    // Exclusive OR
    SRL,    // Shift Right Logical
    SRA,    // Shift Right Arithmetic
    OR,     // OR
    AND,    // AND
    FENCE,  // Fence
    ECALL,  // Environment Call
    EBREAK, // Environment Break
}

#[derive(Debug, Clone, Copy)]
pub struct I {
    pub imm: u32,
    pub rs1: u8,
    pub funct3: u8,
    pub rd: u8,
    pub opcode: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct U {
    pub imm: u32,
    pub rd: u8,
    pub opcode: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum InstructionType {
    I(I),
    U(U),
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub inst_type: InstructionType,
    pub inst: RV32I,
    // doesn't mean anything if I am inputting argument (supposed to confirm parsing)
    pub raw: u32,
}

impl Instruction {
    pub fn from(inst: u32) -> Self {
        let inst_type = parse_inst(inst).expect("Invalid instruction");
        let decoded_inst = get_inst(inst_type).expect("Invalid instruction");

        Instruction {
            inst_type,
            inst: decoded_inst,
            raw: inst,
        }
    }

    pub fn is_nop(&self) -> bool {
        if self.raw == 0 {
            return true;
        }

        match self.inst {
            RV32I::ADDI => {
                let i = match self.inst_type {
                    InstructionType::I(i) => i,
                    _ => panic!("Invalid instruction type"),
                };

                i.imm == 0 && i.rs1 == 0 && i.funct3 == 0 && i.rd == 0
            }
            _ => false,
        }
    }

    pub fn to_bin(&self) -> u32 {
        match self.inst_type {
            InstructionType::I(i) => {
                let mut inst = 0;

                inst |= i.imm << 20;
                inst |= (i.rs1 as u32) << 15;
                inst |= (i.funct3 as u32) << 12;
                inst |= (i.rd as u32) << 7;
                inst |= i.opcode as u32;

                inst
            }
            InstructionType::U(u) => {
                let mut inst = 0;

                inst |= u.imm << 12;
                inst |= (u.rd as u32) << 7;
                inst |= u.opcode as u32;

                inst
            }
        }
    }
}

fn get_opcode(inst: u32) -> u8 {
    (inst & 0x7F) as u8
}

fn parse_inst(inst: u32) -> Result<InstructionType, String> {
    let opcode = get_opcode(inst);

    match opcode {
        // I-Type
        0b1100111 | 0b0000011 | 0b0010011 => {
            let imm = inst >> 20;
            let rs1 = ((inst >> 15) & 0x1F) as u8;
            let funct3 = ((inst >> 12) & 0x7) as u8;
            let rd = ((inst >> 7) & 0x1F) as u8;

            Ok(InstructionType::I(I {
                imm,
                rs1,
                funct3,
                rd,
                opcode,
            }))
        }

        // U-Type
        0b0110111 | 0b0010111 => {
            let imm = inst >> 12;
            let rd = ((inst >> 7) & 0x1F) as u8;

            Ok(InstructionType::U(U { imm, rd, opcode }))
        }

        // NOP
        0b0000000 => {
            // ADDI x0, x0, 0

            let imm = 0;
            let rs1 = 0;
            let funct3 = 0;
            let rd = 0;

            Ok(InstructionType::I(I {
                imm,
                rs1,
                funct3,
                rd,
                opcode,
            }))
        }

        _ => Err(format!("Invalid opcode: {:#b}", opcode)),
    }
}

fn get_inst(inst: InstructionType) -> Result<RV32I, String> {
    match inst {
        InstructionType::I(i) => match i.opcode {
            0b1100111 => match i.funct3 {
                0b000 => Ok(RV32I::JALR),
                _ => Err(format!("Invalid funct3: {:#b}", i.funct3)),
            },
            0b0000011 => match i.funct3 {
                0b000 => Ok(RV32I::LB),
                0b001 => Ok(RV32I::LH),
                0b010 => Ok(RV32I::LW),
                0b100 => Ok(RV32I::LBU),
                0b101 => Ok(RV32I::LHU),
                _ => Err(format!("Invalid funct3: {:#b}", i.funct3)),
            },
            0b0010011 => match i.funct3 {
                0b001 => Ok(RV32I::SLLI),
                0b101 => match i.imm & 0x400 {
                    0x400 => Ok(RV32I::SRAI),
                    _ => Ok(RV32I::SRLI),
                },
                0b000 => Ok(RV32I::ADDI),
                0b010 => Ok(RV32I::SLTI),
                0b011 => Ok(RV32I::SLTIU),
                0b100 => Ok(RV32I::XORI),
                0b110 => Ok(RV32I::ORI),
                0b111 => Ok(RV32I::ANDI),
                _ => Err(format!("Invalid funct3: {:#b}", i.funct3)),
            },
            0b1110011 => match i.funct3 {
                0b000 => Ok(RV32I::ECALL),
                0b001 => Ok(RV32I::EBREAK),
                _ => Err(format!("Invalid funct3: {:#b}", i.funct3)),
            },

            0b0000000 => {
                assert_eq!(i.funct3, 0);
                assert_eq!(i.imm, 0);

                Ok(RV32I::ADDI)
            }
            _ => Err(format!("Invalid funct3: {:#b}", i.funct3)),
        },

        InstructionType::U(u) => match u.opcode {
            0b0110111 => Ok(RV32I::LUI),
            0b0010111 => Ok(RV32I::AUIPC),
            _ => Err(format!("Invalid opcode: {:#b}", u.opcode)),
        },
    }
}
