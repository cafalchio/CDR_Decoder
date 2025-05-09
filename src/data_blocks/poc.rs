#![allow(dead_code)]
#![allow(unused_variables)]

use crate::datatypes::charging_fields::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub struct POC {
    pub intermediate_record_number: String,
    pub intermediate_charging_ind: String,
    pub number_of_ss_records: String,
    pub calling_number_ton: String,
    pub calling_number: String,
    pub called_number_ton: String,
    pub called_number: String,
    pub in_circuit_group: String,
    pub in_circuit: String,
    pub in_channel_allocated_time: String,
    pub charging_start_time: String,
    pub charging_end_time: String,
    pub cause_for_termination: String,
    pub call_type: String,
    pub ticket_type: String,
    pub iaz_chrg_type: String,
    pub iaz_duration: String,
    pub iaz_tariff_class: String,
    pub iaz_pulses: String,
    pub called_msrn_ton: String,
    pub called_msrn: String,
    pub intermediate_chrg_cause: String,
    pub orig_dialling_class: String,
    pub leg_call_reference: String,
    pub basic_service_type: String,
    pub basic_service_code: String,
    pub call_reference_time: String,
    pub number_of_in_records: String,
    pub b_idle_time: String,
    pub redirected_indicator: String,
    pub loc_routing_number: String,
    pub npdb_query_status: String,
    pub scp_connection: String,
    pub number_of_all_in_records: String,
    pub loc_routing_number_ton: String,
    pub camel_call_reference: String,
    pub camel_exchange_id_ton: String,
    pub camel_exchange_id: String,
    pub in_bnc_connection_type: String,
    pub inside_user_plane_index: String,
    pub inside_control_plane_index: String,
    pub collect_call_indicator: String,
    pub redirecting_number: String,
    pub dialled_digits: String,
    pub rate_adaption: String,
    pub calling_pstn_category: String,
}
impl POC {
    pub fn new(bytes: &[u8]) -> Self {
        let intermediate_record_number = IntermediateRecordNumber::new(&bytes[25..26]).value; // BCD(  1)        25
        let intermediate_charging_ind = IntermediateChargingInd::new(bytes[26]).value; // C(  1)         26
        let number_of_ss_records = NumberOfSSRecords::new(bytes[27]).value; // BCD(  1)        27
        let calling_number_ton = TON::new(bytes[28]).value; // C(  1)         28
        let calling_number = CallingNumber::new(&bytes[29..41]).value; // C( 12)        29
        let called_number_ton = TON::new(bytes[41]).value; // C(  1)         41
        let called_number = NUMBER::new(&bytes[42..54]).value; // C( 12)        42
        let in_circuit_group = OutCircuitGroup::new(&bytes[54..56]).value; // BCD(  2)       54
        let in_circuit = OutCircuit::new(&bytes[56..58]).value; // BCD(  2)       56
        let in_channel_allocated_time = InChannelAllocatedTime::new(&bytes[58..65]).value; // C(  7)         58
        let charging_start_time = ChargingStartTime::new(&bytes[65..72]).value; // C(  7)         65
        let charging_end_time = ChargingEndtime::new(&bytes[72..79]).value; // C(  7)         72
        let cause_for_termination = CauseForTermination::new(&bytes[79..83]).value; // DW(  1)        79
        let call_type = CallType::new(bytes[83]).value; // C(  1)  83
                                                        // let ticket_type = TickeType::new(bytes[84]).value;                                     // C(  1)         84
        let ticket_type = "<not implemented>".to_string();
        let iaz_chrg_type = ChargeType::new(bytes[85]).value; // C(  1)         85
        let iaz_duration = Duration::new(&bytes[86..89]).value; // BCD(  3)       86
        let iaz_tariff_class = TariffClass::new(&bytes[89..92]).value; // BCD(  3)       89
        let iaz_pulses = Pulses::new(&bytes[92..94]).value; // BCD(  2)       92
        let called_msrn_ton = TON::new(bytes[94]).value; // C(  1)         94
        let called_msrn = MSRN::new(&bytes[95..107]).value; // C( 12)        95
        let intermediate_chrg_cause = IntermediateChrgCause::new(&bytes[107..109]).value; // C(  2)        107
        let orig_dialling_class = "<not implemented>".to_string(); // W(  1)        109 — not in example
        let leg_call_reference = "<not implemented>".to_string();
        let basic_service_type = BasicServiceType::new(bytes[115]).value; // C(  1)        115
        let basic_service_code = BasicServiceCode::new(bytes[116], &basic_service_type).value; // C(  1)        116
        let call_reference_time = CallReferenceTime::new(&bytes[117..124]).value; // C(  7)        117
        let number_of_in_records = NumberOfInRecords::new(bytes[124]).value; // BCD(  1)      124
        let b_idle_time = BIdleTime::new(&bytes[125..132]).value; // C(  7)        125
        let redirected_indicator = RedirectedIndicator::new(bytes[132]).value; // C(  1)        132
        let loc_routing_number = NUMBER::new(&bytes[133..145]).value; // C( 12)       133
        let npdb_query_status = NPDBQueryStatus::new(bytes[145]).value; // C(  1)       145
        let scp_connection = SCPConnection::new(bytes[146]).value; // C(  1)       146
        let number_of_all_in_records = NumberOfAllInRecords::new(bytes[147]).value; // BCD(  1)      147
        let loc_routing_number_ton = TON::new(bytes[148]).value; // C(  1)       148
        let camel_call_reference = CamelCallReference::new(&bytes[149..157]).value; // C(  8)       149 — not in example
        let camel_exchange_id_ton = TON::new(bytes[149]).value; // C(  1)       157 — not in example
        let camel_exchange_id = CamelExchangeId::new(&bytes[158..167]).value; // C(  9)       158 — not in example
        let in_bnc_connection_type = BncConnectionType::new(bytes[167]).value; // C(  1)       167 — not in example
        let inside_user_plane_index = UserPlaneIndex::new(&bytes[168..170]).value; // BCD(  2)     168 — not in example
        let inside_control_plane_index = ControlPlaneIndex::new(&bytes[170..172]).value; // BCD(  2)     170 — not in example
        let collect_call_indicator = "<not implemented>".to_string(); // C(  1)       172 — not in example
        let redirecting_number = NUMBER::new(&bytes[173..185]).value; // C( 12)       173
        let dialled_digits = DialledDigits::new(&bytes[185..197]).value; // C( 12)       185 — not in example
        let rate_adaption = RateAdaption::new(bytes[197]).value; // C(  1)       197
        let calling_pstn_category = CallingPSTNCategory::new(bytes[198]).value; // C(  1)       198 — not in example

        Self {
            intermediate_record_number,
            intermediate_charging_ind,
            number_of_ss_records,
            calling_number_ton,
            calling_number,
            called_number_ton,
            called_number,
            in_circuit_group,
            in_circuit,
            in_channel_allocated_time,
            charging_start_time,
            charging_end_time,
            cause_for_termination,
            call_type,
            ticket_type,
            iaz_chrg_type,
            iaz_duration,
            iaz_tariff_class,
            iaz_pulses,
            called_msrn_ton,
            called_msrn,
            intermediate_chrg_cause,
            orig_dialling_class,
            leg_call_reference,
            basic_service_type,
            basic_service_code,
            call_reference_time,
            number_of_in_records,
            b_idle_time,
            redirected_indicator,
            loc_routing_number,
            npdb_query_status,
            scp_connection,
            number_of_all_in_records,
            loc_routing_number_ton,
            camel_call_reference,
            camel_exchange_id_ton,
            camel_exchange_id,
            in_bnc_connection_type,
            inside_user_plane_index,
            inside_control_plane_index,
            collect_call_indicator,
            redirecting_number,
            dialled_digits,
            rate_adaption,
            calling_pstn_category,
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
// in_circuit_group                              BCD(  2)        54
// in_circuit                                    BCD(  2)        56
// in_channel_allocated_time                       C(  7)        58
// charging_start_time                             C(  7)        65
// charging_end_time                               C(  7)        72
// cause_for_termination                          DW(  1)        79
// call_type                                       C(  1)        83
// ticket_type                                     C(  1)        84
// iaz_chrg_type                                   C(  1)        85
// iaz_duration                                  BCD(  3)        86
// iaz_tariff_class                              BCD(  3)        89
// iaz_pulses                                    BCD(  2)        92
// called_msrn_ton                                 C(  1)        94
// called_msrn                                     C( 12)        95
// intermediate_chrg_cause                         C(  2)       107
// orig_dialling_class                             W(  1)       109
// leg_call_reference                              C(  5)       111
// basic_service_type                              C(  1)       116
// basic_service_code                              C(  1)       117
// call_reference_time                             C(  7)       118
// number_of_in_records                          BCD(  1)       125
// b_idle_time                                     C(  7)       126
// redirected_indicator                            C(  1)       133
// loc_routing_number                              C( 12)       134
// npdb_query_status                               C(  1)       146
// scp_connection                                  C(  1)       147
// number_of_all_in_records                      BCD(  1)       148
// loc_routing_number_ton                          C(  1)       149
// camel_call_reference                            C(  8)       150
// camel_exchange_id_ton                           C(  1)       158
// camel_exchange_id                               C(  9)       159
// in_bnc_connection_type                          C(  1)       168
// inside_user_plane_index                       BCD(  2)       169
// inside_control_plane_index                    BCD(  2)       171
// collect_call_indicator                          C(  1)       173
// redirecting_number                              C( 12)       174
// dialled_digits                                  C( 12)       186
// rate_adaption                                   C(  1)       198
// calling_pstn_category                           C(  1)       199
