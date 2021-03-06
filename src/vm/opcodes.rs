/// Submodule encapsulating opcode enums.

#[allow(clippy::unusual_byte_groupings)]
pub enum OpMasks {
    Family = 0b111_00000,
    Type = 0b000_111_00,
    AddrMode = 0b000000_11,
    Invalid = 0b00000000,
}

#[allow(clippy::unusual_byte_groupings)]
pub enum OpFamily {
    StackOp,
    ArithmeticOp,
    BitManipOp,
    PortOp,
    Invalid,
}

#[allow(clippy::unusual_byte_groupings)]
impl From<u8> for OpFamily {
    fn from(a: u8) -> Self {
        let a_masked = a & OpMasks::Family as u8;
        match a_masked {
            0b111_000_00 => OpFamily::StackOp,
            0b110_000_00 => OpFamily::ArithmeticOp,
            0b101_000_00 => OpFamily::BitManipOp,
            0b100_000_00 => OpFamily::PortOp,
            _ => OpFamily::Invalid,
        }
    }
}

#[allow(clippy::unusual_byte_groupings)]
pub enum OpAddrMode {
    Immediate = 0b000_000_11,
    IndexStack = 0b000_000_10,
    IndexImmediate = 0b000_000_01,
    Stack = 0b000_000_00,
    Invalid = 0b11111111,
}

#[allow(clippy::unusual_byte_groupings)]
impl From<u8> for OpAddrMode {
    fn from(a: u8) -> Self {
        let a_masked = a & OpMasks::AddrMode as u8;
        match a_masked {
            0b000_000_11 => OpAddrMode::Immediate,
            0b000_000_10 => OpAddrMode::IndexStack,
            0b000_000_01 => OpAddrMode::IndexImmediate,
            0b000_000_00 => OpAddrMode::Stack,
            _ => OpAddrMode::Invalid,
        }
    }
}

// Stack family specific enums
#[allow(clippy::unusual_byte_groupings)]
pub enum StackOpTypes {
    Push = 0b000_111_00,
    Store = 0b000_000_00,
    Pop = 0b000_110_00,
    Dup = 0b000_101_00,
    Rot = 0b000_100_00,
    Swap = 0b000_011_00,
    MovToRts = 0b000_010_00,
    MovFromRts = 0b000_001_00,
    Invalid = 0b11111111,
}

#[allow(clippy::unusual_byte_groupings)]
impl From<u8> for StackOpTypes {
    fn from(a: u8) -> Self {
        let a_masked = a & OpMasks::Type as u8;
        match a_masked {
            0b000_111_00 => StackOpTypes::Push,
            0b000_000_00 => StackOpTypes::Store,
            0b000_110_00 => StackOpTypes::Pop,
            0b000_101_00 => StackOpTypes::Dup,
            0b000_100_00 => StackOpTypes::Rot,
            0b000_011_00 => StackOpTypes::Swap,
            0b000_010_00 => StackOpTypes::MovToRts,
            0b000_001_00 => StackOpTypes::MovFromRts,
            _ => StackOpTypes::Invalid,
        }
    }
}

// Arithmetic family specific enums
#[allow(clippy::unusual_byte_groupings)]
pub enum ArithmeticOpTypes {
    Add = 0b000_110_00,
    Sub = 0b000_100_00,
    Mul = 0b000_010_00,
    Div = 0b000_000_00,
    Invalid = 0b11111111,
}

#[allow(clippy::unusual_byte_groupings)]
impl From<u8> for ArithmeticOpTypes {
    fn from(a: u8) -> Self {
        let a_masked = a & OpMasks::Type as u8;
        match a_masked {
            0b000_11_000 => ArithmeticOpTypes::Add,
            0b000_10_000 => ArithmeticOpTypes::Sub,
            0b000_01_000 => ArithmeticOpTypes::Mul,
            0b000_00_000 => ArithmeticOpTypes::Div,
            _ => ArithmeticOpTypes::Invalid,
        }
    }
}

// Bit manipulation family specific enums
#[allow(clippy::unusual_byte_groupings)]
pub enum BitOpTypes {
    Shl = 0b000_111_00,
    Shr = 0b000_110_00,
    Rotl = 0b000_101_00,
    Rotr = 0b000_100_00,
    And = 0b000_011_00,
    Or = 0b101_010_00,
    Xor = 0b000_001_00,
    Not = 0b000_000_00,
    Invalid = 0b11111111,
}

#[allow(clippy::unusual_byte_groupings)]
impl From<u8> for BitOpTypes {
    fn from(a: u8) -> Self {
        let a_masked = a & OpMasks::Type as u8;
        match a_masked {
            0b000_111_00 => BitOpTypes::Shl,
            0b000_110_00 => BitOpTypes::Shr,
            0b000_101_00 => BitOpTypes::Rotl,
            0b000_100_00 => BitOpTypes::Rotr,
            0b000_011_00 => BitOpTypes::And,
            0b000_010_00 => BitOpTypes::Or,
            0b000_001_00 => BitOpTypes::Xor,
            0b000_000_00 => BitOpTypes::Not,
            _ => BitOpTypes::Invalid,
        }
    }
}

// Port manipulation family specific enums
#[allow(clippy::unusual_byte_groupings)]
pub enum PortOpTypes {
    Push = 0b000_11_000,
    Invalid = 0b11111111,
}

#[allow(clippy::unusual_byte_groupings)]
impl From<u8> for PortOpTypes {
    fn from(a: u8) -> Self {
        let a_masked = a & OpMasks::Type as u8;
        match a_masked {
         0b000_11_000 => PortOpTypes::Push,
            _ => PortOpTypes::Invalid,
        }
    }
}

// All Opcodes
#[allow(clippy::unusual_byte_groupings)]
pub enum OpCodes {
    PushImm = 0b111_111_11,
    PushIndStk = 0b111_111_10,
    PushIndImm = 0b111_111_01,
    PushStk = 0b111_111_00,
    StoreImm = 0b111_000_11,
    StoreIndStk = 0b111_000_10,
    StoreIndImm = 0b111_000_01,
    StoreStk = 0b111_000_00,
    Pop = 0b111_110_00,
    Dup = 0b111_101_00,
    Rot = 0b111_100_00,
    Swap = 0b111_011_00,
    MovToRts = 0b111_010_00,
    MovFromRts = 0b111_001_00,
    Add = 0b110_11_000,
    Sub = 0b110_10_000,
    Mul = 0b110_01_000,
    Div = 0b110_00_000,
    Shl = 0b101_111_00,
    Shr = 0b101_110_00,
    Rotl = 0b101_101_00,
    Rotr = 0b101_100_00,
    And = 0b101_011_00,
    Or = 0b101_010_00,
    Xor = 0b101_001_00,
    Not = 0b101_000_00,
}
