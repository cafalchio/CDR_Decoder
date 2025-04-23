#![allow(dead_code)]
#![allow(unused_variables)]
// use crate::datatypes::charging_fields::*;
use serde::{Deserialize, Serialize};

use crate::datatypes::charging_fields::*;

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
        let ss_record_number = SSRecordNumber::new(bytes[25]).value;
        let served_imsi = IMSI::new(&bytes[26..34]).value;
        let served_imei = IMEI::new(&bytes[34..42]).value;
        let served_number = NUMBER::new(&bytes[42..52]).value;
        let served_subs_lac = LAC::new(&bytes[52..54]).value;
        let served_subs_ci = CI::new(&bytes[54..56]).value;
        let supplementary_service_code = SupplementaryServicecode::new(bytes[56]).value;
        let action = Action::new(bytes[57]).value;
        let in_channel_allocated_time = InChannelAllocatedTime::new(&bytes[58..65]).value;
        let charging_time = ChargingTime::new(&bytes[65..72]).value;
        let cause_for_termination = CauseForTermination::new(&bytes[72..76]).value;
        let result_indicator = ResultIndicator::new(&bytes[76..78]).value;
        let parameters_length = ParametersLength::new(bytes[78]).value;
        let parameters = "".to_string();
        let served_number_ton = TON::new(bytes[109]).value;
        let served_ms_classmark = MSClassMark::new(bytes[110]).value;
        let basic_service_type = BasicServiceType::new(bytes[111]).value;
        let basic_service_code = BasicServiceCode::new(bytes[112], &basic_service_type).value;
        let leg_call_reference = CallReference::new(&bytes[113..118]).value;
        let call_reference_time = CallReferenceTime::new(&bytes[118..125]).value;
        let pni = PNI::new(&bytes[125..128]).value;
        let hot_billing_record_number = HotBilingRecordNumber::new(&bytes[128..132]).value;
        let routing_category = RoutingCategory::new(bytes[132]).value;
        let ms_classmark3 = MSClassMark3::new(bytes[133]).value;
        let served_cell_band = CellBand::new(bytes[134]).value;
        let basic_call_state_model = BasicCallStateModel::new(bytes[135]).value;
        let camel_call_reference = CamelCallReference::new(&bytes[136..144]).value;
        let camel_exchange_id_ton = TON::new(bytes[144]).value;
        let camel_exchange_id = CamelExchangeId::new(&bytes[145..154]).value;
        let add_routing_category = AddRoutingCategory::new(&bytes[154..156]).value;
        let radio_network_type = RadioNetworkType::new(bytes[156]).value;

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
