use crate::datatypes::charging_fields::*;
use serde::{Deserialize, Serialize};

// FORMAT TYPE:      5
// MESSAGE NUMBER:   efcd
// FORMAT TYPE NAME: SUPS Supplementary Service
// RECORD LENGTH:    157

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
#[derive(Serialize, Deserialize, Debug)]
pub struct SUPS {
    pub ss_record_number: String,
    pub served_imsi: String,
    pub served_imei: String,
    pub served_number: String,
    pub served_subs_lac: String,
    pub served_subs_ci: String,
    pub supplementary_service_code: String,
    pub action: String,
    pub in_channel_allocated_time: String,
    pub charging_time: String,
    pub cause_for_termination: String,
    pub result_indicator: String,
    pub parameters_length: String,
    pub parameters: String,
    pub served_number_ton: String,
    pub served_ms_classmark: String,
    pub basic_service_type: String,
    pub basic_service_code: String,
    pub leg_call_reference: String,
    pub call_reference_time: String,
    pub pni: String,
    pub hot_billing_record_number: String,
    pub routing_category: String,
    pub ms_classmark3: String,
    pub served_cell_band: String,
    pub basic_call_state_model: String,
    pub camel_call_reference: String,
    pub camel_exchange_id_ton: String,
    pub camel_exchange_id: String,
    pub add_routing_category: String,
    pub radio_network_type: String,
}
impl SUPS {
    pub fn new(bytes: &[u8]) -> Self {
        let ss_record_number = "".to_string();
        let served_imsi = "".to_string();
        let served_imei = "".to_string();
        let served_number = "".to_string();
        let served_subs_lac = "".to_string();
        let served_subs_ci = "".to_string();
        let supplementary_service_code = "".to_string();
        let action = "".to_string();
        let in_channel_allocated_time = "".to_string();
        let charging_time = "".to_string();
        let cause_for_termination = "".to_string();
        let result_indicator = "".to_string();
        let parameters_length = "".to_string();
        let parameters = "".to_string();
        let served_number_ton = "".to_string();
        let served_ms_classmark = "".to_string();
        let basic_service_type = "".to_string();
        let basic_service_code = "".to_string();
        let leg_call_reference = "".to_string();
        let call_reference_time = "".to_string();
        let pni = "".to_string();
        let hot_billing_record_number = "".to_string();
        let routing_category = "".to_string();
        let ms_classmark3 = "".to_string();
        let served_cell_band = "".to_string();
        let basic_call_state_model = "".to_string();
        let camel_call_reference = "".to_string();
        let camel_exchange_id_ton = "".to_string();
        let camel_exchange_id = "".to_string();
        let add_routing_category = "".to_string();
        let radio_network_type = "".to_string();

        Self {
            ss_record_number,
            served_imsi,
            served_imei,
            served_number,
            served_subs_lac,
            served_subs_ci,
            supplementary_service_code,
            action,
            in_channel_allocated_time,
            charging_time,
            cause_for_termination,
            result_indicator,
            parameters_length,
            parameters,
            served_number_ton,
            served_ms_classmark,
            basic_service_type,
            basic_service_code,
            leg_call_reference,
            call_reference_time,
            pni,
            hot_billing_record_number,
            routing_category,
            ms_classmark3,
            served_cell_band,
            basic_call_state_model,
            camel_call_reference,
            camel_exchange_id_ton,
            camel_exchange_id,
            add_routing_category,
            radio_network_type,
        }
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

// ss_record_number                              BCD(  1)        25
// served_imsi                                     C(  8)        26
// served_imei                                     C(  8)        34
// served_number                                   C( 10)        42
// served_subs_lac                                 W(  1)        52
// served_subs_ci                                  W(  1)        54
// supplementary_service_code                      C(  1)        56
// action                                          C(  1)        57
// in_channel_allocated_time                       C(  7)        58
// charging_time                                   C(  7)        65
// cause_for_termination                          DW(  1)        72
// result_indicator                                W(  1)        76
// parameters_length                               C(  1)        78
// parameters                                      C( 30)        79
// served_number_ton                               C(  1)       109
// served_ms_classmark                             C(  1)       110
// basic_service_type                              C(  1)       111
// basic_service_code                              C(  1)       112
// leg_call_reference                              C(  5)       113
// call_reference_time                             C(  7)       118
// pni                                             C(  3)       125
// hot_billing_record_number                     BCD(  4)       128
// routing_category                                C(  1)       132
// ms_classmark3                                   C(  1)       133
// served_cell_band                                C(  1)       134
// basic_call_state_model                          C(  1)       135
// camel_call_reference                            C(  8)       136
// camel_exchange_id_ton                           C(  1)       144
// camel_exchange_id                               C(  9)       145
// add_routing_category                            W(  1)       154
// radio_network_type                              C(  1)       156
