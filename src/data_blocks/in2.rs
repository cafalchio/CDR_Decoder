use crate::datatypes::charging_fields::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IN2 {
    pub in_record_number: String,
    pub in_data: String,
    pub leg_call_reference: String,
    pub in_channel_allocated_time: String,
    pub in_data_length: String,
    pub call_reference_time: String,
    pub protocol_identification: String,
}
impl IN2 {
    pub fn new(bytes: &[u8]) -> Self {
        let in_record_number = InRecordNumber::new(&bytes[25]).value;
        let in_data = InData::new(&bytes[26..43]).value;
        let leg_call_reference = CallReference::new(&bytes[43..48]).value;
        let in_channel_allocated_time = InChannelAllocatedTime::new(&bytes[48..55]).value;
        let in_data_length = InDataLength::new(&bytes[55..57]).value;
        let call_reference_time = CallReferenceTime::new(&bytes[57..64]).value;
        let protocol_identification = ProtocolIdentification::new(bytes[64]).value;
        Self {  
            in_record_number,
            in_data,
            leg_call_reference,
            in_channel_allocated_time,
            in_data_length,
            call_reference_time,
            protocol_identification,
        }
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

// FORMAT TYPE:      18
// MESSAGE NUMBER:   efc6
// FORMAT TYPE NAME: IN2 Intelligent Network CDR2
// RECORD LENGTH:    65

// FIELD NAME                                   DATA TYPE  POSITION
// in_record_number                              BCD(  1)        25
// in_data                                         C( 17)        26
// leg_call_reference                              C(  5)        43
// in_channel_allocated_time                       C(  7)        48
// in_data_length                                  W(  1)        55
// call_reference_time                             C(  7)        57
// protocol_identification                         C(  1)        64
