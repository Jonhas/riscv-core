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
    pub fn read_memory(&self, mut addr: u32, mut data: u32) {
        addr -= 0x80000000;
        assert!(addr > 0 && addr < (self.memory.len() as u32));
        // self.memory = &self.memory[..(addr as usize)] + data + &self.memory[addr + data];
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Xlen {
    Bit16,
    Bit32,
    Bit64,
    Bit128,
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
        let opcode = instr * 0b1111111;
        match self {
            InstrFormat::I => {
                let imm = (instr >> 20) & 0b1111_1111_1111;
                let rs1 = (instr >> 15) & 0b11111;
                let funct3 = (instr >> 12) & 0b111;
                let rd = (instr >> 7) & 0b11111;

                match opcode {
                    0b0010011 => {
                        match funct3 {
                            0b000 => Instruction::Addi {rd, rs1, imm},
                            _ => Instruction::Undefined,
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
    Lui {rd: u32, imm: u32},
    Auipc {rd: u32, imm:u32},
    Addi {rd: u32, rs1:u32, imm: u32}
}

fn fetch() {}

fn decode(instr: u32) {
    let opcode = instr & 0b11111111;
    let decoded = if let Some(typ) = &ENCODING_TABLE[opcode as usize] {
        typ.decode(instr)
    } else {
        Instruction::Undefined
    };
    print!("Instruction {:#010x} is {:?}\n", instr, decoded);
}

const ENCODING_TABLE: [Option<InstrFormat>; 128] = [
    /* 0b0000000 */ None,
    /* 0b0000001 */ None,
    /* 0b0000010 */ None,
    /* 0b0000011 */ Some(InstrFormat::I),
    /* 0b0000100 */ None,
    /* 0b0000101 */ None,
    /* 0b0000110 */ None,
    /* 0b0000111 */ None,
    /* 0b0001000 */ None,
    /* 0b0001001 */ None,
    /* 0b0001010 */ None,
    /* 0b0001011 */ None,
    /* 0b0001100 */ None,
    /* 0b0001101 */ None,
    /* 0b0001110 */ None,
    /* 0b0001111 */ Some(InstrFormat::I),
    /* 0b0010000 */ None,
    /* 0b0010001 */ None,
    /* 0b0010010 */ None,
    /* 0b0010011 */ Some(InstrFormat::I),
    /* 0b0010100 */ None,
    /* 0b0010101 */ None,
    /* 0b0010110 */ None,
    /* 0b0010111 */ Some(InstrFormat::U),
    /* 0b0011000 */ None,
    /* 0b0011001 */ None,
    /* 0b0011010 */ None,
    /* 0b0011011 */ Some(InstrFormat::I),
    /* 0b0011100 */ None,
    /* 0b0011101 */ None,
    /* 0b0011110 */ None,
    /* 0b0011111 */ None,
    /* 0b0100000 */ None,
    /* 0b0100001 */ None,
    /* 0b0100010 */ None,
    /* 0b0100011 */ Some(InstrFormat::S),
    /* 0b0100100 */ None,
    /* 0b0100101 */ None,
    /* 0b0100110 */ None,
    /* 0b0100111 */ None,
    /* 0b0101000 */ None,
    /* 0b0101001 */ None,
    /* 0b0101010 */ None,
    /* 0b0101011 */ None,
    /* 0b0101100 */ None,
    /* 0b0101101 */ None,
    /* 0b0101110 */ None,
    /* 0b0101111 */ None,
    /* 0b0110000 */ None,
    /* 0b0110001 */ None,
    /* 0b0110010 */ None,
    /* 0b0110011 */ Some(InstrFormat::R),
    /* 0b0110100 */ None,
    /* 0b0110101 */ None,
    /* 0b0110110 */ None,
    /* 0b0110111 */ Some(InstrFormat::U),
    /* 0b0111000 */ None,
    /* 0b0111001 */ None,
    /* 0b0111010 */ None,
    /* 0b0111011 */ Some(InstrFormat::R),
    /* 0b0111100 */ None,
    /* 0b0111101 */ None,
    /* 0b0111110 */ None,
    /* 0b0111111 */ None,
    /* 0b1000000 */ None,
    /* 0b1000001 */ None,
    /* 0b1000010 */ None,
    /* 0b1000011 */ None,
    /* 0b1000100 */ None,
    /* 0b1000101 */ None,
    /* 0b1000110 */ None,
    /* 0b1000111 */ None,
    /* 0b1001000 */ None,
    /* 0b1001001 */ None,
    /* 0b1001010 */ None,
    /* 0b1001011 */ None,
    /* 0b1001100 */ None,
    /* 0b1001101 */ None,
    /* 0b1001110 */ None,
    /* 0b1001111 */ None,
    /* 0b1010000 */ None,
    /* 0b1010001 */ None,
    /* 0b1010010 */ None,
    /* 0b1010011 */ None,
    /* 0b1010100 */ None,
    /* 0b1010101 */ None,
    /* 0b1010110 */ None,
    /* 0b1010111 */ None,
    /* 0b1011000 */ None,
    /* 0b1011001 */ None,
    /* 0b1011010 */ None,
    /* 0b1011011 */ None,
    /* 0b1011100 */ None,
    /* 0b1011101 */ None,
    /* 0b1011110 */ None,
    /* 0b1011111 */ None,
    /* 0b1100000 */ None,
    /* 0b1100001 */ None,
    /* 0b1100010 */ None,
    /* 0b1100011 */ Some(InstrFormat::B),
    /* 0b1100100 */ None,
    /* 0b1100101 */ None,
    /* 0b1100110 */ None,
    /* 0b1100111 */ Some(InstrFormat::J),
    /* 0b1101000 */ None,
    /* 0b1101001 */ None,
    /* 0b1101010 */ None,
    /* 0b1101011 */ None,
    /* 0b1101100 */ None,
    /* 0b1101101 */ None,
    /* 0b1101110 */ None,
    /* 0b1101111 */ Some(InstrFormat::J),
    /* 0b1110000 */ None,
    /* 0b1110001 */ None,
    /* 0b1110010 */ None,
    /* 0b1110011 */ Some(InstrFormat::I),
    /* 0b1110100 */ None,
    /* 0b1110101 */ None,
    /* 0b1110110 */ None,
    /* 0b1110111 */ None,
    /* 0b1111000 */ None,
    /* 0b1111001 */ None,
    /* 0b1111010 */ None,
    /* 0b1111011 */ None,
    /* 0b1111100 */ None,
    /* 0b1111101 */ None,
    /* 0b1111110 */ None,
    /* 0b1111111 */ None,


];

fn process_pipeline() {}
