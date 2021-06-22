use std::hint::unreachable_unchecked;

#[derive(Debug)]
pub struct CPU {
    xlen: Xlen,
    pc: u64,
    instr_count:i32,
    reg_file: [u32;32],
    memory: Vec<u32>,
}

impl CPU {
    pub fn read_memory(&self, mut addr: u32, data: u32) {
        addr -= 0x80000000;
        assert!(addr > 0 && addr < (self.memory.len() as u32));
        // self.memory = &self.memory[..(addr as usize)] + data + &self.memory[addr + data];
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Xlen {
    Bit32,
    Bit64
}

#[allow(dead_code)]
pub enum PrivilegeMode {
    User,
    Supervisor,
    Reserved,
    Machine,
}

#[allow(dead_code)]
pub enum TrapType {
    LoadPageFault,
    UserSoftwareInterrupt,
}

#[derive(Clone,Copy)]
pub enum InstrFormat {
    R,
    I,
    S,
    B,
    U,
    J,
}

impl InstrFormat {
    fn decode(&self, instr: u32) -> Instruction {
        let opcode = (instr & 0b1111111) as u8;
        match self {
            InstrFormat::I => {
                let imm = (instr >> 20) & 0b1111_1111_1111;
                let rs1 = ((instr >> 15) & 0b11111) as u8;
                let funct3 = ((instr >> 12) & 0b111) as u8;
                let rd = ((instr >> 7) & 0b11111) as u8;

                let imm = ((imm as i32) << 20) >> 20;
                match opcode {
                    0b0010011 => {
                        match funct3 {
                            0b000 => Instruction::Addi {rd, rs1, imm},
                            0b010 => Instruction::Slti {rd, rs1, imm},
                            0b011 => Instruction::Sltiu {rd, rs1, imm},
                            0b100 => Instruction::Xori {rd, rs1, imm},
                            0b110 => Instruction::Ori {rd, rs1, imm},
                            0b111 => Instruction::Andi {rd, rs1, imm},
                            _ => Instruction::Undefined
                        };

                    }
                    _ => {}
                };
            }
            _ => {}
        }
        Instruction::Undefined
    }
}

#[derive(Debug)]
enum Instruction {
    Undefined,
    Lui {rd: u8, imm: i32},
    Auipc {rd: u8, imm:i32},
    Addi {rd: u8, rs1:u8, imm: i32},
    Slti {rd: u8, rs1: u8, imm: i32},
    Sltiu {rd: u8, rs1: u8, imm:i32},
    Xori {rd:u8, rs1:u8, imm:i32},
    Ori {rd:u8, rs1:u8, imm:i32},
    Andi {rd:u8, rs1:u8, imm:i32}
}

fn fetch() {}

fn decode(instr: u32) {
    let opcode = (instr & 0b11111111) as u8;
    let decoded = if let Some(typ) = &ENCODING_TABLE[opcode as usize] {
        typ.decode(instr)
    } else {
        Instruction::Undefined
    };
    print!("Instruction {:#010x} is {:?}\n", instr, decoded);
}

/* Encoding table for rv32i and rv64i */
const ENCODING_TABLE: [Option<InstrFormat>; 13] = [
    /* 0b0000011 */ Some(InstrFormat::I),
    /* 0b0001111 */ Some(InstrFormat::I),
    /* 0b0010011 */ Some(InstrFormat::I),
    /* 0b0010111 */ Some(InstrFormat::U),
    /* 0b0011011 */ Some(InstrFormat::I),
    /* 0b0100011 */ Some(InstrFormat::S),
    /* 0b0110011 */ Some(InstrFormat::R),
    /* 0b0110111 */ Some(InstrFormat::U),
    /* 0b0111011 */ Some(InstrFormat::R),
    /* 0b1100011 */ Some(InstrFormat::B),
    /* 0b1100111 */ Some(InstrFormat::J),
    /* 0b1101111 */ Some(InstrFormat::J),
    /* 0b1110011 */ Some(InstrFormat::I),
];

fn process_pipeline() {}
