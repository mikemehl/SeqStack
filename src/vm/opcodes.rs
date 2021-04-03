/// Submodule encapsulating opcode enums.

pub enum OpMasks {
    Family = 0b11100000,
    Type = 0b00011100,
    AddrMode = 0b00000011,
    Invalid = 0b00000000,
}

pub enum OpFamily {
    StackOp,
    Invalid,
}

impl From<u8> for OpFamily {
    fn from(a : u8) -> Self {
        let a_masked = a & OpMasks::Family as u8;
        match a_masked {
            0b11100000 => OpFamily::StackOp,
            _ => OpFamily::Invalid,
        }
    }
}

pub enum OpAddrMode {
    Immediate = 0b00000011,
    IndexStack = 0b00000010,
    IndexImmediate = 0b00000001,
    Stack = 0b00000000,
    Invalid = 0b11111111,
}

impl From<u8> for OpAddrMode {
    fn from(a : u8) -> Self {
        let a_masked = a & OpMasks::AddrMode as u8;
        match a_masked {
            0b00000011 => OpAddrMode::Immediate,
            0b00000010 => OpAddrMode::IndexStack,
            0b00000001 => OpAddrMode::IndexImmediate,
            0b00000000 => OpAddrMode::Stack,
            _ => OpAddrMode::Invalid,
        }
    }
}

// Stack family specific enums
pub enum StackOpTypes {
    Push  = 0b00011100,
    Store = 0b00000000,
    Pop   = 0b00011000,
    Dup   = 0b00010100,
    Invalid = 0b11111111,
}

impl From<u8> for StackOpTypes {
    fn from(a : u8) -> Self {
        let a_masked = a & OpMasks::Type as u8;
        match a_masked {
            0b00011100 => StackOpTypes::Push,
            0b00000000 => StackOpTypes::Store,
            0b00011000 => StackOpTypes::Pop,
            0b00010100 => StackOpTypes::Dup,
            _ => StackOpTypes::Invalid,
        }
    }
}

// All Opcodes
pub enum OpCodes {
    PushImm     = 0b11111111,
    PushIndStk  = 0b11111110,
    PushIndImm  = 0b11111101,
    PushStk     = 0b11111100,
    StoreImm    = 0b11100011,
    StoreIndStk = 0b11100010,
    StoreIndImm = 0b11100001,
    StoreStk    = 0b11100000,
    Pop         = 0b11111000,
    Dup         = 0b11110100,
    Rot         = 0b11110000,
}


