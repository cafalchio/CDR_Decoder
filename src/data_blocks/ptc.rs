#![allow(dead_code)]
#![allow(unused_variables)]

use crate::datatypes::charging_fields::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub struct PTC {
    pub intermediate_record_number: String,
    pub intermediate_charging_ind: String,
    pub number_of_ss_records: String,
    pub calling_number_ton: String,
    pub calling_number: String,
    pub called_number_ton: String,
    pub called_number: String,
    pub out_circuit_group: String,
    pub out_circuit: String,
    pub in_channel_allocated_time: String,
    pub charging_start_time: String,
    pub charging_end_time: String,
    pub cause_for_termination: String,
    pub call_type: String,
    pub ticket_type: String,
    pub oaz_chrg_type: String,
    pub oaz_duration: String,
    pub oaz_tariff_class: String,
    pub oaz_pulses: String,
    pub called_msrn_ton: String,
    pub called_msrn: String,
    pub intermediate_chrg_cause: String,
    pub leg_call_reference: String,
    pub out_channel_allocated_time: String,
    pub basic_service_type: String,
    pub basic_service_code: String,
    pub call_reference_time: String,
    pub b_idle_time: String,
    pub redirected_indicator: String,
    pub number_of_in_records: String,
    pub tns_carrier_code: String,
    pub carrier_selection: String,
    pub npdb_query_status: String,
    pub loc_routing_number: String,
    pub scp_connection: String,
    pub number_of_all_in_records: String,
    pub loc_routing_number_ton: String,
    pub out_bnc_connection_type: String,
    pub outside_user_plane_index: String,
    pub outside_control_plane_index: String,
    pub collect_call_indicator: String,
    pub outpulsed_number: String,
    pub redirecting_number: String,
    pub rate_adaption: String,
}
impl PTC {
    pub fn new(bytes: &[u8]) -> Self {
        let intermediate_record_number = IntermediateRecordNumber::new(&bytes[25..26]).value; //BCD(  1)        25
        let intermediate_charging_ind = "".to_string(); //  C(  1)        26
        let number_of_ss_records = "".to_string(); //BCD(  1)        27
        let calling_number_ton = "".to_string(); //  C(  1)        28
        let calling_number = "".to_string(); //  C( 12)        29
        let called_number_ton = "".to_string(); //  C(  1)        41
        let called_number = "".to_string(); //  C( 12)        42
        let out_circuit_group = "".to_string(); //BCD(  2)        54
        let out_circuit = "".to_string(); //BCD(  2)        56
        let in_channel_allocated_time = "".to_string(); //  C(  7)        58
        let charging_start_time = "".to_string(); //  C(  7)        65
        let charging_end_time = "".to_string(); //  C(  7)        72
        let cause_for_termination = "".to_string(); // DW(  1)        79
        let call_type = "".to_string(); //  C(  1)        83
        let ticket_type = "".to_string(); //  C(  1)        84
        let oaz_chrg_type = "".to_string(); //  C(  1)        85
        let oaz_duration = "".to_string(); //BCD(  3)        86
        let oaz_tariff_class = "".to_string(); //BCD(  3)        89
        let oaz_pulses = "".to_string(); //BCD(  2)        92
        let called_msrn_ton = "".to_string(); //  C(  1)        94
        let called_msrn = "".to_string(); //  C( 12)        95
        let intermediate_chrg_cause = "".to_string(); //  C(  2)       107
        let leg_call_reference = "".to_string(); //  C(  5)       109
        let out_channel_allocated_time = "".to_string(); //  C(  7)       114
        let basic_service_type = "".to_string(); //  C(  1)       121
        let basic_service_code = "".to_string(); //  C(  1)       122
        let call_reference_time = "".to_string(); //  C(  7)       123
        let b_idle_time = "".to_string(); //  C(  7)       130
        let redirected_indicator = "".to_string(); //  C(  1)       137
        let number_of_in_records = "".to_string(); //BCD(  1)       138
        let tns_carrier_code = "".to_string(); //  W(  1)       139
        let carrier_selection = "".to_string(); //  C(  1)       141
        let npdb_query_status = "".to_string(); //  C(  1)       142
        let loc_routing_number = "".to_string(); //  C( 12)       143
        let scp_connection = "".to_string(); //  C(  1)       155
        let number_of_all_in_records = "".to_string(); //BCD(  1)       156
        let loc_routing_number_ton = "".to_string(); //  C(  1)       157
        let out_bnc_connection_type = "".to_string(); //  C(  1)       158
        let outside_user_plane_index = "".to_string(); //BCD(  2)       159
        let outside_control_plane_index = "".to_string(); //BCD(  2)       161
        let collect_call_indicator = "".to_string(); //  C(  1)       163
        let outpulsed_number = "".to_string(); //  C( 12)       164
        let redirecting_number = "".to_string(); //  C( 12)       176
        let rate_adaption = "".to_string(); //  C(  1)       188

        Self {
            intermediate_record_number,
            intermediate_charging_ind,
            number_of_ss_records,
            calling_number_ton,
            calling_number,
            called_number_ton,
            called_number,
            out_circuit_group,
            out_circuit,
            in_channel_allocated_time,
            charging_start_time,
            charging_end_time,
            cause_for_termination,
            call_type,
            ticket_type,
            oaz_chrg_type,
            oaz_duration,
            oaz_tariff_class,
            oaz_pulses,
            called_msrn_ton,
            called_msrn,
            intermediate_chrg_cause,
            leg_call_reference,
            out_channel_allocated_time,
            basic_service_type,
            basic_service_code,
            call_reference_time,
            b_idle_time,
            redirected_indicator,
            number_of_in_records,
            tns_carrier_code,
            carrier_selection,
            npdb_query_status,
            loc_routing_number,
            scp_connection,
            number_of_all_in_records,
            loc_routing_number_ton,
            out_bnc_connection_type,
            outside_user_plane_index,
            outside_control_plane_index,
            collect_call_indicator,
            outpulsed_number,
            redirecting_number,
            rate_adaption,
        }
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

// DATA:
// FIELD NAME                                   DATA TYPE  POSITION

// intermediate_record_number                    BCD(  1)        25
// intermediate_charging_ind                       C(  1)        26
// number_of_ss_records                          BCD(  1)        27
// calling_number_ton                              C(  1)        28
// calling_number                                  C( 12)        29
// called_number_ton                               C(  1)        41
// called_number                                   C( 12)        42
// out_circuit_group                             BCD(  2)        54
// out_circuit                                   BCD(  2)        56
// in_channel_allocated_time                       C(  7)        58
// charging_start_time                             C(  7)        65
// charging_end_time                               C(  7)        72
// cause_for_termination                          DW(  1)        79
// call_type                                       C(  1)        83
// ticket_type                                     C(  1)        84
// oaz_chrg_type                                   C(  1)        85
// oaz_duration                                  BCD(  3)        86
// oaz_tariff_class                              BCD(  3)        89
// oaz_pulses                                    BCD(  2)        92
// called_msrn_ton                                 C(  1)        94
// called_msrn                                     C( 12)        95
// intermediate_chrg_cause                         C(  2)       107
// leg_call_reference                              C(  5)       109
// out_channel_allocated_time                      C(  7)       114
// basic_service_type                              C(  1)       121
// basic_service_code                              C(  1)       122
// call_reference_time                             C(  7)       123
// b_idle_time                                     C(  7)       130
// redirected_indicator                            C(  1)       137
// number_of_in_records                          BCD(  1)       138
// tns_carrier_code                                W(  1)       139
// carrier_selection                               C(  1)       141
// npdb_query_status                               C(  1)       142
// loc_routing_number                              C( 12)       143
// scp_connection                                  C(  1)       155
// number_of_all_in_records                      BCD(  1)       156
// loc_routing_number_ton                          C(  1)       157
// out_bnc_connection_type                         C(  1)       158
// outside_user_plane_index                      BCD(  2)       159
// outside_control_plane_index                   BCD(  2)       161
// collect_call_indicator                          C(  1)       163
// outpulsed_number                                C( 12)       164
// redirecting_number                              C( 12)       176
// rate_adaption                                   C(  1)       188
