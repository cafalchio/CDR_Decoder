use crate::datatypes::{charging_fields::*, primitives::BcdTimestamp};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
// FORMAT TYPE:      24
// MESSAGE NUMBER:   dd8c
// FORMAT TYPE NAME: COC Camel-originated Call
// RECORD LENGTH:    135
pub struct COC {
    pub intermediate_record_number: String,
    pub intermediate_charging_ind: String,
    pub in_channel_allocated_time: String,
    pub leg_call_reference: String,
    pub intermediate_chrg_cause: String,
    pub camel_call_reference: String,
    pub camel_exchange_id_ton: String,
    pub camel_exchange_id: String,
    pub charging_start_time: String,
    pub charging_end_time: String,
    pub duration_before_answer: String,
    pub chargeable_duration: String,
    pub basic_call_state_model: String,
    pub scf_address_ton: String,
    pub scf_address: String,
    pub camel_service_key: String,
    pub default_call_handling: String,
    pub destination_number_ton: String,
    pub destination_number: String,
    pub level_of_camel_service: String,
    pub camel_modification: String,
    pub camel_modify_parameters: String,
    pub number_of_in_records: String,
    pub call_reference_time: String,
}

impl COC {
    pub fn new(bytes: &[u8]) -> Self {
        let intermediate_record_number = IntermediateRecordNumber::new(&bytes[25..26]).value;
        let intermediate_charging_ind = IntermediateChargingInd::new(bytes[26]).value;
        let in_channel_allocated_time = BcdTimestamp::new(&bytes[27..34]).value;
        // let leg_call_reference = LegCallReference::new(&bytes[34..39]).value;
        let leg_call_reference = "not implemented".to_string();
        let intermediate_chrg_cause = IntermediateChrgCause::new(&bytes[39..41]).value;
        let camel_call_reference = CamelCallReference::new(&bytes[41..49]).value;
        let camel_exchange_id_ton = TON::new(bytes[49]).value;
        let camel_exchange_id = CamelExchangeId::new(&bytes[50..59]).value;
        let charging_start_time = ChargingStartTime::new(&bytes[59..66]).value;
        let charging_end_time = ChargingEndtime::new(&bytes[66..73]).value;
        let duration_before_answer = DurationBeforeAnswer::new(&bytes[73..76]).value;
        let chargeable_duration = Duration::new(&bytes[76..79]).value;
        let basic_call_state_model = BasicCallStateModel::new(bytes[79]).value;
        let scf_address_ton = TON::new(bytes[80]).value;
        let scf_address = SCFAddress::new(&bytes[81..90]).value;
        let camel_service_key = CamelServiceKey::new(&bytes[90..94]).value;
        let default_call_handling = DefaultCallHandling::new(bytes[94]).value;
        let destination_number_ton = TON::new(bytes[95]).value;
        let destination_number = NUMBER::new(&bytes[96..108]).value;
        let level_of_camel_service = LevelOfCamelService::new(bytes[108]).value;
        let camel_modification = CamelModification::new(&bytes[109..113]).value;
        let camel_modify_parameters = CamelModifyParameters::new(&bytes[113..127]).value;
        let number_of_in_records = NumberOfInRecords::new(bytes[127]).value;
        let call_reference_time = CallReferenceTime::new(&bytes[128..135]).value;

        Self {
            intermediate_record_number,
            intermediate_charging_ind,
            in_channel_allocated_time,
            leg_call_reference,
            intermediate_chrg_cause,
            camel_call_reference,
            camel_exchange_id_ton,
            camel_exchange_id,
            charging_start_time,
            charging_end_time,
            duration_before_answer,
            chargeable_duration,
            basic_call_state_model,
            scf_address_ton,
            scf_address,
            camel_service_key,
            default_call_handling,
            destination_number_ton,
            destination_number,
            level_of_camel_service,
            camel_modification,
            camel_modify_parameters,
            number_of_in_records,
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
// in_channel_allocated_time                       C(  7)        27
// leg_call_reference                              C(  5)        34
// intermediate_chrg_cause                         C(  2)        39
// camel_call_reference                            C(  8)        41
// camel_exchange_id_ton                           C(  1)        49
// camel_exchange_id                               C(  9)        50
// charging_start_time                             C(  7)        59
// charging_end_time                               C(  7)        66
// duration_before_answer                        BCD(  3)        73
// chargeable_duration                           BCD(  3)        76
// basic_call_state_model                          C(  1)        79
// scf_address_ton                                 C(  1)        80
// scf_address                                     C(  9)        81
// camel_service_key                              DW(  1)        90
// default_call_handling                           C(  1)        94
// destination_number_ton                          C(  1)        95
// destination_number                              C( 12)        96
// level_of_camel_service                          C(  1)       108
// camel_modification                              C(  4)       109
// camel_modify_parameters                         C( 14)       113
// number_of_in_records                          BCD(  1)       127
// call_reference_time                             C(  7)       128
