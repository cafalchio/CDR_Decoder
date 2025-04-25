

use crate::datatypes::charging_fields::*;
use crate::datatypes::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Trailer {
    pub record_length: u32,     // W(1) at offset 0
    pub record_type: String,    // BCD(1) at offset 2
    pub exchange_id: String,     // BCD(4) at offset 3
    pub end_time: String,          // C(1) at offset 7
    pub last_record_number: String // W(1) at offset 8
}
impl Trailer {
    pub fn new(bytes: &[u8]) -> Self {
        let record_length = HWord::new(&bytes[0..2]).value as u32;
        let record_type = RecordType::new(bytes[2]).value;
        let exchange_id = format!("{}", BCD2uword::new(&bytes[3..7]).value);
        let end_time = format!("{}", HByte::new(bytes[7]).value);
        let last_record_number = format!("{}", HWord::new(&bytes[8..10]).value);
        Self {
            record_length,
            record_type,
            exchange_id,
            end_time,
            last_record_number,  
        }
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

// Block Trailer
// dec lgth hex
// 0 Tra_record_length 2 0
// 2 Tra_record_type 1 2
// 3 Tra_exchange_id 10 3
// 13 Tra_end_time 7 D
// 20 Tra_last_record_number 4 14