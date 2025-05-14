use crate::datatypes::charging_fields::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct HLRI {
    pub called_imsi: String,
    pub called_number: String,
    pub routing_number: String,
    pub charging_time: String,
    pub number_of_forwardings: String,
    pub cause_for_termination: String,
}

impl HLRI {
    pub fn new(bytes: &[u8]) -> Self {
        let called_imsi = IMSI::new(&bytes[25..33]).value;
        let called_number = NUMBER::new(&bytes[33..43]).value;
        let routing_number = NUMBER::new(&bytes[43..55]).value;
        let charging_time = ChargingTime::new(&bytes[55..62]).value;
        let number_of_forwardings = NumberOfForwardings::new(bytes[62..63][0]).value;
        let cause_for_termination = CauseForTermination::new(&bytes[63..66]).value;

        Self {
            called_imsi,
            called_number,
            routing_number,
            charging_time,
            number_of_forwardings,
            cause_for_termination,
        }
    }
    pub fn to_json_str(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
    pub fn to_json(&self) -> serde_json::Result<Value> {
        serde_json::to_value(self)
    }
}

// FIELD NAME                                   DATA TYPE  POSITION

// called_imsi                                     C(  8)        25
// called_number                                   C( 10)        33
// routing_number                                  C( 12)        43
// charging_time                                   C(  7)        55
// number_of_forwardings                           C(  1)        62
// cause_for_termination                          DW(  1)        63
