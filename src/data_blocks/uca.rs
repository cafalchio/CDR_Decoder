use crate::datatypes::charging_fields::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UCA {
    pub intermediate_record_number: String,
    pub intermediate_charging_ind: String,
    pub number_of_ss_records: String,
    pub calling_imsi: String,
    pub calling_imei: String,
    pub calling_number_ton: String,
    pub calling_number: String,
    pub called_number_ton: String,
    pub called_number: String,
    pub in_circuit_group: String,
    pub in_circuit: String,
    pub calling_subs_lac: String,
    pub calling_subs_ci: String,
    pub basic_service_type: String,
    pub basic_service_code: String,
    pub start_time: String,
    pub release_time: String,
    pub cause_for_termination: String,
    pub routing_info: String,
    pub call_state: String,
    pub dialled_digits: String,
    pub calling_ms_classmark: String,
    pub dialled_digits_ton: String,
    pub facility_usage: String,
    pub call_reference_time: String,
    pub out_channel_allocated_time: String,
    pub out_circuit_group: String,
    pub out_circuit: String,
    pub forwarded_to_number_ton: String,
    pub forwarded_to_number: String,
    pub called_imsi: String,
    pub hot_billing_record_number: String,
    pub in_channel_allocated_time: String,
    pub routing_category: String,
    pub camel_call_reference: String,
    pub camel_exchange_id_ton: String,
    pub camel_exchange_id: String,
    pub req_fixed_nw_user_rate: String,
    pub req_other_modem_type: String,
    pub acceptable_channel_codings: String,
    pub req_number_of_channels: String,
    pub req_air_interface_user_rate: String,
    pub req_user_initiated_mod_ind: String,
    pub used_number_of_channels: String,
    pub used_other_modem_type: String,
    pub used_fixed_nw_user_rate: String,
    pub used_channel_coding: String,
    pub number_of_in_records: String,
    pub ms_classmark3: String,
    pub calling_cell_band: String,
    pub number_of_all_in_records: String,
    pub loc_routing_number_ton: String,
    pub loc_routing_number: String,
    pub npdb_query_status: String,
    pub tns_carrier_code: String,
    pub pic: String,
    pub carrier_selection: String,
    pub collect_call_indicator: String,
    pub in_bnc_connection_type: String,
    pub inside_user_plane_index: String,
    pub inside_control_plane_index: String,
    pub radio_network_type: String,
    pub used_air_interface_user_rate: String,
}
impl UCA {
    pub fn new(bytes: &[u8]) -> Self {
        let intermediate_record_number = "".to_string();
        let intermediate_charging_ind = "".to_string();
        let number_of_ss_records = "".to_string();
        let calling_imsi = "".to_string();
        let calling_imei = "".to_string();
        let calling_number_ton = "".to_string();
        let calling_number = "".to_string();
        let called_number_ton = "".to_string();
        let called_number = "".to_string();
        let in_circuit_group = "".to_string();
        let in_circuit = "".to_string();
        let calling_subs_lac = "".to_string();
        let calling_subs_ci = "".to_string();
        let basic_service_type = "".to_string();
        let basic_service_code = "".to_string();
        let start_time = "".to_string();
        let release_time = "".to_string();
        let cause_for_termination = "".to_string();
        let routing_info = "".to_string();
        let call_state = "".to_string();
        let dialled_digits = "".to_string();
        let calling_ms_classmark = "".to_string();
        let dialled_digits_ton = "".to_string();
        let facility_usage = "".to_string();
        let call_reference_time = "".to_string();
        let out_channel_allocated_time = "".to_string();
        let out_circuit_group = "".to_string();
        let out_circuit = "".to_string();
        let forwarded_to_number_ton = "".to_string();
        let forwarded_to_number = "".to_string();
        let called_imsi = "".to_string();
        let hot_billing_record_number = "".to_string();
        let in_channel_allocated_time = "".to_string();
        let routing_category = "".to_string();
        let camel_call_reference = "".to_string();
        let camel_exchange_id_ton = "".to_string();
        let camel_exchange_id = "".to_string();
        let req_fixed_nw_user_rate = "".to_string();
        let req_other_modem_type = "".to_string();
        let acceptable_channel_codings = "".to_string();
        let req_number_of_channels = "".to_string();
        let req_air_interface_user_rate = "".to_string();
        let req_user_initiated_mod_ind = "".to_string();
        let used_number_of_channels = "".to_string();
        let used_other_modem_type = "".to_string();
        let used_fixed_nw_user_rate = "".to_string();
        let used_channel_coding = "".to_string();
        let number_of_in_records = "".to_string();
        let ms_classmark3 = "".to_string();
        let calling_cell_band = "".to_string();
        let number_of_all_in_records = "".to_string();
        let loc_routing_number_ton = "".to_string();
        let loc_routing_number = "".to_string();
        let npdb_query_status = "".to_string();
        let tns_carrier_code = "".to_string();
        let pic = "".to_string();
        let carrier_selection = "".to_string();
        let collect_call_indicator = "".to_string();
        let in_bnc_connection_type = "".to_string();
        let inside_user_plane_index = "".to_string();
        let inside_control_plane_index = "".to_string();
        let radio_network_type = "".to_string();
        let used_air_interface_user_rate = "".to_string();

        Self {
            intermediate_record_number,
            intermediate_charging_ind,
            number_of_ss_records,
            calling_imsi,
            calling_imei,
            calling_number_ton,
            calling_number,
            called_number_ton,
            called_number,
            in_circuit_group,
            in_circuit,
            calling_subs_lac,
            calling_subs_ci,
            basic_service_type,
            basic_service_code,
            start_time,
            release_time,
            cause_for_termination,
            routing_info,
            call_state,
            dialled_digits,
            calling_ms_classmark,
            dialled_digits_ton,
            facility_usage,
            call_reference_time,
            out_channel_allocated_time,
            out_circuit_group,
            out_circuit,
            forwarded_to_number_ton,
            forwarded_to_number,
            called_imsi,
            hot_billing_record_number,
            in_channel_allocated_time,
            routing_category,
            camel_call_reference,
            camel_exchange_id_ton,
            camel_exchange_id,
            req_fixed_nw_user_rate,
            req_other_modem_type,
            acceptable_channel_codings,
            req_number_of_channels,
            req_air_interface_user_rate,
            req_user_initiated_mod_ind,
            used_number_of_channels,
            used_other_modem_type,
            used_fixed_nw_user_rate,
            used_channel_coding,
            number_of_in_records,
            ms_classmark3,
            calling_cell_band,
            number_of_all_in_records,
            loc_routing_number_ton,
            loc_routing_number,
            npdb_query_status,
            tns_carrier_code,
            pic,
            carrier_selection,
            collect_call_indicator,
            in_bnc_connection_type,
            inside_user_plane_index,
            inside_control_plane_index,
            radio_network_type,
            used_air_interface_user_rate,
        }
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

// FORMAT TYPE:      17
// MESSAGE NUMBER:   dd8f
// FORMAT TYPE NAME: UCA Unsuccessful call attempt
// RECORD LENGTH:    227

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

// intermediate_record_number                    BCD(  1)        25
// intermediate_charging_ind                       C(  1)        26
// number_of_ss_records                          BCD(  1)        27
// calling_imsi                                    C(  8)        28
// calling_imei                                    C(  8)        36
// calling_number_ton                              C(  1)        44
// calling_number                                  C( 10)        45
// called_number_ton                               C(  1)        55
// called_number                                   C( 12)        56
// in_circuit_group                              BCD(  2)        68
// in_circuit                                    BCD(  2)        70
// calling_subs_lac                                W(  1)        72
// calling_subs_ci                                 W(  1)        74
// basic_service_type                              C(  1)        76
// basic_service_code                              C(  1)        77
// start_time                                      C(  7)        78
// release_time                                    C(  7)        85
// cause_for_termination                          DW(  1)        92
// routing_info                                    C(  2)        96
// call_state                                      C(  1)        98
// dialled_digits                                  C( 12)        99
// calling_ms_classmark                            C(  1)       111
// dialled_digits_ton                              C(  1)       112
// facility_usage                                  C(  4)       113
// call_reference_time                             C(  7)       117
// out_channel_allocated_time                      C(  7)       124
// out_circuit_group                             BCD(  2)       131
// out_circuit                                   BCD(  2)       133
// forwarded_to_number_ton                         C(  1)       135
// forwarded_to_number                             C( 12)       136
// called_imsi                                     C(  8)       148
// hot_billing_record_number                     BCD(  4)       156
// in_channel_allocated_time                       C(  7)       160
// routing_category                                C(  1)       167
// camel_call_reference                            C(  8)       168
// camel_exchange_id_ton                           C(  1)       176
// camel_exchange_id                               C(  9)       177
// req_fixed_nw_user_rate                          C(  1)       186
// req_other_modem_type                            C(  1)       187
// acceptable_channel_codings                      C(  1)       188
// req_number_of_channels                          C(  1)       189
// req_air_interface_user_rate                     C(  1)       190
// req_user_initiated_mod_ind                      C(  1)       191
// used_number_of_channels                         C(  1)       192
// used_other_modem_type                           C(  1)       193
// used_fixed_nw_user_rate                         C(  1)       194
// used_channel_coding                             C(  1)       195
// number_of_in_records                          BCD(  1)       196
// ms_classmark3                                   C(  1)       197
// calling_cell_band                               C(  1)       198
// number_of_all_in_records                      BCD(  1)       199
// loc_routing_number_ton                          C(  1)       200
// loc_routing_number                              C( 12)       201
// npdb_query_status                               C(  1)       213
// tns_carrier_code                                W(  1)       214
// pic                                             W(  1)       216
// carrier_selection                               C(  1)       218
// collect_call_indicator                          C(  1)       219
// in_bnc_connection_type                          C(  1)       220
// inside_user_plane_index                       BCD(  2)       221
// inside_control_plane_index                    BCD(  2)       223
// radio_network_type                              C(  1)       225
// used_air_interface_user_rate                    C(  1)       226
