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
pub struct R {
    pub funct7: u8,
    pub rs2: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub rd: u8,
    pub opcode: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct I {
    pub imm: u16,
    pub rs1: u8,
    pub funct3: u8,
    pub rd: u8,
    pub opcode: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct S {
    pub imm: u16,
    pub rs2: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub opcode: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct B {
    pub imm: u16,
    pub rs2: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub opcode: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct U {
    pub imm: u32,
    pub rd: u8,
    pub opcode: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct J {
    pub imm: u32,
    pub rd: u8,
    pub opcode: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct FENCE {
    pub fm: u8,
    pub pred: u8,
    pub succ: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub rd: u8,
    pub opcode: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum InstructionType {
    R(R),
    I(I),
    S(S),
    B(B),
    U(U),
    J(J),
    FENCE(FENCE),
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

    pub fn get_args(&self) -> InstructionType {
        self.inst_type
    }
}

fn get_opcode(inst: u32) -> u8 {
    (inst & 0x7F) as u8
}

fn parse_inst(inst: u32) -> Option<InstructionType> {
    let opcode = get_opcode(inst);

    match opcode {
        // U-Type
        0b0110111 | 0b0010111 => {
            let imm = ((inst >> 12) & 0xFFFFF) as u32;
            let rd = ((inst >> 7) & 0x1F) as u8;

            Some(InstructionType::U(U { imm, rd, opcode }))
        }

        // J-Type
        0b1101111 => {
            let imm = ((inst >> 12) & 0xFFFFF) as u32;
            let rd = ((inst >> 7) & 0x1F) as u8;

            Some(InstructionType::J(J { imm, rd, opcode }))
        }

        // I-Type
        0b1100111 | 0b0000011 | 0b0010011 => {
            let imm = ((inst >> 20) & 0xFFF) as u16;
            let rs1 = ((inst >> 15) & 0x1F) as u8;
            let funct3 = ((inst >> 12) & 0x7) as u8;
            let rd = ((inst >> 7) & 0x1F) as u8;

            Some(InstructionType::I(I {
                imm,
                rs1,
                funct3,
                rd,
                opcode,
            }))
        }

        // B-Type
        0b1100011 => {
            let imm = ((((inst >> 31) & 0x1) << 12)
                | (((inst >> 7) & 0x1) << 11)
                | (((inst >> 25) & 0x3F) << 5)
                | (((inst >> 8) & 0xF) << 1)) as u16;
            let rs2 = ((inst >> 20) & 0x1F) as u8;
            let rs1 = ((inst >> 15) & 0x1F) as u8;
            let funct3 = ((inst >> 12) & 0x7) as u8;

            Some(InstructionType::B(B {
                imm,
                rs2,
                rs1,
                funct3,
                opcode,
            }))
        }

        // S-Type
        0b0100011 => {
            let imm = ((((inst >> 25) & 0x7F) << 5) | (((inst >> 7) & 0x1F) << 0)) as u16;
            let rs2 = ((inst >> 20) & 0x1F) as u8;
            let rs1 = ((inst >> 15) & 0x1F) as u8;
            let funct3 = ((inst >> 12) & 0x7) as u8;

            Some(InstructionType::S(S {
                imm,
                rs2,
                rs1,
                funct3,
                opcode,
            }))
        }

        // R-Type
        0b0110011 => {
            let funct7 = ((inst >> 25) & 0x7F) as u8;
            let rs2 = ((inst >> 20) & 0x1F) as u8;
            let rs1 = ((inst >> 15) & 0x1F) as u8;
            let funct3 = ((inst >> 12) & 0x7) as u8;
            let rd = ((inst >> 7) & 0x1F) as u8;

            Some(InstructionType::R(R {
                funct7,
                rs2,
                rs1,
                funct3,
                rd,
                opcode,
            }))
        }

        // ECALL, EBREAK
        0b1110011 => {
            let imm = ((inst >> 20) & 0xFFF) as u16;
            let rs1 = ((inst >> 15) & 0x1F) as u8;
            let funct3 = ((inst >> 12) & 0x7) as u8;
            let rd = ((inst >> 7) & 0x1F) as u8;

            assert_eq!(imm, 0);
            assert_eq!(rs1, 0);
            assert_eq!(funct3, 0);
            assert_eq!(rd, 0);

            Some(InstructionType::I(I {
                imm,
                rs1,
                funct3,
                rd,
                opcode,
            }))
        }

        // FENCE
        0b0001111 => {
            let fm = ((inst >> 28) & 0xF) as u8;
            let pred = ((inst >> 24) & 0xF) as u8;
            let succ = ((inst >> 20) & 0xF) as u8;
            let rs1 = ((inst >> 15) & 0x1F) as u8;
            let funct3 = ((inst >> 12) & 0x7) as u8;
            let rd = ((inst >> 7) & 0x1F) as u8;

            assert_eq!(funct3, 0);

            Some(InstructionType::FENCE(FENCE {
                fm,
                pred,
                succ,
                rs1,
                funct3,
                rd,
                opcode,
            }))
        }

        0b0000000 => {
            // ADDI x0, x0, 0

            let imm = 0;
            let rs1 = 0;
            let funct3 = 0;
            let rd = 0;

            Some(InstructionType::I(I {
                imm,
                rs1,
                funct3,
                rd,
                opcode,
            }))
        }

        _ => None,
    }
}

fn get_inst(inst: InstructionType) -> Option<RV32I> {
    match inst {
        InstructionType::R(i) => match i.funct7 {
            0b0000000 => match i.funct3 {
                0b000 => Some(RV32I::ADD),
                0b001 => Some(RV32I::SLL),
                0b010 => Some(RV32I::SLT),
                0b011 => Some(RV32I::SLTU),
                0b100 => Some(RV32I::XOR),
                0b101 => Some(RV32I::SRL),
                0b110 => Some(RV32I::OR),
                0b111 => Some(RV32I::AND),
                _ => None,
            },
            0b0100000 => match i.funct3 {
                0b000 => Some(RV32I::SUB),
                0b101 => Some(RV32I::SRA),
                _ => None,
            },
            _ => None,
        },

        InstructionType::B(i) => match i.funct3 {
            0b000 => Some(RV32I::BEQ),
            0b001 => Some(RV32I::BNE),
            0b100 => Some(RV32I::BLT),
            0b101 => Some(RV32I::BGE),
            0b110 => Some(RV32I::BLTU),
            0b111 => Some(RV32I::BGEU),
            _ => None,
        },

        InstructionType::I(i) => match i.opcode {
            0b1100111 => match i.funct3 {
                0b000 => Some(RV32I::JALR),
                _ => None,
            },
            0b0000011 => match i.funct3 {
                0b000 => Some(RV32I::LB),
                0b001 => Some(RV32I::LH),
                0b010 => Some(RV32I::LW),
                0b100 => Some(RV32I::LBU),
                0b101 => Some(RV32I::LHU),
                _ => None,
            },
            0b0010011 => match i.funct3 {
                0b001 => Some(RV32I::SLLI),
                0b101 => match i.imm & 0x400 {
                    0x400 => Some(RV32I::SRAI),
                    _ => Some(RV32I::SRLI),
                },
                0b000 => Some(RV32I::ADDI),
                0b010 => Some(RV32I::SLTI),
                0b011 => Some(RV32I::SLTIU),
                0b100 => Some(RV32I::XORI),
                0b110 => Some(RV32I::ORI),
                0b111 => Some(RV32I::ANDI),
                _ => None,
            },
            0b1110011 => match i.funct3 {
                0b000 => Some(RV32I::ECALL),
                0b001 => Some(RV32I::EBREAK),
                _ => None,
            },

            0b0000000 => {
                assert_eq!(i.funct3, 0);
                assert_eq!(i.imm, 0);

                Some(RV32I::ADDI)
            }
            _ => None,
        },

        InstructionType::S(i) => match i.funct3 {
            0b000 => Some(RV32I::SB),
            0b001 => Some(RV32I::SH),
            0b010 => Some(RV32I::SW),
            _ => None,
        },

        InstructionType::U(i) => match i.opcode {
            0b0110111 => Some(RV32I::LUI),
            0b0010111 => Some(RV32I::AUIPC),
            _ => None,
        },

        InstructionType::J(i) => match i.opcode {
            0b1101111 => Some(RV32I::JAL),
            _ => None,
        },

        InstructionType::FENCE(i) => match i.opcode {
            0b0001111 => Some(RV32I::FENCE),
            _ => None,
        },
    }
}
