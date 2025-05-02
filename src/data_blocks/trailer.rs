use crate::datatypes::charging_fields::*;
use crate::datatypes::charging_fields_impl::decode_bcds;
use crate::datatypes::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub struct Trailer {
    pub record_length: u16,  // W(1) at offset 0
    pub record_type: String, // BCD(1) at offset 2
    pub exchange_id: String, // C(1) at offset 7
    pub end_time: String,
    pub last_record_number: String,
}
impl Trailer {
    pub fn new(bytes: &[u8]) -> Self {
        let record_length = HWord::new(&bytes[0..2]).value as u16;
        let record_type = RecordType::new(bytes[2]).value;
        let exchange_id = ExchangeId::new(&bytes[3..13]).value;
        let end_time = decode_bcds(&bytes[13..20]);
        let last_record_number = LastRecordNumber::new(&bytes[20..24]).value;
        Self {
            record_length,
            record_type,
            exchange_id,
            end_time,
            last_record_number,
        }
    }
}
