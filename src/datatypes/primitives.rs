
#[allow(dead_code)] 
pub struct Hex {
    value: u8,
}

pub struct Hex2 {
    value: [u8; 2], 
}


pub struct HexWord {
    value: [u8; 4], 
}


pub struct HexDuword {
    value: [u8; 4], 
}

pub struct BCD {
    value: u8,
}

pub struct BCDWord {
    value: [u8; 2], 
}

pub struct BCDDuword {
    value: [u8; 4], 
}

pub struct Timestamp {
    value: [u8; 5],
}