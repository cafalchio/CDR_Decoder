use crate::datatypes::charging_fields::*;
use crate::datatypes::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PBXO {
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
    pub iaz_chrg_type: String,
    pub iaz_duration: String,
    pub iaz_tariff_class: String,
    pub iaz_pulses: String,
    pub called_msrn_ton: String,
    pub called_msrn: String,
    pub intermediate_chrg_cause: String,
    pub orig_dialling_class: String,
    pub leg_call_reference: String,
    pub call_reference_time: String,
    pub redirected_indicator: String,
    pub pni: String,
    pub b_idle_time: String,
    pub number_of_in_records: String,
    pub tns_carrier_code: String,
    pub carrier_selection: String,
    pub pic: String,
    pub npdb_query_status: String,
    pub loc_routing_number: String,
    pub scp_connection: String,
    pub number_of_all_in_records: String,
    pub loc_routing_number_ton: String,
    pub camel_call_reference: String,
    pub camel_exchange_id_ton: String,
    pub camel_exchange_id: String,
    pub collect_call_indicator: String,
    pub redirecting_number: String,
}
impl PBXO {
    pub fn new(bytes: &[u8]) -> Self {
        let intermediate_record_number = IntermediateRecordNumber::new(&bytes[25..26]).value; // BCD(  1)        25
        let intermediate_charging_ind = IntermediateChargingInd::new(bytes[26]).value; //   C(  1)        26
        let number_of_ss_records = NumberOfSSRecords::new(bytes[27]).value; // BCD(  1)        27
        let calling_number_ton = TON::new(bytes[28]).value; //   C(  1)        28
        let calling_number = NUMBER::new(&bytes[29..41]).value; //   C( 12)        29
        let called_number_ton = TON::new(bytes[41]).value; //   C(  1)        41
        let called_number = NUMBER::new(&bytes[42..54]).value; //   C( 12)        42
        let in_circuit_group = InCircuitGroup::new(&bytes[54..56]).value; // BCD(  2)        54
        let in_circuit = InCircuit::new(&bytes[56..58]).value; // BCD(  2)        56
        let in_channel_allocated_time = InChannelAllocatedTime::new(&bytes[58..65]).value; //   C(  7)        58
        let charging_start_time = BcdTimestamp::new(&bytes[65..72]).value; //   C(  7)        65
        let charging_end_time = BcdTimestamp::new(&bytes[72..79]).value; //   C(  7)        72
        let cause_for_termination = CauseForTermination::new(&bytes[79..83]).value; //  DW(  1)        79
        let call_type = CallType::new(bytes[83]).value; //   C(  1)        83
        let iaz_chrg_type = ChargeType::new(bytes[84]).value; //   C(  1)        84
        let iaz_duration = Duration::new(&bytes[85..88]).value; // BCD(  3)        85
        let iaz_tariff_class = TariffClass::new(&bytes[88..91]).value; // BCD(  3)        88
        let iaz_pulses = Pulses::new(&bytes[91..93]).value; // BCD(  2)        91
        let called_msrn_ton = TON::new(bytes[93]).value; //   C(  1)        93
        let called_msrn = MSRN::new(&bytes[94..106]).value; //   C( 12)        94
        let intermediate_chrg_cause = IntermediateChrgCause::new(&bytes[106..108]).value; //   C(  2)       106
        let orig_dialling_class = OrigDiallingClass::new(&bytes[108..110]).value; //   W(  1)       108
        let leg_call_reference = "<not implemented>".to_string(); //   C(  5)       110
        let call_reference_time = BcdTimestamp::new(&bytes[115..122]).value; //   C(  7)       115
        let redirected_indicator = RedirectedIndicator::new(bytes[122]).value; //   C(  1)       122
        let pni = PNI::new(&bytes[123..126]).value; //   C(  3)       123
        let b_idle_time = BcdTimestamp::new(&bytes[126..133]).value; //   C(  7)       126
        let number_of_in_records = NumberOfInRecords::new(bytes[133]).value; // BCD(  1)       133
        let tns_carrier_code = TNSCarrierCode::new(&bytes[134..136]).value; //   W(  1)       134
        let carrier_selection = CarrierSelection::new(bytes[136]).value; //   C(  1)       136
        let pic = PIC::new(&bytes[137..139]).value; //   W(  1)       137
        let npdb_query_status = NPDBQueryStatus::new(bytes[139]).value; //   C(  1)       139
        let loc_routing_number = NUMBER::new(&bytes[140..152]).value; //   C( 12)       140
        let scp_connection = SCPConnection::new(bytes[152]).value; //   C(  1)       152
        let number_of_all_in_records = NumberOfAllInRecords::new(bytes[153]).value; // BCD(  1)       153
        let loc_routing_number_ton = TON::new(bytes[154]).value; //   C(  1)       154
        let camel_call_reference = CamelCallReference::new(&bytes[155..163]).value; //   C(  8)       155
        let camel_exchange_id_ton = TON::new(bytes[163]).value; //   C(  1)       163
        let camel_exchange_id = CamelExchangeId::new(&bytes[164..173]).value; //   C(  9)       164
                                                                              // let collect_call_indicator = CollectCallIndicator::new(bytes[173]).value;     //   C(  1)       173
        let collect_call_indicator = "<not implemented>".to_string();
        let redirecting_number = NUMBER::new(&bytes[174..186]).value; //   C( 12)       174

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
            iaz_chrg_type,
            iaz_duration,
            iaz_tariff_class,
            iaz_pulses,
            called_msrn_ton,
            called_msrn,
            intermediate_chrg_cause,
            orig_dialling_class,
            leg_call_reference,
            call_reference_time,
            redirected_indicator,
            pni,
            b_idle_time,
            number_of_in_records,
            tns_carrier_code,
            carrier_selection,
            pic,
            npdb_query_status,
            loc_routing_number,
            scp_connection,
            number_of_all_in_records,
            loc_routing_number_ton,
            camel_call_reference,
            camel_exchange_id_ton,
            camel_exchange_id,
            collect_call_indicator,
            redirecting_number,
        }
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}
