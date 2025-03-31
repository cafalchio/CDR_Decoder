
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

impl BCD {
    pub fn as_dec(&self) -> u8 {
        let low = self.value & 0b00001111;
        let high = (self.value & 0b11110000) >> 4;
        low + (high * 10)
    }
}
pub struct BCDWord {
    value: [BCD; 2],
}
impl BCDWord {
    pub fn as_dec(&self) -> u16 {
        let high = self.value[0].as_dec() as u16;
        let low = self.value[1].as_dec() as u16;
        high * 100 + low
    }
}

pub struct BCDDuword {
    value: [BCDWord; 2],
}
impl BCDDuword {
    pub fn as_dec(&self) -> u32 {
        let high = self.value[0].as_dec() as u32;
        let low = self.value[1].as_dec() as u32;
        high * 10000 + low
    }
}
pub struct Timestamp {
    value: [u8; 5],
}
