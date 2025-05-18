// FORMAT TYPE:      14
// MESSAGE NUMBER:   dd91
// FORMAT TYPE NAME: PBXT
// RECORD LENGTH:    134
use crate::datatypes::charging_fields::*;
use crate::datatypes::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PBXT {
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
    pub oaz_chrg_type: String,
    pub oaz_duration: String,
    pub oaz_tariff_class: String,
    pub oaz_pulses: String,
    pub intermediate_chrg_cause: String,
    pub leg_call_reference: String,
    pub call_reference_time: String,
    pub redirected_indicator: String,
    pub pni: String,
    pub b_idle_time: String,
    pub number_of_in_records: String,
    pub scp_connection: String,
    pub number_of_all_in_records: String,
    pub redirecting_number: String,
    pub collect_call_indicator: String,
}
impl PBXT {
    pub fn new(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 134 {
            return Err("PBXT: insufficient data".into());
        }
        let intermediate_record_number = IntermediateRecordNumber::new(&bytes[25..26]).value; //  BCD(  1)        25
        let intermediate_charging_ind = IntermediateChargingInd::new(bytes[26]).value; //    C(  1)        26
        let number_of_ss_records = NumberOfSSRecords::new(bytes[27]).value; //  BCD(  1)        27
        let calling_number_ton = TON::new(bytes[28]).value; //    C(  1)        28
        let calling_number = NUMBER::new(&bytes[29..41]).value; //    C( 12)        29
        let called_number_ton = TON::new(bytes[41]).value; //    C(  1)        41
        let called_number = NUMBER::new(&bytes[42..54]).value; //    C( 12)        42
        let out_circuit_group = OutCircuitGroup::new(&bytes[54..56]).value; //  BCD(  2)        54
        let out_circuit = OutCircuit::new(&bytes[56..58]).value; //  BCD(  2)        56
        let in_channel_allocated_time = InChannelAllocatedTime::new(&bytes[58..65]).value; //    C(  7)        58
        let charging_start_time = BcdTimestamp::new(&bytes[65..72]).value; //    C(  7)        65
        let charging_end_time = BcdTimestamp::new(&bytes[72..79]).value; //    C(  7)        72
        let cause_for_termination = CauseForTermination::new(&bytes[79..83]).value; //   DW(  1)        79
        let call_type = CallType::new(bytes[83]).value; //    C(  1)        83
        let oaz_chrg_type = ChargeType::new(bytes[84]).value; //    C(  1)        84
        let oaz_duration = Duration::new(&bytes[85..88]).value; //  BCD(  3)        85
        let oaz_tariff_class = TariffClass::new(&bytes[88..91]).value; //  BCD(  3)        88
        let oaz_pulses = Pulses::new(&bytes[91..93]).value; //  BCD(  2)        91
        let intermediate_chrg_cause = IntermediateChrgCause::new(&bytes[93..95]).value; //    C(  2)        93
                                                                                        // let leg_call_reference = PNI::new(&bytes[95..100]).value;      //    C(  5)        95
        let leg_call_reference = "<not implemented>".to_string();
        let call_reference_time = BcdTimestamp::new(&bytes[100..107]).value; //    C(  7)       100
        let redirected_indicator = RedirectedIndicator::new(bytes[107]).value; //    C(  1)       107
        let pni = PNI::new(&bytes[108..111]).value; //    C(  3)       108
        let b_idle_time = BcdTimestamp::new(&bytes[111..118]).value; //    C(  7)       111
        let number_of_in_records = NumberOfInRecords::new(bytes[118]).value; //  BCD(  1)       118
        let scp_connection = SCPConnection::new(bytes[119]).value; //    C(  1)       119
        let number_of_all_in_records = NumberOfAllInRecords::new(bytes[120]).value; //  BCD(  1)       120
        let redirecting_number = NUMBER::new(&bytes[121..133]).value; //    C( 12)       121
                                                                      // let collect_call_indicator = PNI::new(bytes[36]).value;       //    C(  1)       133
        let collect_call_indicator = "<not implemented>".to_string();

        Ok(Self {
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
            oaz_chrg_type,
            oaz_duration,
            oaz_tariff_class,
            oaz_pulses,
            intermediate_chrg_cause,
            leg_call_reference,
            call_reference_time,
            redirected_indicator,
            pni,
            b_idle_time,
            number_of_in_records,
            scp_connection,
            number_of_all_in_records,
            redirecting_number,
            collect_call_indicator,
        })
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}
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
// oaz_chrg_type                                   C(  1)        84
// oaz_duration                                  BCD(  3)        85
// oaz_tariff_class                              BCD(  3)        88
// oaz_pulses                                    BCD(  2)        91
// intermediate_chrg_cause                         C(  2)        93
// leg_call_reference                              C(  5)        95
// call_reference_time                             C(  7)       100
// redirected_indicator                            C(  1)       107
// pni                                             C(  3)       108
// b_idle_time                                     C(  7)       111
// number_of_in_records                          BCD(  1)       118
// scp_connection                                  C(  1)       119
// number_of_all_in_records                      BCD(  1)       120
// redirecting_number                              C( 12)       121
// collect_call_indicator                          C(  1)       133
