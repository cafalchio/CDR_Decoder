use crate::data_blocks::{header::Header, loca::LOCA};

pub enum Blocks {
    Header(Header),
    Loca(LOCA),
}

impl Blocks {
    pub fn new(length: usize, data: &[u8]) -> Option<Self> {
        match length {
            25 => Some(Blocks::Header(Header::new(data))),
            60 => Some(Blocks::Loca(LOCA::new(data))),
            _ => None,
        }
    }
}
