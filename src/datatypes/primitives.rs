use std::fmt::{self};

#[derive(Debug, Clone, Copy)]
pub struct BCD {
    value: u8,
}

impl BCD {
    pub fn as_dec(&self) -> u8 {
        let low = self.value & 0x0F;
        let high = (self.value >> 4) & 0x0F;
        high * 10 + low
    }
}
impl fmt::Display for BCD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02X}",
            self.as_dec()
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BCDWord {
    high: BCD,
    low: BCD,
}

impl BCDWord {
    pub fn as_dec(&self) -> u16 {
        self.high.as_dec() as u16 * 100 + self.low.as_dec() as u16
    }

}
impl fmt::Display for BCDWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02X}{:02X})",
            self.high.as_dec(), self.low.as_dec()
        )
    }
}

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
