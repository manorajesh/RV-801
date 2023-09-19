pub enum RV32I {
    LUI, // Load Upper Immediate
    AUIPC, // Add Upper Immediate to PC
    JAL, // Jump and Link
    JALR, // Jump and Link Register
    BEQ, // Branch if Equal
    BNE, // Branch if Not Equal
    BLT, // Branch if Less Than
    BGE, // Branch if Greater Than or Equal
    BLTU, // Branch if Less Than Unsigned
    BGEU, // Branch if Greater Than or Equal Unsigned
    LB, // Load Byte
    LH, // Load Halfword
    LW, // Load Word
    LBU, // Load Byte Unsigned
    LHU, // Load Halfword Unsigned
    SB, // Store Byte
    SH, // Store Halfword
    SW, // Store Word
    ADDI, // Add Immediate
    SLTI, // Set Less Than Immediate
    SLTIU, // Set Less Than Immediate Unsigned
    XORI, // Exclusive OR Immediate
    ORI, // OR Immediate
    ANDI, // AND Immediate
    SLLI, // Shift Left Logical Immediate
    SRLI, // Shift Right Logical Immediate
    SRAI, // Shift Right Arithmetic Immediate
    ADD, // Add
    SUB, // Subtract
    SLL, // Shift Left Logical
    SLT, // Set Less Than
    SLTU, // Set Less Than Unsigned
    XOR, // Exclusive OR
    SRL, // Shift Right Logical
    SRA, // Shift Right Arithmetic
    OR, // OR
    AND, // AND
    FENCE, // Fence
    ECALL, // Environment Call
    EBREAK, // Environment Break
}

pub struct R {
    funct7: u8,
    rs2: u8,
    rs1: u8,
    funct3: u8,
    rd: u8,
    opcode: u8,
}

pub struct I {
    imm: u16,
    rs1: u8,
    funct3: u8,
    rd: u8,
    opcode: u8,
}

pub struct S {
    imm: u16,
    rs2: u8,
    rs1: u8,
    funct3: u8,
    opcode: u8,
}

pub struct B {
    imm: u16,
    rs2: u8,
    rs1: u8,
    funct3: u8,
    opcode: u8,
}

pub struct U {
    imm: u32,
    rd: u8,
    opcode: u8,
}

pub struct J {
    imm: u32,
    rd: u8,
    opcode: u8,
}

pub enum InstructionType {
    R(R),
    I(I),
    S(S),
    B(B),
    U(U),
    J(J),
}

pub struct Instruction {
    inst_type: InstructionType,
    inst: RV32I,
}

impl Instruction {
    pub fn from(inst: u32) -> Self {

    }
}

fn get_opcode(inst: u32) -> u8 {
    (inst & 0x7F) as u8
}

fn get_inst_type(opcode: u8) -> InstructionType {
    match opcode {
        0b0110111 || 0b0010111 => {
            
        }
    }
}