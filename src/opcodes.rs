

pub enum OpMasks {
    Family = 0b11100000,
    Type = 0b00011100,
    AddrMode = 0b00000011,
    Invalid,
}

impl From<u8> for OpMasks {
    fn from(a : u8) -> Self {
        match a {
            0b11100000 => OpMasks::Family,
            0b00011100 => OpMasks::Type,
            0b00000011 => OpMasks::AddrMode,
            _ => OpMasks::Invalid,
        }
    }
}

pub enum OpFamily {
    StackOp = 0b11100000,
    Invalid,
}

impl From<u8> for OpFamily {
    fn from(a : u8) -> Self {
        match a {
            0b11100000 => OpFamily::StackOp,
            _ => OpFamily::Invalid,
        }
    }
}

pub enum OpCodes {
    PushImm = 0b11111111,
}
