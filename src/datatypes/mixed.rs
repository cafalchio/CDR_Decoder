use crate::datatypes::primitives::*;
use std::convert::TryFrom;
use std::fmt;


#[derive(Debug)]
pub enum RecordType {
    Header,
    MobileOriginatedCall,
    MobileTerminatedCall,
    ForwardedCall,
    CallToRoamingSubscriber,
    SupplementaryService,
    HlrInterrogation,
    LocationUpdate,
    SmsMo, // Short Message Service, mobile-originated
    SmsMt, // Short Message Service, mobile-terminated
    Trailer,
    PstnOriginatedCall,
    PstnTerminatedCall,
    PbxOriginatedCall,
    PbxTerminatedCall,
    UseOfHardware,
    InData1,
    UnsuccessfulCallAttempt,
    InData2,
    InData3,
    DeviceOriginatedCall,
    RemoteChargingControl = 22,
    InForwardedSms,
    CamelOriginatedCall,
    CamelTerminatedCall,
    InData4,
    LocationService,
    InData5,
    Ussd,
    SipOriginatedCall,
    SipTerminatedCall,
    SipOriginatingMessage,
    SipTerminatingMessage,
    SipRegistrationCdr = 35,
}
impl TryFrom<u8> for RecordType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use RecordType::*;
        let value = BCD::new(&value).value;
        let rt = match value {
            0 => Header,
            1 => MobileOriginatedCall,
            2 => MobileTerminatedCall,
            3 => ForwardedCall,
            4 => CallToRoamingSubscriber,
            5 => SupplementaryService,
            6 => HlrInterrogation,
            7 => LocationUpdate,
            8 => SmsMo,
            9 => SmsMt,
            10 => Trailer,
            11 => PstnOriginatedCall,
            12 => PstnTerminatedCall,
            13 => PbxOriginatedCall,
            14 => PbxTerminatedCall,
            15 => UseOfHardware,
            16 => InData1,
            17 => UnsuccessfulCallAttempt,
            18 => InData2,
            19 => InData3,
            20 => DeviceOriginatedCall,
            22 => RemoteChargingControl,
            23 => InForwardedSms,
            24 => CamelOriginatedCall,
            25 => CamelTerminatedCall,
            26 => InData4,
            27 => LocationService,
            28 => InData5,
            29 => Ussd,
            30 => SipOriginatedCall,
            31 => SipTerminatedCall,
            32 => SipOriginatingMessage,
            33 => SipTerminatingMessage,
            35 => SipRegistrationCdr,
            _ => panic!("Invalid record type"),
        };
        Ok(rt)
    }
}

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            RecordType::Header => "Header",
            RecordType::MobileOriginatedCall => "Mobile-Originated Call",
            RecordType::MobileTerminatedCall => "Mobile-Terminated Call",
            RecordType::ForwardedCall => "Forwarded Call",
            RecordType::CallToRoamingSubscriber => "Call to Roaming Subscriber",
            RecordType::SupplementaryService => "Supplementary Service",
            RecordType::HlrInterrogation => "HLR Interrogation",
            RecordType::LocationUpdate => "Location Update",
            RecordType::SmsMo => "SMS (MO)",
            RecordType::SmsMt => "SMS (MT)",
            RecordType::Trailer => "Trailer",
            RecordType::PstnOriginatedCall => "PSTN-Originated Call",
            RecordType::PstnTerminatedCall => "PSTN-Terminated Call",
            RecordType::PbxOriginatedCall => "PBX-Originated Call",
            RecordType::PbxTerminatedCall => "PBX-Terminated Call",
            RecordType::UseOfHardware => "Use of Hardware",
            RecordType::InData1 => "Intelligent Network Data 1",
            RecordType::UnsuccessfulCallAttempt => "Unsuccessful Call Attempt",
            RecordType::InData2 => "Intelligent Network Data 2",
            RecordType::InData3 => "Intelligent Network Data 3",
            RecordType::DeviceOriginatedCall => "Device-Originated Call",
            RecordType::RemoteChargingControl => "Remote Charging Control",
            RecordType::InForwardedSms => "IN-Forwarded SMS",
            RecordType::CamelOriginatedCall => "Camel-Originated Call",
            RecordType::CamelTerminatedCall => "Camel-Terminated Call",
            RecordType::InData4 => "Intelligent Network Data 4",
            RecordType::LocationService => "Location Service",
            RecordType::InData5 => "Intelligent Network Data 5",
            RecordType::Ussd => "USSD",
            RecordType::SipOriginatedCall => "SIP-Originated Call",
            RecordType::SipTerminatedCall => "SIP-Terminated Call",
            RecordType::SipOriginatingMessage => "SIP-Originating Message",
            RecordType::SipTerminatingMessage => "SIP-Terminating Message",
            RecordType::SipRegistrationCdr => "SIP Registration CDR",
        };
        write!(f, "{label}")
    }
}

pub enum RecordStatus {
    Normal,
    SynchronisingError,
    DifferentContents,
}
impl TryFrom<u8> for RecordStatus {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RecordStatus::Normal),
            1 => Ok(RecordStatus::SynchronisingError),
            2 => Ok(RecordStatus::DifferentContents),
            _ => Err("Invalid record status"),
        }
    }
}
impl fmt::Display for RecordStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RecordStatus::Normal => "Normal",
            RecordStatus::SynchronisingError => "Synchronising Error",
            RecordStatus::DifferentContents => "Different Contents",
        };
        write!(f, "{s}")
    }
}   

