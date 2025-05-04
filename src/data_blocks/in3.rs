use crate::datatypes::charging_fields::*;

// FORMAT TYPE:      19
// MESSAGE NUMBER:   efc6
// FORMAT TYPE NAME: IN3
// RECORD LENGTH:    358

// HEADER:
// FIELD NAME                                   DATA TYPE  POSITION

// record_length                                   W(  1)         0
// record_type                                   BCD(  1)         2
// record_number                                 BCD(  4)         3
// record_status                                   C(  1)         7
// check_sum                                       W(  1)         8
// call_reference                                  C(  5)        10
// exchange_id                                     C( 10)        15
                                                                                                                            
use crate::datatypes::charging_fields::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IN3 {
    pub in_record_number: String,
    pub in_data: String,
    pub leg_call_reference: String,
    pub in_channel_allocated_time: String,
    pub in_data_length: String,
    pub call_reference_time: String,
    pub protocol_identification: String,
}
impl IN3 {
    pub fn new(bytes: &[u8]) -> Self {
        let in_record_number = InRecordNumber::new(&bytes[25]).value;                               //BCD(  1)        25
        let in_data = InData::new(&bytes[26..336]).value;                                            //  C(310)        26
        let leg_call_reference = CallReference::new(&bytes[336..341]).value;                          //  C(  5)       336
        let in_channel_allocated_time = InChannelAllocatedTime::new(&bytes[341..348]).value;          //  C(  7)       341
        let in_data_length = InDataLength::new(&bytes[348..350]).value;                               //  W(  1)       348
        let call_reference_time = CallReferenceTime::new(&bytes[350..357]).value;                     //  C(  7)       350
        let protocol_identification = ProtocolIdentification::new(bytes[357]).value;                 //  C(  1)       357
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
                                                                                                                            
                                                                                                                            
                                                                                                                            
                                                                                                                            
                                                                                                                            
                                                                                                                            
                                                                                                                            
// DATA:
// FIELD NAME                                   DATA TYPE  POSITION

// in_record_number                              BCD(  1)        25
// in_data                                         C(310)        26
// leg_call_reference                              C(  5)       336
// in_channel_allocated_time                       C(  7)       341
// in_data_length                                  W(  1)       348
// call_reference_time                             C(  7)       350
// protocol_identification                         C(  1)       357
                                                                                                                            
