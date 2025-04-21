use serde::{Deserialize, Serialize};
use std::fmt;

use crate::data_blocks::{header::Header, loca::LOCA};

#[derive(Serialize, Deserialize, Debug)]
pub enum Blocks {
    Header(Header),
    Loca(LOCA),
}

impl Blocks {
    pub fn new(name: &str, data: &[u8]) -> Option<Self> {
        match name {
            "Location update" => Some(Blocks::Loca(LOCA::new(data))), // 25 - 85 location update
            _ => None,
        }
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}
