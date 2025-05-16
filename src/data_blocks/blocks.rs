#![allow(dead_code)]
#![allow(unused_variables)]

use crate::data_blocks::{
    doc::DOC, forw::FORW, header::Header, hlri::HLRI, in1::IN1, in2::IN2, in3::IN3, in4::IN4,
    loca::LOCA, moc::Moc, ptc::PTC, roam::ROAM, smmf::SMMF, smmo::SMMO, smmt::SMMT, sups::SUPS,
    trailer::Trailer, uca::UCA, pbxt::PBXT
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Blocks {
    Header(Header),
    Loca(LOCA), // Location update
    Hlri(HLRI), //HLR interrogation
    Smmo(SMMO), //"Short Message service (point-to-point), Mobile-originated"
    Smmt(SMMT),
    Smmf(SMMF),
    In1(IN1),
    In2(IN2),
    In3(IN3),
    In4(IN4),
    Forw(FORW),
    Uca(UCA),
    Sups(SUPS),
    Moc(Moc),
    Ptc(PTC),
    Doc(DOC),
    Roam(ROAM),
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
            "Short message service (point-to-point), mobile-terminated" => {
                Some(Blocks::Smmt(SMMT::new(data)))
            }
            "Intelligent network data 1" => Some(Blocks::In1(IN1::new(data))),
            "Intelligent network data 2" => Some(Blocks::In2(IN2::new(data))),
            "Intelligent network data 3" => Some(Blocks::In3(IN3::new(data))),
            "Intelligent network data 4" => Some(Blocks::In4(IN4::new(data))),
            "Forwarded call" => Some(Blocks::Forw(FORW::new(data))),
            "Unsuccessful call attempt" => Some(Blocks::Uca(UCA::new(data))),
            "Supplementary service" => Some(Blocks::Sups(SUPS::new(data))),
            "Mobile-originated call" => Some(Blocks::Moc(Moc::new(data))),
            "PSTN-terminated call" => Some(Blocks::Ptc(PTC::new(data))),
            "Device-originated Call" => Some(Blocks::Doc(DOC::new(data))),
            "Call to a Roaming Subscriber" => Some(Blocks::Roam(ROAM::new(data))),
            "PBX-terminated Call" => Some(Blocks::Roam(ROAM::new(data))),
            "Trailer" => Some(Blocks::Trailer(Trailer::new(data))),
            _ => None,
        }
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}
