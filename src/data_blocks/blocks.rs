#![allow(dead_code)]
#![allow(unused_variables)]

use crate::data_blocks::{
    doc::DOC, forw::FORW, header::Header, hlri::HLRI, in1::IN1, in2::IN2, in3::IN3, in4::IN4,
    lcs::LCS, loca::LOCA, moc::Moc, pbxo::PBXO, pbxt::PBXT, ptc::PTC, roam::ROAM, smmf::SMMF,
    smmo::SMMO, smmt::SMMT, sups::SUPS, trailer::Trailer, uca::UCA,
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
    Pbxt(PBXT),
    Lcs(LCS),
    Pbxo(PBXO),
    Trailer(Trailer),
}

impl Blocks {
    pub fn new(name: &str, data: &[u8]) -> Result<Self, String> {
        match name {
            "Location update" => LOCA::new(data).map(Blocks::Loca),
            "HLR interrogation" => HLRI::new(data).map(Blocks::Hlri),
            "Short message service (point-to-point), mobile-originated" => {
                SMMO::new(data).map(Blocks::Smmo)
            }
            "Short message service (point-to-point), mobile-terminated" => {
                SMMT::new(data).map(Blocks::Smmt)
            }
            "Short message service (point-to-point), failure" => {
                SMMF::new(data).map(Blocks::Smmf)
            }
            "Intelligent network data 1" => IN1::new(data).map(Blocks::In1),
            "Intelligent network data 2" => IN2::new(data).map(Blocks::In2),
            "Intelligent network data 3" => IN3::new(data).map(Blocks::In3),
            "Intelligent network data 4" => IN4::new(data).map(Blocks::In4),
            "Forwarded call" => FORW::new(data).map(Blocks::Forw),
            "Unsuccessful call attempt" => UCA::new(data).map(Blocks::Uca),
            "Supplementary service" => SUPS::new(data).map(Blocks::Sups),
            "Mobile-originated call" => Moc::new(data).map(Blocks::Moc),
            "PSTN-terminated call" => PTC::new(data).map(Blocks::Ptc),
            "Device-originated Call" => DOC::new(data).map(Blocks::Doc),
            "Call to a Roaming Subscriber" => ROAM::new(data).map(Blocks::Roam),
            "PBX-terminated Call" => PBXT::new(data).map(Blocks::Pbxt),
            "PBX-originated Cal" => PBXO::new(data).map(Blocks::Pbxo),
            "Location Services" => LCS::new(data).map(Blocks::Lcs),
            "Trailer" => Trailer::new(data).map(Blocks::Trailer),
            _ => Err(format!("Unknown block name: {}", name)),
        }
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}
