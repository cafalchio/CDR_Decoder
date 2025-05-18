use crate::datatypes::charging_fields::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IN4 {
    pub in_record_number: String,
    pub in_data: String,
    pub leg_call_reference: String,
    pub in_channel_allocated_time: String,
    pub in_data_length: String,
    pub basic_call_state_model: String,
    pub party_to_charge: String,
    pub protocol_identification: String,
    pub call_reference_time: String,
    pub camel_call_reference: String,
    pub camel_exchange_id_ton: String,
    pub camel_exchange_id: String,
}
impl IN4 {
    pub fn new(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 107 {
            return Err("IN4: insufficient data".into());
        }
        let in_record_number = InRecordNumber::new(&bytes[25]).value; //BCD(  1)        25
        let in_data = InData::new(&bytes[26..66]).value; //  C( 40)        26
        let leg_call_reference = CallReference::new(&bytes[66..71]).value; //  C(  5)        66
        let in_channel_allocated_time = InChannelAllocatedTime::new(&bytes[71..78]).value; //  C(  7)        71
        let in_data_length = InDataLength::new(&bytes[78..80]).value; //  W(  1)        78
        let basic_call_state_model = BasicCallStateModel::new(bytes[80]).value; //  C(  1)        80
        let party_to_charge = PartyToCharge::new(bytes[81]).value; //  C(  1)        81
        let protocol_identification = ProtocolIdentification::new(bytes[82]).value; //  C(  1)        82
        let call_reference_time = CallReferenceTime::new(&bytes[83..90]).value; //  C(  7)        83
        let camel_call_reference = CamelCallReference::new(&bytes[90..98]).value; //  C(  8)        90
        let camel_exchange_id_ton = CamelExchangeId::new(&bytes[98..99]).value; //  C(  1)        98
        let camel_exchange_id = CamelExchangeId::new(&bytes[99..107]).value; //  C(  9)        99

        Ok(Self {
            in_record_number,
            in_data,
            leg_call_reference,
            in_channel_allocated_time,
            in_data_length,
            basic_call_state_model,
            party_to_charge,
            protocol_identification,
            call_reference_time,
            camel_call_reference,
            camel_exchange_id_ton,
            camel_exchange_id,
        })
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}
