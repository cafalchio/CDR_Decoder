// FORMAT TYPE:      27
// MESSAGE NUMBER:   e20d
// FORMAT TYPE NAME: LCS
// RECORD LENGTH:    164
use crate::datatypes::{charging_fields::*, primitives::BcdTimestamp};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LCS {
    pub served_imsi: String,
    pub served_imei: String,
    pub service_time: String,
    pub served_number_ton: String,
    pub served_number: String,
    pub served_subs_lac: String,
    pub served_subs_ci: String,
    pub location_estimate: String,
    pub location_request_type: String,
    pub cause_for_termination: String,
    pub used_position_method: String,
    pub response_time: String,
    pub vertical_accuracy: String,
    pub horizontal_accuracy: String,
    pub gmlc_address: String,
    pub subs_roaming_status: String,
    pub gps_data_length: String,
    pub gps_data: String,
    pub client_external_id: String,
    pub client_external_id_ton: String,
    pub age_of_estimate: String,
    pub radio_network_type: String,
}
impl LCS {
    pub fn new(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 164 {
            return Err("LCS: insufficient data".into());
        }
        let served_imsi = IMSI::new(&bytes[25..33]).value; // C(  8)        25
        let served_imei = IMEI::new(&bytes[33..41]).value; // C(  8)        33
        let service_time = BcdTimestamp::new(&bytes[41..48]).value; // C(  7)        41
        let served_number_ton = TON::new(bytes[48]).value; // C(  1)        48
        let served_number = NUMBER::new(&bytes[49..59]).value; // C( 10)        49
        let served_subs_lac = LAC::new(&bytes[59..61]).value; // W(  1)        59
        let served_subs_ci = CI::new(&bytes[61..63]).value; // W(  1)        61
        let location_estimate = LocationEstimate::new(&bytes[63..83]).value; // C( 20)        63
        let location_request_type = LocationRequestType::new(bytes[83]).value; // C(  1)        83
        let cause_for_termination = CauseForTermination::new(&bytes[84..88]).value; //DW(  1)        84
        let used_position_method = UsedPositionMethod::new(&bytes[88..96]).value; // C(  8)        88
        let response_time = ResponseTime::new(bytes[96]).value; // C(  1)        96
        let vertical_accuracy = VerticalAccuracy::new(bytes[97]).value; // C(  1)        97
        let horizontal_accuracy = HorizontalAccuracy::new(bytes[98]).value; // C(  1)        98
        let gmlc_address = GMLCAddress::new(&bytes[99..108]).value; // C(  9)        99
        let subs_roaming_status = SubsRoamingStatus::new(bytes[108]).value; // C(  1)       108
        let gps_data_length = GPSDataLength::new(bytes[109]).value; // C(  1)       109
        let gps_data = GPSData::new(&bytes[110..148]).value; // C( 38)       110
        let client_external_id = ClientExternalId::new(&bytes[148..160]).value; // C( 12)       148
        let client_external_id_ton = TON::new(bytes[160]).value; // C(  1)       160
        let age_of_estimate = AgeOfEstimate::new(&bytes[161..163]).value; // W(  1)       161
        let radio_network_type = RadioNetworkType::new(bytes[163]).value; // C(  1)       163
        Ok(Self {
            served_imsi,
            served_imei,
            service_time,
            served_number_ton,
            served_number,
            served_subs_lac,
            served_subs_ci,
            location_estimate,
            location_request_type,
            cause_for_termination,
            used_position_method,
            response_time,
            vertical_accuracy,
            horizontal_accuracy,
            gmlc_address,
            subs_roaming_status,
            gps_data_length,
            gps_data,
            client_external_id,
            client_external_id_ton,
            age_of_estimate,
            radio_network_type,
        })
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

// DATA:
// FIELD NAME                                   DATA TYPE  POSITION

// served_imsi                                     C(  8)        25
// served_imei                                     C(  8)        33
// service_time                                    C(  7)        41
// served_number_ton                               C(  1)        48
// served_number                                   C( 10)        49
// served_subs_lac                                 W(  1)        59
// served_subs_ci                                  W(  1)        61
// location_estimate                               C( 20)        63
// location_request_type                           C(  1)        83
// cause_for_termination                          DW(  1)        84
// used_position_method                            C(  8)        88
// response_time                                   C(  1)        96
// vertical_accuracy                               C(  1)        97
// horizontal_accuracy                             C(  1)        98
// gmlc_address                                    C(  9)        99
// subs_roaming_status                             C(  1)       108
// gps_data_length                                 C(  1)       109
// gps_data                                        C( 38)       110
// client_external_id                              C( 12)       148
// client_external_id_ton                          C(  1)       160
// age_of_estimate                                 W(  1)       161
// radio_network_type                              C(  1)       163
