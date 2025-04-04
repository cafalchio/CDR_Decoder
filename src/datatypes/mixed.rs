use crate::datatypes::primitives::{BCD, BCD2uword};

#[derive(Debug)]
pub struct Header {
    pub record_length: u16,     // W(1) at offset 0
    pub record_type: BCD,       // BCD(1) at offset 2
    pub record_number: BCD2uword, // BCD(4) at offset 3
    pub record_status: u8,      // C(1) at offset 7
    pub check_sum: u16,         // W(1) at offset 8
    pub call_reference: [u8; 5],  // C(5) at offset 10
    pub exchange_id: [u8; 10],    // C(10) at offset 15
}
