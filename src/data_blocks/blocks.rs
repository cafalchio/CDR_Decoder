#![allow(dead_code)]
#![allow(unused_variables)]

use crate::data_blocks::{
    forw::FORW,
    header::Header,
    hlri::HLRI,
    in1::IN1,
    in2::IN2,
    in3::IN3,
    in4::IN4,
    loca::LOCA,
    smmo::SMMO,
    sups::SUPS,
    trailer::Trailer,
    uca::UCA,
    moc::Moc,
    // ptc::PTC,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Blocks {
    Header(Header),
    Loca(LOCA), // Location update
    Hlri(HLRI), //HLR interrogation
    Smmo(SMMO), //"Short Message service (point-to-point), Mobile-originated"
    In1(IN1),
    In2(IN2),
    In3(IN3),
    In4(IN4),
    Forw(FORW),
    Uca(UCA),
    Sups(SUPS),
    Moc(Moc),
    // Ptc(PTC),
    Trailer(Trailer),
}

impl Blocks {
    pub fn new(name: &str, data: &[u8]) -> Option<Self> {
        match name {
            "Location update" => Some(Blocks::Loca(LOCA::new(data))), // 25 - 85 location update
            "HLR interrogation" => Some(Blocks::Hlri(HLRI::new(data))),
            "Short message service (point-to-point), mobile-originated" => {
                Some(Blocks::Smmo(SMMO::new(data)))
            }
            "Intelligent network data 1" => Some(Blocks::In1(IN1::new(data))),
            "Intelligent network data 2" => Some(Blocks::In2(IN2::new(data))),
            "Intelligent network data 3" => Some(Blocks::In3(IN3::new(data))),
            "Intelligent network data 4" => Some(Blocks::In4(IN4::new(data))),
            "Forwarded call" => Some(Blocks::Forw(FORW::new(data))),
            "Unsuccessful call attempt" => Some(Blocks::Uca(UCA::new(data))),
            "Supplementary service" => Some(Blocks::Sups(SUPS::new(data))),
            "Mobile-originated call" => Some(Blocks::Moc(Moc::new(data))),
            // "PSTN-terminated call" => Some(Blocks::Ptc(PTC::new(data))),
            "Trailer" => Some(Blocks::Trailer(Trailer::new(data))),
            _ => None,
        }
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}
