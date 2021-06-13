pub struct CPU 
{
    clock: u64, 
    xlen:Xlen, 
    pc: u64
}

#[derive(Debug)]
enum Instruction {
    Undefined,
}

pub enum Type {
    R,
    I,
    S,
    B,
    U,
    J,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Xlen {
    Bit32, /* Bit64  */
           /* Bit128 */
}


#[allow(dead_code)]
pub enum PrivelegeMode 
{
    User,
    Supervisor, 
    Reserved,
    Machine
}

#[allow(dead_code)]
pub enum TrapType {
    LoadPageFault, 
    UserSoftwareInterrupt
}

fn decode(instr: u32) {
    let opcode = instr & 0b11111111;
    let decoded = if let Some(typ) = &ENCODING_TABLE[opcode as usize] {
        typ.decode(instr)
    } else {
        Instruction::Undefined
    };
    print!("Instruction {:#010x} is {:?}\n", instr, decoded);
}

impl Type {
    fn decode(&self, inst: u32) -> Instruction {
        Instruction::Undefined
    }
}

const ENCODING_TABLE: [Option<Type>; 128] = [
    /* 000000000 */ None, /* 000000001 */ None, /* 000000010 */ None,
    /* 000000011 */ None, /* 000000100 */ None, /* 000000101 */ None,
    /* 000000110 */ None, /* 000000111 */ None, /* 000001000 */ None,
    /* 000001001 */ None, /* 000001010 */ None, /* 000001011 */ None,
    /* 000001100 */ None, /* 000001101 */ None, /* 000001110 */ None,
    /* 000001111 */ None, /* 000010000 */ None, /* 000010001 */ None,
    /* 000010010 */ None, /* 000010011 */ None, /* 000010100 */ None,
    /* 000010101 */ None, /* 000010110 */ None, /* 000010111 */ None,
    /* 000011000 */ None, /* 000011001 */ None, /* 000011010 */ None,
    /* 000011011 */ None, /* 000011100 */ None, /* 000011101 */ None,
    /* 000011110 */ None, /* 000011111 */ None, /* 000100000 */ None,
    /* 000100001 */ None, /* 000100010 */ None, /* 000100011 */ None,
    /* 000100100 */ None, /* 000100101 */ None, /* 000100110 */ None,
    /* 000100111 */ None, /* 000101000 */ None, /* 000101001 */ None,
    /* 000101010 */ None, /* 000101011 */ None, /* 000101100 */ None,
    /* 000101101 */ None, /* 000101110 */ None, /* 000101111 */ None,
    /* 000110000 */ None, /* 000110001 */ None, /* 000110010 */ None,
    /* 000110011 */ None, /* 000110100 */ None, /* 000110101 */ None,
    /* 000110110 */ None, /* 000110111 */ None, /* 000111000 */ None,
    /* 000111001 */ None, /* 000111010 */ None, /* 000111011 */ None,
    /* 000111100 */ None, /* 000111101 */ None, /* 000111110 */ None,
    /* 000111111 */ None, /* 001000000 */ None, /* 001000001 */ None,
    /* 001000010 */ None, /* 001000011 */ None, /* 001000100 */ None,
    /* 001000101 */ None, /* 001000110 */ None, /* 001000111 */ None,
    /* 001001000 */ None, /* 001001001 */ None, /* 001001010 */ None,
    /* 001001011 */ None, /* 001001100 */ None, /* 001001101 */ None,
    /* 001001110 */ None, /* 001001111 */ None, /* 001010000 */ None,
    /* 001010001 */ None, /* 001010010 */ None, /* 001010011 */ None,
    /* 001010100 */ None, /* 001010101 */ None, /* 001010110 */ None,
    /* 001010111 */ None, /* 001011000 */ None, /* 001011001 */ None,
    /* 001011010 */ None, /* 001011011 */ None, /* 001011100 */ None,
    /* 001011101 */ None, /* 001011110 */ None, /* 001011111 */ None,
    /* 001100000 */ None, /* 001100001 */ None, /* 001100010 */ None,
    /* 001100011 */ None, /* 001100100 */ None, /* 001100101 */ None,
    /* 001100110 */ None, /* 001100111 */ None, /* 001101000 */ None,
    /* 001101001 */ None, /* 001101010 */ None, /* 001101011 */ None,
    /* 001101100 */ None, /* 001101101 */ None, /* 001101110 */ None,
    /* 001101111 */ None, /* 001110000 */ None, /* 001110001 */ None,
    /* 001110010 */ None, /* 001110011 */ None, /* 00110100 */ None,
    /* 001110101 */ None, /* 001110110 */ None, /* 001110111 */ None,
    /* 001111000 */ None, /* 001111001 */ None, /* 001111010 */ None,
    /* 001111011 */ None, /* 001111100 */ None, /* 001111101 */ None,
    /* 001111110 */ None, /* 001111111 */ None,
];
