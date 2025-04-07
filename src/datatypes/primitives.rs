#[derive(Debug, Clone, Copy)]
pub struct HByte {
    pub value: u32,
}
impl HByte {
    pub fn new(value: &[u8]) -> HByte {
        let hbyte = value[0] as u32;
        HByte { value: hbyte }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HWord {
    pub value: u32,
}
impl HWord {
    pub fn new(bytes: &[u8]) -> HWord {
        let hbytes = u16::from_le_bytes([bytes[0], bytes[1]]);
        HWord {
            value: hbytes.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HDWord {
    pub value: u32,
}
impl HDWord {
    pub fn new(bytes: &[u8]) -> HDWord {
        let hdword: u32 = u32::from_be_bytes([bytes[0],bytes[1], bytes[2], bytes[3]]);
        HDWord { 
            value: hdword
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Word {
    // hexadecimal word
    pub value: u32,
}
impl Word {
    pub fn new(value: [u8; 2]) -> Word {
        let word: u16 = u16::from_be_bytes(value);
        Word { value: word as u32 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BCD {
    pub value: u32,
}
impl BCD {
    pub fn new(value: &u8) -> BCD {
        let low = value & 0b0000_1111;
        let high = (value >> 4) & 0b0000_1111;
        let decimal = high * 10 + low;
        BCD {
            value: decimal as u32,
        }
    }
}




pub struct BCDWord {
    pub value: u32,
}
impl BCDWord {
    pub fn new(bytes: &[u8]) -> BCDWord {
        let high = BCD::new(&bytes[1]);
        let low = BCD::new(&bytes[0]);
        BCDWord {
            value: (high.value * 100 + low.value),
        }
    }
}

pub struct BCD2uword {
    pub value: u32,
}
impl BCD2uword {
    pub fn new(bytes: &[u8]) -> BCD2uword {
        let high = BCDWord::new(&bytes[2..4]);
        let low = BCDWord::new(&bytes[0..2]);
        let decimal = high.value * 10000 + low.value;
        BCD2uword { value: decimal }
    }
}
pub struct BcdTimestamp {
    pub value: String,
}

impl BcdTimestamp {
    pub fn new(bytes: &[u8]) -> BcdTimestamp {
        let year = BCDWord::new(&bytes[5..7]).value;
        let month = BCD::new(&bytes[4]).value;
        let day = BCD::new(&bytes[3]).value;
        let hour = BCD::new(&bytes[2]).value;
        let minute = BCD::new(&bytes[1]).value;
        let second = BCD::new(&bytes[0]).value;
        let timestamp = format!(
            "{:02}/{:02}/{:04} {:02}:{:02}:{:02}",
            day, month, year, hour, minute, second
        );
        BcdTimestamp { value: timestamp }
    }
}

// 3/4 bcd bytes (except PNI)
// The numerical value in bcd form.

// 8 - 16 bcd/hex bytes (and also PNI)
// Used in IMSI, IMEI, subscriber numbers, and exchange ID.
