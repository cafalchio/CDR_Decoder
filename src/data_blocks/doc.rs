use crate::datatypes::charging_fields::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct DOC {
    pub intermediate_record_number: String,
    pub intermediate_charging_ind: String,
    pub intermediate_chrg_cause: String,
    pub number_of_ss_records: String,
    pub calling_number_ton: String,
    pub calling_number: String,
    pub called_number_ton: String,
    pub called_number: String,
    pub in_channel_allocated_time: String,
    pub charging_start_time: String,
    pub charging_end_time: String,
    pub cause_for_termination: String,
    pub call_type: String,
    pub orig_mcz_chrg_type: String,
    pub orig_mcz_duration: String,
    pub orig_mcz_tariff_class: String,
    pub orig_mcz_pulses: String,
    pub orig_mcz_change_percent: String,
    pub orig_mcz_change_direction: String,
    pub leg_call_reference: String,
    pub device_identifier: String,
    pub service_identifier: String,
    pub call_reference_time: String,
}

impl DOC {
    pub fn new(bytes: &[u8]) -> Self {
        let intermediate_record_number = IntermediateRecordNumber::new(&bytes[25..26]).value; // BCD(  1)        25
        let intermediate_charging_ind = IntermediateChargingInd::new(bytes[26]).value; //   C(  1)        26
        let intermediate_chrg_cause = IntermediateChrgCause::new(&bytes[27..29]).value; //   C(  2)        27
        let number_of_ss_records = NumberOfSSRecords::new(bytes[29]).value; // BCD(  1)        29
        let calling_number_ton = TON::new(bytes[30]).value; //   C(  1)        30
        let calling_number = CallingNumber::new(&bytes[31..43]).value; //   C( 12)        31
        let called_number_ton = TON::new(bytes[43]).value; //   C(  1)        43
        let called_number = NUMBER::new(&bytes[44..56]).value; //   C( 12)        44
        let in_channel_allocated_time = InChannelAllocatedTime::new(&bytes[56..63]).value; //   C(  7)        56
        let charging_start_time = ChargingStartTime::new(&bytes[63..70]).value; //   C(  7)        63
        let charging_end_time = ChargingEndtime::new(&bytes[70..77]).value; //   C(  7)        70
        let cause_for_termination = CauseForTermination::new(&bytes[77..81]).value; //  DW(  1)        77
        let call_type = CallType::new(bytes[80]).value; //   C(  1)        81
        let orig_mcz_chrg_type = ChargeType::new(bytes[82]).value; //   C(  1)        82
        let orig_mcz_duration = Duration::new(&bytes[83..86]).value; // BCD(  3)        83
        let orig_mcz_tariff_class = TariffClass::new(&bytes[86..89]).value; // BCD(  3)        86
        let orig_mcz_pulses = Pulses::new(&bytes[89..91]).value; // BCD(  2)        89
        let orig_mcz_change_percent = ChangePercent::new(bytes[91]).value; //   C(  1)        91
        let orig_mcz_change_direction = ChangeDirection::new(bytes[92]).value; //   C(  1)        92
                                                                               // let leg_call_reference = LegCallReference::new(bytes[93..98]).value; //   C(  5)        93
        let leg_call_reference = "<not implemented>".to_string();
        let device_identifier = DeviceIdentifier::new(bytes[98]).value; //   C(  1)        98
        let service_identifier = ServiceIdentifier::new(bytes[99]).value; //   C(  1)        99
        let call_reference_time = CallReferenceTime::new(&bytes[100..107]).value; //   C(  7)       100

        Self {
            intermediate_record_number,
            intermediate_charging_ind,
            intermediate_chrg_cause,
            number_of_ss_records,
            calling_number_ton,
            calling_number,
            called_number_ton,
            called_number,
            in_channel_allocated_time,
            charging_start_time,
            charging_end_time,
            cause_for_termination,
            call_type,
            orig_mcz_chrg_type,
            orig_mcz_duration,
            orig_mcz_tariff_class,
            orig_mcz_pulses,
            orig_mcz_change_percent,
            orig_mcz_change_direction,
            leg_call_reference,
            device_identifier,
            service_identifier,
            call_reference_time,
        }
    }

    pub fn to_json_str(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
    pub fn to_json(&self) -> serde_json::Result<Value> {
        serde_json::to_value(self)
    }
}

// DATA:
// FIELD NAME                                   DATA TYPE  POSITION

// intermediate_record_number                    BCD(  1)        25
// intermediate_charging_ind                       C(  1)        26
// intermediate_chrg_cause                         C(  2)        27
// number_of_ss_records                          BCD(  1)        29
// calling_number_ton                              C(  1)        30
// calling_number                                  C( 12)        31
// called_number_ton                               C(  1)        43
// called_number                                   C( 12)        44
// in_channel_allocated_time                       C(  7)        56
// charging_start_time                             C(  7)        63
// charging_end_time                               C(  7)        70
// cause_for_termination                          DW(  1)        77
// call_type                                       C(  1)        81
// orig_mcz_chrg_type                              C(  1)        82
// orig_mcz_duration                             BCD(  3)        83
// orig_mcz_tariff_class                         BCD(  3)        86
// orig_mcz_pulses                               BCD(  2)        89
// orig_mcz_change_percent                         C(  1)        91
// orig_mcz_change_direction                       C(  1)        92
// leg_call_reference                              C(  5)        93
// device_identifier                               C(  1)        98
// service_identifier                              C(  1)        99
// call_reference_time                             C(  7)       100