pub struct CallReference {
    // word + word + byte
    pub value: String,
}
impl CallReference {
    pub fn new(bytes: &[u8]) ->  CallReference {
        let comp = BCDWord::new(&bytes[0..2]).value;
        let process = BCDWord::new(&bytes[2..4]).value;
        let focus = bytes[4];
        CallReference { value: format!("comp:{} process:{:04} focus:{:02}", comp, process, focus)}
    }
}

pub struct ExchangeId {
    pub value: String,
}

impl ExchangeId {
    pub fn new(bytes: &[u8]) -> ExchangeId {
        ExchangeId { value: decode_bcds(bytes) }
    }
}

pub fn decode_bcds(bcd_bytes: &[u8]) -> String {
    let mut decoded = String::new(); 
    for &byte in bcd_bytes.iter() {
        if byte == 0xFF {
            continue;  // skip 0xFF
        }
        let low = (byte >> 4) & 0b0000_1111; 
        let high = byte &  0b0000_1111; 
        decoded.push_str(&format!("{}{}", high, low)); 
    }
    decoded 
}

// intermediate_record_number

// intermediate_charging_ind

// number_of_ss_records

// calling_imsi

// calling_imei

// calling_number

// calling_category

// calling_ms_classmark

// called_imsi

// called_imei

// called_number_ton

// called_number

// called_category

// called_ms_classmark

// dialled_digits_ton

// dialled_digits

// calling_subs_first_lac

// calling_subs_first_ci

// calling_subs_last_ex_id

// calling_subs_last_lac

// calling_subs_last_ci

// called_subs_first_lac

// called_subs_first_ci

// called_subs_last_ex_id

// called_subs_last_lac

// called_subs_last_ci

// out_circuit_group

// out_circuit

// basic_service_type

// basic_service_code

// facility_usage

// non_transparency_indicator

// channel_rate_indicator

// set_up_start_time

// in_channel_allocated_time

// b_idle_time

// charging_start_time

// charging_end_time

// cause_for_termination

// data_volume

// call_type

// dtmf_indicator

// aoc_indicator

// pni

// redirected_indicator

// cdb_indicator

// orig_mcz_chrg_type

// orig_mcz_duration

// orig_mcz_duration_ten_ms

// orig_mcz_tariff_class

// orig_mcz_pulses

// orig_mcz_change_percent

// orig_mcz_change_direction

// called_msrn_ton

// called_msrn

// calling_charging_area

// called_charging_area

// called_msrn_npi

// calling_number_ton

// calling_number_npi

// called_number_npi

// dialled_digits_npi

// connected_to_number_ton

// connected_to_number_npi

// connected_to_number

// cug_interlock

// cug_outgoing_access

// hot_billing_record_number

// number_of_in_records

// regional_subs_indicator

// regional_subs_location_type

// leg_call_reference

// answer time

// char_band_chrg_type

// char_band_duration

// char_band_duration_ten_ms

// char_band_tariff_class

// charg_band_pulses

// char_band_change_percent

// char_band_change_direction

// charge_number_ton

// charge_number_npi

// charge_number

// charge_nature

// oli

// tns_circuit_code

// tns_carrier_code

// cip_carrier_code

// carrier_selection

// pic

// routing_category

// speech_version

// ms_classmark3

// calling_cell_band

// req_fixed_network_user_rate

// req_other_modem_type

// acceptable_channel_codings
pub struct AcceptableChannelCodings {
    pub value: String,
}
impl AcceptableChannelCodings {
    pub fn new(byte: u8) -> AcceptableChannelCodings {
        let mut acceptable_codings = Vec::new();
        let values = [
            "4,8",  // bit 1
            "9,6",  // bit 2
            "14,4", // bit 3
            "", // bit 4 not used
            "28,8", // bit 5
            "32,0", // bit 6
            "43,2", // bit 7
        ];
        for i in 0..7 {
            if byte & (1 << i) != 0 {
                acceptable_codings.push(values[i]);
            }
        }
        if acceptable_codings.len() > 0 {
            acceptable_codings.push("kbit/s");
        }
        AcceptableChannelCodings { value: acceptable_codings.join(" ") }
    }
}


// req_number_of_channels

// req_air_interface_user_rate

// req_user_initiated_mod_ind

// used_number_of_channels

// used_other_modem_type

// used_fixed_nw_user_rate

// used_channel_coding

// intermediate_chrg_cause

// cug_information

// in_category_key

// camel_call_reference

// camel_exchange_id_ton

// camel_exchange_id

// orig_mcz_tariff_change_cnt

// char_band_tariff_change_cnt

// calling_modify_parameters

// orig_mcz_modify_percent

// orig_mcz_modify_direction

// orig_dialling_class

// npdb_query_status

// loc_routing_number

// scp_connection

// number_of_all_in_records

// loc_routing_number_ton

// add_routing_category

// calling_subs_last_ex_id_ton

// called_subs_last_ex_id_ton

// calling_subs_first_mcc

// calling_subs_first_mnc

// calling_subs_last_mcc

// calling_subs_last_mnc

// called_subs_first_mcc

// called_subs_first_mnc

// called_subs_last_mcc

// called_subs_last_mnc

// radio_network_type

// used_air_interface_user_rate

// stream_identifier

// call_reference_time

// outside_user_plane_index

// outside_control_plane_index

// out_bnc_connection_type

// emergency_call_category

// rate_adaption

// jip

// global_call_reference

// virtual_msc_id

// scf_address_ton

// scf_address

// destination_number_ton

// destination_number_npi

// destination_number

// outpulsed_number_ton

// outpulsed_number_npi

// outpulsed_number

// optimal_routing_indicator

// calling_imeisv

// called_imeisv

// out_circuit_group_name

// in_circuit_group

// in_circuit

// virtual_msc_id_ton

// virtual_msc_id_npi

// in_circuit_group_name

// disconnecting_party