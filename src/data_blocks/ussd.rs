use crate::datatypes::{charging_fields::*, primitives::BcdTimestamp};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct USSD {
    pub served_imsi: String,
    pub served_imei: String,
    pub served_number_ton: String,
    pub served_number: String,
    pub initiator: String,
    pub action: String,
    pub start_time: String,
    pub end_time: String,
    pub cause_for_termination: String,
    pub served_subs_lac: String,
    pub served_subs_ci: String,
    pub number_of_transactions: String,
    pub service_code: String,
    pub radio_network_type: String,
}
impl USSD {
    pub fn new(bytes: &[u8]) -> Self {
            let served_imsi = IMSI::new(&bytes[25..33]).value;     //   C(  8)        25
            let served_imei = IMEI::new(&bytes[33..41]).value;     //   C(  8)        33
            let served_number_ton = TON::new(bytes[41]).value;       //   C(  1)        41
            let served_number = NUMBER::new(&bytes[42..52]).value;       //   C( 10)        42
            let initiator = Initiator::new(bytes[52]).value;       //   C(  1)        52
            let action = Action::new(bytes[53]).value;      //   C(  1)        53
            let start_time = BcdTimestamp::new(&bytes[54..61]).value;      //   C(  7)        54
            let end_time = BcdTimestamp::new(&bytes[61..68]).value;        //   C(  7)        61
            let cause_for_termination = CauseForTermination::new(&bytes[68..72]).value;       //  DW(  1)        68
            let served_subs_lac = LAC::new(&bytes[72..74]).value;     //   W(  1)        72
            let served_subs_ci = CI::new(&bytes[74..76]).value;      //   W(  1)        74
            let number_of_transactions = NumberOfTransactions::new(bytes[76]).value;      // BCD(  1)        76
            let service_code = ServiceCode::new(&bytes[77..87]).value;        //   C( 10)        77
            let radio_network_type = RadioNetworkType::new(bytes[87]).value;      //   C(  1)        87
        Self {
            served_imsi,
            served_imei,
            served_number_ton,
            served_number,
            initiator,
            action,
            start_time,
            end_time,
            cause_for_termination,
            served_subs_lac,
            served_subs_ci,
            number_of_transactions,
            service_code,
            radio_network_type,
        }
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

// record_length                                   W(  1)         0
// record_type                                   BCD(  1)         2
// record_number                                 BCD(  4)         3
// record_status                                   C(  1)         7
// check_sum                                       W(  1)         8
// call_reference                                  C(  5)        10
// exchange_id                                     C( 10)        15
                                                                                                                            
// DATA:
// FIELD NAME                                   DATA TYPE  POSITION

// served_imsi                                     C(  8)        25
// served_imei                                     C(  8)        33
// served_number_ton                               C(  1)        41
// served_number                                   C( 10)        42
// initiator                                       C(  1)        52
// action                                          C(  1)        53
// start_time                                      C(  7)        54
// end_time                                        C(  7)        61
// cause_for_termination                          DW(  1)        68
// served_subs_lac                                 W(  1)        72
// served_subs_ci                                  W(  1)        74
// number_of_transactions                        BCD(  1)        76
// service_code                                    C( 10)        77
// radio_network_type                              C(  1)        87
    