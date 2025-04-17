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
                                                                                                                            
                                                                                                                            
                                                                                                                            
                                                                                                                            
                                                                                                                            
                                                                                                                            
                                                                                                                            
                                                                                                                            
                                                                                                                            
// DATA:
// FIELD NAME                                   DATA TYPE  POSITION

// in_record_number                              BCD(  1)        25
// in_data                                         C(310)        26
// leg_call_reference                              C(  5)       336
// in_channel_allocated_time                       C(  7)       341
// in_data_length                                  W(  1)       348
// call_reference_time                             C(  7)       350
// protocol_identification                         C(  1)       357
                                                                                                                            

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
