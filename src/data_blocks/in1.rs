use crate::datatypes::charging_fields::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IN1 {
    pub in_record_number: String,
    pub in_data: String,
    pub leg_call_reference: String,
    pub in_channel_allocated_time: String,
    pub in_data_length: String,
    pub call_reference_time: String,
    pub protocol_identification: String,
}
impl IN1 {
    pub fn new(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 109 {
            return Err("IN1: insufficient data".into());
        }
        let in_record_number = InRecordNumber::new(&bytes[25]).value; //BCD(  1)        25
        let in_data = InData::new(&bytes[26..87]).value; //  C( 61)        26
        let leg_call_reference = CallReference::new(&bytes[87..92]).value; //  C(  5)        87
        let in_channel_allocated_time = InChannelAllocatedTime::new(&bytes[92..99]).value; //  C(  7)        92
        let in_data_length = InDataLength::new(&bytes[99..101]).value; //  W(  1)        99
        let call_reference_time = CallReferenceTime::new(&bytes[101..108]).value; //  C(  7)       101
        let protocol_identification = ProtocolIdentification::new(bytes[108]).value; //  C(  1)       108
        Ok(Self {
            in_record_number,
            in_data,
            leg_call_reference,
            in_channel_allocated_time,
            in_data_length,
            call_reference_time,
            protocol_identification,
        })
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

// DATA:
// FIELD NAME                                   DATA TYPE  POSITION
// in_record_number                              BCD(  1)        25
// in_data                                         C( 61)        26
// leg_call_reference                              C(  5)        87
// in_channel_allocated_time                       C(  7)        92
// in_data_length                                  W(  1)        99
// call_reference_time                             C(  7)       101
// protocol_identification                         C(  1)       108
