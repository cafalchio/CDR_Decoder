use std::fmt::{self};

#[derive(Debug, Clone, Copy)]


pub struct Word {
    // hexadecimal word
    pub value: u16,
}
impl Word {
    fn new(value: [u8; 2]) -> Word {
        let word: u16 = u16::from_be_bytes(value);
        Word { value: word }
    }
}


pub struct BCD {
    pub value: u8,
}

impl BCD {
    fn new(value: u8) -> BCD {
        let low: u8 = value & 0x0F;
        let high: u8 = (value >> 4) & 0x0F;
        let _decimal: u8 = high * 10 + low;
        BCD { value: _decimal }
    }
}

pub struct BCDWord {
    pub value: u8,
}

impl BCDWord {
    pub fn new(value: [BCD; 2]) -> BCDWord {
        let dec = value[1].value * 100 + value[0].value;
        BCDWord { value: dec }
    }
}

// Work in progress ----------------------------------------------------


#[derive(Debug, Clone, Copy)]
pub struct BcdTimestamp {
    sec: BCD,
    min: BCD,
    hour: BCD,
    day: BCD,
    month: BCD,
    year: BCDWord,
}

#[derive(Debug, Clone, Copy)]
pub struct BCD2uword {
    low : BCDWord,
    high: BCDWord
}
impl fmt::Display for BCD2uword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02X}{:02X}",
            self.high.as_dec(), self.low.as_dec()
        )
    }
}

impl BcdTimestamp {
    pub fn from_hex_dump(hex_dump: [u8; 7]) -> Self {
        Self {
            sec: BCD { value: hex_dump[0] },
            min: BCD { value: hex_dump[1] },
            hour: BCD { value: hex_dump[2] },
            day: BCD { value: hex_dump[3] },
            month: BCD { value: hex_dump[4] },
            year: BCDWord {
                low: BCD { value: hex_dump[5] },
                high: BCD { value: hex_dump[6] },
            },
        }
    }

    pub fn to_bcd_string(&self) -> String {
        format!(
            "{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}H",
            self.year.high.value, self.year.low.value, self.month.value, self.day.value, self.hour.value, self.min.value, self.sec.value
        )
    }
}

impl fmt::Display for BcdTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Time: {:02}:{:02}:{:02} Date: {:02}/{:02}/{}",
            self.hour.as_dec(), self.min.as_dec(), self.sec.as_dec(), self.day.as_dec(), self.month.as_dec(), self.year.as_dec()
        )
    }
}
