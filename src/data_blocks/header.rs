use crate::datatypes::primitives::*;
use crate::datatypes::mixed::*;

pub struct Header {
    pub record_length: u32,     // W(1) at offset 0
    pub record_type: String,    // BCD(1) at offset 2
    pub record_number: u32,     // BCD(4) at offset 3
    pub record_status: String,  // C(1) at offset 7
    pub check_sum: u32,         // W(1) at offset 8
    pub call_reference: String, // C(5) at offset 10
    pub exchange_id: String,    // C(10) at offset 15
}
impl Header {
    pub fn new(bytes:&[u8]) -> Header {
        let record_length = HWord::new(&bytes[0..2]).value as u32;
        let record_type = RecordType::try_from(bytes[2]).unwrap().to_string();
        let record_number = BCD2uword::new(&bytes[3..7]).value; 
        let mut record_status: String = "".to_string();  
        if (record_type != "Header".to_string()) & (record_type != "Trailer".to_string()) {
            record_status = RecordStatus::try_from(bytes[7]).unwrap().to_string();
        }
        let check_sum = HWord::new(&bytes[8..10]).value;
        let call_reference = CallReference::new(&bytes[10..15]).value;
        let exchange_id = ExchangeId::new(&bytes[15..25]).value;
        Header {
            record_length,
            record_type,
            record_number,
            record_status,
            check_sum,
            call_reference,
            exchange_id,
        }
    }
}