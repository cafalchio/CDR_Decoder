#![allow(dead_code)]
#![allow(unused_variables)]

use crate::data_blocks::{
    forw::FORW, header::Header, hlri::HLRI, in2::IN2, loca::LOCA, smmo::SMMO, uca::UCA,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Blocks {
    Header(Header),
    Loca(LOCA), // Location update
    Hlri(HLRI), //HLR interrogation
    Smmo(SMMO), //"Short Message service (point-to-point), Mobile-originated"
    In2(IN2),
    Forw(FORW),
    Uca(UCA),
}

impl Blocks {
    pub fn new(name: &str, data: &[u8]) -> Option<Self> {
        match name {
            "Location update" => Some(Blocks::Loca(LOCA::new(data))), // 25 - 85 location update
            "HLR interrogation" => Some(Blocks::Hlri(HLRI::new(data))),
            "Short message service (point-to-point), mobile-originated" => {
                Some(Blocks::Smmo(SMMO::new(data)))
            }
            "Intelligent network data 2" => Some(Blocks::In2(IN2::new(data))),
            "Forwarded call" => Some(Blocks::Forw(FORW::new(data))),
            "Unsuccessful call attempt" => Some(Blocks::Uca(UCA::new(data))),
            _ => None,
        }
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}
