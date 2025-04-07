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


