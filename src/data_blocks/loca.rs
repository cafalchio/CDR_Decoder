use crate::datatypes::charging_fields::*;
use crate::datatypes::charging_fields_impl::*;
use crate::datatypes::primitives::*;

// served_imsi                                     C(  8)        25
// subs_old_lac                                    W(  1)        33
// subs_old_ex_id                                  C( 10)        35
// subs_new_lac                                    W(  1)        45
// subs_new_ex_id                                  C( 10)        47
// charging_time                                   C(  7)        57
// served_number_ton                               C(  1)        64
// served_number                                   C( 12)        65
// call_reference_time                             C(  7)        77
// loc_up_indicator                                C(  1)        84
// number_of_in_records                          BCD(  1)        85
  
pub struct LOCA {
    pub served_imsi: String,
    pub subs_old_lac: String,
    pub subs_old_ex_id: String,
    pub subs_new_lac: String,
    pub subs_new_ex_id: String,
    pub charging_time: String,
    pub served_number_ton: String,
    pub served_number: String,
    pub call_reference_time: String,
    pub loc_up_indicator: String,
    pub number_of_in_records: String,
}

impl LOCA {
    pub fn new(bytes: &[u8]) -> Self {
        let served_imsi = IMSI::new(&bytes[25..33]).value;
        let subs_old_lac = LAC::new(&bytes[33..34]).value;
        let subs_old_ex_id = "".to_string();
        let subs_new_lac = "".to_string();
        let subs_new_ex_id = "".to_string();
        let charging_time = ChargingTime::new(&bytes[57..64]).value;
        let served_number_ton = "".to_string();
        let served_number = "".to_string();
        let call_reference_time = CallReferenceTime::new(&bytes[77..84]).value;
        let loc_up_indicator = "".to_string();
        let number_of_in_records = "".to_string();

        Self {
            served_imsi,
            subs_old_lac,
            subs_old_ex_id,
            subs_new_lac,
            subs_new_ex_id,
            charging_time,
            served_number_ton,
            served_number,
            call_reference_time,
            loc_up_indicator,
            number_of_in_records,
        }
    }
}
