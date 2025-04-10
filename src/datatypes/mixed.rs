use crate::datatypes::primitives::*;
use std::convert::TryFrom;
use std::fmt;

pub enum IntermediateChargingInd {
    Normal,
    Intermediate,
    LastPartial,
    NotUsed,
    None,
}

impl TryFrom<u8> for IntermediateChargingInd {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(IntermediateChargingInd::Normal),
            1 => Ok(IntermediateChargingInd::Intermediate),
            2 => Ok(IntermediateChargingInd::LastPartial),
            0xFF => Ok(IntermediateChargingInd::NotUsed),
            _ => Err("Invalid value"),
        }
    }
}

impl IntermediateChargingInd {
    // Direct method to return the string value
    pub fn value(&self) -> &str {
        match self {
            IntermediateChargingInd::Normal => "Normal",
            IntermediateChargingInd::Intermediate => "Intermediate",
            IntermediateChargingInd::LastPartial => "LastPartial",
            IntermediateChargingInd::NotUsed => "NotUsed",
            IntermediateChargingInd::None => "None",
        }
    }
}

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
        let rt = match value {
            0 => RecordType::Header,
            1 => RecordType::MobileOriginatedCall,
            2 => RecordType::MobileTerminatedCall,
            3 => RecordType::ForwardedCall,
            4 => RecordType::CallToRoamingSubscriber,
            5 => RecordType::SupplementaryService,
            6 => RecordType::HlrInterrogation,
            7 => RecordType::LocationUpdate,
            8 => RecordType::SmsMo,
            9 => RecordType::SmsMt,
            10 => RecordType::Trailer,
            11 => RecordType::PstnOriginatedCall,
            12 => RecordType::PstnTerminatedCall,
            13 => RecordType::PbxOriginatedCall,
            14 => RecordType::PbxTerminatedCall,
            15 => RecordType::UseOfHardware,
            16 => RecordType::InData1,
            17 => RecordType::UnsuccessfulCallAttempt,
            18 => RecordType::InData2,
            19 => RecordType::InData3,
            20 => RecordType::DeviceOriginatedCall,
            22 => RecordType::RemoteChargingControl,
            23 => RecordType::InForwardedSms,
            24 => RecordType::CamelOriginatedCall,
            25 => RecordType::CamelTerminatedCall,
            26 => RecordType::InData4,
            27 => RecordType::LocationService,
            28 => RecordType::InData5,
            29 => RecordType::Ussd,
            30 => RecordType::SipOriginatedCall,
            31 => RecordType::SipTerminatedCall,
            32 => RecordType::SipOriginatingMessage,
            33 => RecordType::SipTerminatingMessage,
            35 => RecordType::SipRegistrationCdr,
            _ => return Err(()), // Invalid value, return an error
        };
        Ok(rt)
    }
}

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RecordType::Header => "Header",
            RecordType::MobileOriginatedCall => "MobileOriginatedCall",
            RecordType::MobileTerminatedCall => "MobileTerminatedCall",
            RecordType::ForwardedCall => "ForwardedCall",
            RecordType::CallToRoamingSubscriber => "CallToRoamingSubscriber",
            RecordType::SupplementaryService => "SupplementaryService",
            RecordType::HlrInterrogation => "HlrInterrogation",
            RecordType::LocationUpdate => "LocationUpdate",
            RecordType::SmsMo => "SmsMo",
            RecordType::SmsMt => "SmsMt",
            RecordType::Trailer => "Trailer",
            RecordType::PstnOriginatedCall => "PstnOriginatedCall",
            RecordType::PstnTerminatedCall => "PstnTerminatedCall",
            RecordType::PbxOriginatedCall => "PbxOriginatedCall",
            RecordType::PbxTerminatedCall => "PbxTerminatedCall",
            RecordType::UseOfHardware => "UseOfHardware",
            RecordType::InData1 => "InData1",
            RecordType::UnsuccessfulCallAttempt => "UnsuccessfulCallAttempt",
            RecordType::InData2 => "InData2",
            RecordType::InData3 => "InData3",
            RecordType::DeviceOriginatedCall => "DeviceOriginatedCall",
            RecordType::RemoteChargingControl => "RemoteChargingControl",
            RecordType::InForwardedSms => "InForwardedSms",
            RecordType::CamelOriginatedCall => "CamelOriginatedCall",
            RecordType::CamelTerminatedCall => "CamelTerminatedCall",
            RecordType::InData4 => "InData4",
            RecordType::LocationService => "LocationService",
            RecordType::InData5 => "InData5",
            RecordType::Ussd => "Ussd",
            RecordType::SipOriginatedCall => "SipOriginatedCall",
            RecordType::SipTerminatedCall => "SipTerminatedCall",
            RecordType::SipOriginatingMessage => "SipOriginatingMessage",
            RecordType::SipTerminatingMessage => "SipTerminatingMessage",
            RecordType::SipRegistrationCdr => "SipRegistrationCdr",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug)]
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
    pub fn new(bytes: &[u8]) -> CallReference {
        let comp = BCDWord::new(&bytes[0..2]).value;
        let process = BCDWord::new(&bytes[2..4]).value;
        let mut focus: u8 = 0;
        if bytes[4] != 0xFF {
            focus = bytes[4];
        }
        CallReference {
            value: format!("comp:{} process:{:04} focus:{:02}", comp, process, focus),
        }
    }
}

pub struct ExchangeId {
    pub value: String,
}

impl ExchangeId {
    pub fn new(bytes: &[u8]) -> ExchangeId {
        ExchangeId {
            value: decode_bcds(bytes),
        }
    }
}

pub fn decode_bcds(bcd_bytes: &[u8]) -> String {
    let mut decoded = String::new();
    for &byte in bcd_bytes.iter() {
        if byte == 0xFF {
            continue; // skip 0xFF
        }
        let low = (byte >> 4) & 0b0000_1111;
        let high = byte & 0b0000_1111;
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
            "",     // bit 4 not used
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
        AcceptableChannelCodings {
            value: acceptable_codings.join(" "),
        }
    }
}

#[derive(Debug)]
enum SelectedCodec {
    FullRateCodecGSM = 0x00,              // Full rate codec for GSM
    HalfRateCodecGSM = 0x01,              // Half rate codec for GSM
    EnhancedFullRateCodecGSM = 0x02,      // Enhanced full rate codec for GSM
    NarrowbandFullRateAmrCodecGSM = 0x03, // Narrowband full rate AMR codec for GSM
    NarrowbandHalfRateAmrCodecGSM = 0x04, // Narrowband half rate AMR codec for GSM
    NarrowbandAmrCodecUmts20ms = 0x05,    // Narrowband AMR codec for UMTS with 20 ms Codec Mode
    NarrowbandAmrCodecUmts40ms = 0x06,    // Narrowband AMR codec for UMTS with 40 ms Codec Mode
    Spare1 = 0x0E,                        // Spare
    Spare2 = 0x0F,                        // Spare
    PcmALaw64k = 0x10,                    // 64 kbps PCM coding with A-law
    PcmULaw64k = 0x11,                    // 64 kbps PCM coding with U-law
    ItuT53k63kCodec = 0x12, // ITU-T specified dual-rate speech codec at 5.3 and 6.3 kbit/s
    ItuTG7231Codec = 0x13,  // ITU-T dual-rate speech codec at 5.3 and 6.3 kbit/s with G.723.1
    ItuT8kCodec = 0x14,     // ITU-T widely used 8 kbit/s codec
    ItuTG729ACodec = 0x15,  // ITU-T widely used 8 kbit/s codec with G.729A
    InternetLowBitRateCodec = 0x16, // Internet low bit-rate codec
    ComfortNoise = 0x17,    // Comfort noise
    FchRtp = 0xF0,          // FCH Real-time Transport Protocol
    FdhClearmode = 0xFD,    // FDHClearmode
}

impl SelectedCodec {
    // Method to get the string description for the variant
    pub fn value(&self) -> &'static str {
        match self {
            SelectedCodec::FullRateCodecGSM => "Full rate codec for GSM",
            SelectedCodec::HalfRateCodecGSM => "Half rate codec for GSM",
            SelectedCodec::EnhancedFullRateCodecGSM => "Enhanced full rate codec for GSM",
            SelectedCodec::NarrowbandFullRateAmrCodecGSM => {
                "Narrowband full rate AMR codec for GSM"
            }
            SelectedCodec::NarrowbandHalfRateAmrCodecGSM => {
                "Narrowband half rate AMR codec for GSM"
            }
            SelectedCodec::NarrowbandAmrCodecUmts20ms => {
                "Narrowband AMR codec for UMTS with 20 ms Codec Mode"
            }
            SelectedCodec::NarrowbandAmrCodecUmts40ms => {
                "Narrowband AMR codec for UMTS with 40 ms Codec Mode"
            }
            SelectedCodec::Spare1 => "Spare",
            SelectedCodec::Spare2 => "Spare",
            SelectedCodec::PcmALaw64k => "64 kbps PCM coding with A-law",
            SelectedCodec::PcmULaw64k => "64 kbps PCM coding with U-law",
            SelectedCodec::ItuT53k63kCodec => {
                "ITU-T specified dual-rate speech codec at 5.3 and 6.3 kbit/s"
            }
            SelectedCodec::ItuTG7231Codec => {
                "ITU-T dual-rate speech codec at 5.3 and 6.3 kbit/s with G.723.1"
            }
            SelectedCodec::ItuT8kCodec => "ITU-T widely used 8 kbit/s codec",
            SelectedCodec::ItuTG729ACodec => "ITU-T widely used 8 kbit/s codec with G.729A",
            SelectedCodec::InternetLowBitRateCodec => "Internet low bit-rate codec",
            SelectedCodec::ComfortNoise => "Comfort noise",
            SelectedCodec::FchRtp => "FCH Real-time Transport Protocol",
            SelectedCodec::FdhClearmode => "FDHClearmode",
        }
    }
}

impl TryFrom<u8> for SelectedCodec {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(SelectedCodec::FullRateCodecGSM),
            0x01 => Ok(SelectedCodec::HalfRateCodecGSM),
            0x02 => Ok(SelectedCodec::EnhancedFullRateCodecGSM),
            0x03 => Ok(SelectedCodec::NarrowbandFullRateAmrCodecGSM),
            0x04 => Ok(SelectedCodec::NarrowbandHalfRateAmrCodecGSM),
            0x05 => Ok(SelectedCodec::NarrowbandAmrCodecUmts20ms),
            0x06 => Ok(SelectedCodec::NarrowbandAmrCodecUmts40ms),
            0x0E => Ok(SelectedCodec::Spare1),
            0x0F => Ok(SelectedCodec::Spare2),
            0x10 => Ok(SelectedCodec::PcmALaw64k),
            0x11 => Ok(SelectedCodec::PcmULaw64k),
            0x12 => Ok(SelectedCodec::ItuT53k63kCodec),
            0x13 => Ok(SelectedCodec::ItuTG7231Codec),
            0x14 => Ok(SelectedCodec::ItuT8kCodec),
            0x15 => Ok(SelectedCodec::ItuTG729ACodec),
            0x16 => Ok(SelectedCodec::InternetLowBitRateCodec),
            0x17 => Ok(SelectedCodec::ComfortNoise),
            0xF0 => Ok(SelectedCodec::FchRtp),
            0xFD => Ok(SelectedCodec::FdhClearmode),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum Action {
    Registration = 0x00,                           // Registration
    Erasure = 0x01,                                // Erasure
    Activation = 0x02,                             // Activation
    Deactivation = 0x03,                           // Deactivation
    Interrogation = 0x04,                          // Interrogation
    Invocation = 0x05,                             // Invocation
    PasswordRegistration = 0x06,                   // Password registration
    Phase1ProcessUnstructuredSSData = 0x07,        // Phase 1 process unstructured SS data
    Phase2ProcessUnstructuredSSDataRequest = 0x08, // Phase 2 process unstructured SS data request
    Phase2ProcessUnstructuredSSDataNotify = 0x09,  // Phase 2 process unstructured SS data notify
}

impl Action {
    // Method to get the string description for the enum variant
    pub fn value(&self) -> &'static str {
        match self {
            Action::Registration => "Registration",
            Action::Erasure => "Erasure",
            Action::Activation => "Activation",
            Action::Deactivation => "Deactivation",
            Action::Interrogation => "Interrogation",
            Action::Invocation => "Invocation",
            Action::PasswordRegistration => "Password registration",
            Action::Phase1ProcessUnstructuredSSData => "Phase 1 process unstructured SS data",
            Action::Phase2ProcessUnstructuredSSDataRequest => {
                "Phase 2 process unstructured SS data request"
            }
            Action::Phase2ProcessUnstructuredSSDataNotify => {
                "Phase 2 process unstructured SS data notify"
            }
        }
    }
}

impl TryFrom<u8> for Action {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Action::Registration),
            0x01 => Ok(Action::Erasure),
            0x02 => Ok(Action::Activation),
            0x03 => Ok(Action::Deactivation),
            0x04 => Ok(Action::Interrogation),
            0x05 => Ok(Action::Invocation),
            0x06 => Ok(Action::PasswordRegistration),
            0x07 => Ok(Action::Phase1ProcessUnstructuredSSData),
            0x08 => Ok(Action::Phase2ProcessUnstructuredSSDataRequest),
            0x09 => Ok(Action::Phase2ProcessUnstructuredSSDataNotify),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum ApplicationInfo {
    NormalShortMessage = 0x00, // Normal short message
    PictureMessage = 0x01,     // Picture message
    NotKnown = 0xFF,           // Not known
}

impl ApplicationInfo {
    // Method to get the string description for the enum variant
    pub fn value(&self) -> &'static str {
        match self {
            ApplicationInfo::NormalShortMessage => "Normal short message",
            ApplicationInfo::PictureMessage => "Picture message",
            ApplicationInfo::NotKnown => "Not known",
        }
    }
}

impl TryFrom<u8> for ApplicationInfo {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ApplicationInfo::NormalShortMessage),
            0x01 => Ok(ApplicationInfo::PictureMessage),
            0xFF => Ok(ApplicationInfo::NotKnown),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum TeleserviceCode {
    AllTeleservices = 0x00,            // All teleservices
    SpeechTransmission = 0x10,         // Speech transmission
    Telephony = 0x11,                  // Telephony
    EmergencyCalls = 0x12,             // Emergency calls
    ShortMessageServices = 0x20,       // Short message services
    ShortMessageMTPP = 0x21,           // Short message MT/PP
    ShortMessageMOPP = 0x22,           // Short message MO/PP
    DataMHS = 0x30,                    // Data MHS
    AdvancedMHSAccess = 0x31,          // Advanced MHS access
    VideotexAccess = 0x40,             // Videotex access services
    VideotexProfile1 = 0x41,           // Videotex access profile 1
    VideotexProfile2 = 0x42,           // Videotex access profile 2
    VideotexProfile3 = 0x43,           // Videotex access profile 3
    TeletexService = 0x50,             // Teletex service
    TeletexCS = 0x51,                  // Teletex CS
    TeletexPS = 0x52,                  // Teletex PS
    Facsimile = 0x60,                  // Facsimile
    FacsimileGroup3AlterSpeech = 0x61, // Facsimile Group 3 and alter speech
    AutomaticFacsimileGroup3 = 0x62,   // Automatic facsimile Group 3
    DualNumbering = 0xD1,              // Dual numbering (alternate line service)
}

impl TeleserviceCode {
    // Method to return the string description of the enum variant
    pub fn value(&self) -> &'static str {
        match self {
            TeleserviceCode::AllTeleservices => "All teleservices",
            TeleserviceCode::SpeechTransmission => "Speech transmission",
            TeleserviceCode::Telephony => "Telephony",
            TeleserviceCode::EmergencyCalls => "Emergency calls",
            TeleserviceCode::ShortMessageServices => "Short message services",
            TeleserviceCode::ShortMessageMTPP => "Short message MT/PP",
            TeleserviceCode::ShortMessageMOPP => "Short message MO/PP",
            TeleserviceCode::DataMHS => "Data MHS",
            TeleserviceCode::AdvancedMHSAccess => "Advanced MHS access",
            TeleserviceCode::VideotexAccess => "Videotex access services",
            TeleserviceCode::VideotexProfile1 => "Videotex access profile 1",
            TeleserviceCode::VideotexProfile2 => "Videotex access profile 2",
            TeleserviceCode::VideotexProfile3 => "Videotex access profile 3",
            TeleserviceCode::TeletexService => "Teletex service",
            TeleserviceCode::TeletexCS => "Teletex CS",
            TeleserviceCode::TeletexPS => "Teletex PS",
            TeleserviceCode::Facsimile => "Facsimile",
            TeleserviceCode::FacsimileGroup3AlterSpeech => "Facsimile Group 3 and alter speech",
            TeleserviceCode::AutomaticFacsimileGroup3 => "Automatic facsimile Group 3",
            TeleserviceCode::DualNumbering => "Dual numbering (alternate line service)",
        }
    }
}

impl TryFrom<u8> for TeleserviceCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(TeleserviceCode::AllTeleservices),
            0x10 => Ok(TeleserviceCode::SpeechTransmission),
            0x11 => Ok(TeleserviceCode::Telephony),
            0x12 => Ok(TeleserviceCode::EmergencyCalls),
            0x20 => Ok(TeleserviceCode::ShortMessageServices),
            0x21 => Ok(TeleserviceCode::ShortMessageMTPP),
            0x22 => Ok(TeleserviceCode::ShortMessageMOPP),
            0x30 => Ok(TeleserviceCode::DataMHS),
            0x31 => Ok(TeleserviceCode::AdvancedMHSAccess),
            0x40 => Ok(TeleserviceCode::VideotexAccess),
            0x41 => Ok(TeleserviceCode::VideotexProfile1),
            0x42 => Ok(TeleserviceCode::VideotexProfile2),
            0x43 => Ok(TeleserviceCode::VideotexProfile3),
            0x50 => Ok(TeleserviceCode::TeletexService),
            0x51 => Ok(TeleserviceCode::TeletexCS),
            0x52 => Ok(TeleserviceCode::TeletexPS),
            0x60 => Ok(TeleserviceCode::Facsimile),
            0x61 => Ok(TeleserviceCode::FacsimileGroup3AlterSpeech),
            0x62 => Ok(TeleserviceCode::AutomaticFacsimileGroup3),
            0xD1 => Ok(TeleserviceCode::DualNumbering),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum BearerServiceCode {
    AllBearerServices = 0x00,       // All bearer services
    KHz31Group = 0x10,              // 3.1 kHz group
    KHz31ExPLMN = 0x11,             // 3.1 kHz ex PLMN
    AlternateSpeech = 0x12,         // Alternate/speech
    SpeechFollowedByKHz31 = 0x13,   // Speech followed by 3.1 kHz
    DataCDA = 0x20,                 // Data c.d.a
    DataCDA300bps = 0x21,           // Data c.d.a 300 b/s
    DataCDA1200bps = 0x22,          // Data c.d.a 1200 b/s
    DataCDA120075bps = 0x23,        // Data c.d.a 1200-75 b/s
    DataCDA2400bps = 0x24,          // Data c.d.a 2400 b/s
    DataCDA4800bps = 0x25,          // Data c.d.a 4800 b/s
    DataCDA9600bps = 0x26,          // Data c.d.a 9600 b/s
    DataCDAGeneral = 0x27,          // Data c.d.a general
    DataCDS = 0x30,                 // Data c.d.s
    DataCDS1200bps = 0x32,          // Data c.d.s 1200 b/s
    DataCDS2400bps = 0x34,          // Data c.d.s 2400 b/s
    DataCDS4800bps = 0x35,          // Data c.d.s 4800 b/s
    DataCDS9600bps = 0x36,          // Data c.d.s 9600 b/s
    DataCDSGeneral = 0x37,          // Data c.d.s general
    PADAccessCDA = 0x40,            // PAD access c.d.a
    PADAccessCDA300bps = 0x41,      // PAD access c.d.a 300 b/s
    PADAccessCDA1200bps = 0x42,     // PAD access c.d.a 1200 b/s
    PADAccessCDA120075bps = 0x43,   // PAD access c.d.a 1200-75 b/s
    PADAccessCDA2400bps = 0x44,     // PAD access c.d.a 2400 b/s
    PADAccessCDA4800bps = 0x45,     // PAD access c.d.a 4800 b/s
    PADAccessCDA9600bps = 0x46,     // PAD access c.d.a 9600 b/s
    PADAccessCDAGeneral = 0x47,     // PAD access c.d.a general
    DataPDS = 0x50,                 // Data p.d.s
    DataPDS2400bps = 0x54,          // Data p.d.s 2400 b/s
    DataPDS4800bps = 0x55,          // Data p.d.s 4800 b/s
    DataPDS9600bps = 0x56,          // Data p.d.s 9600 b/s
    DataPDSGeneral = 0x57,          // Data p.d.s general
    AlternateSpeechDataCDA = 0x60,  // Alternate speech/data c.d.a
    AlternateSpeechDataCDS = 0x70,  // Alternate speech/data c.d.s
    SpeechFollowedByDataCDA = 0x80, // Speech followed by data c.d.a
    SpeechFollowedByDataCDS = 0x90, // Speech followed by data c.d.s
    ServiceNotUsed = 0xFF,          // Service not used
}

impl BearerServiceCode {
    // Method to return the string description of the enum variant
    pub fn value(&self) -> &'static str {
        match self {
            BearerServiceCode::AllBearerServices => "All bearer services",
            BearerServiceCode::KHz31Group => "3.1 kHz group",
            BearerServiceCode::KHz31ExPLMN => "3.1 kHz ex PLMN",
            BearerServiceCode::AlternateSpeech => "Alternate/speech",
            BearerServiceCode::SpeechFollowedByKHz31 => "Speech followed by 3.1 kHz",
            BearerServiceCode::DataCDA => "Data c.d.a",
            BearerServiceCode::DataCDA300bps => "Data c.d.a 300 b/s",
            BearerServiceCode::DataCDA1200bps => "Data c.d.a 1200 b/s",
            BearerServiceCode::DataCDA120075bps => "Data c.d.a 1200-75 b/s",
            BearerServiceCode::DataCDA2400bps => "Data c.d.a 2400 b/s",
            BearerServiceCode::DataCDA4800bps => "Data c.d.a 4800 b/s",
            BearerServiceCode::DataCDA9600bps => "Data c.d.a 9600 b/s",
            BearerServiceCode::DataCDAGeneral => "Data c.d.a general",
            BearerServiceCode::DataCDS => "Data c.d.s",
            BearerServiceCode::DataCDS1200bps => "Data c.d.s 1200 b/s",
            BearerServiceCode::DataCDS2400bps => "Data c.d.s 2400 b/s",
            BearerServiceCode::DataCDS4800bps => "Data c.d.s 4800 b/s",
            BearerServiceCode::DataCDS9600bps => "Data c.d.s 9600 b/s",
            BearerServiceCode::DataCDSGeneral => "Data c.d.s general",
            BearerServiceCode::PADAccessCDA => "PAD access c.d.a",
            BearerServiceCode::PADAccessCDA300bps => "PAD access c.d.a 300 b/s",
            BearerServiceCode::PADAccessCDA1200bps => "PAD access c.d.a 1200 b/s",
            BearerServiceCode::PADAccessCDA120075bps => "PAD access c.d.a 1200-75 b/s",
            BearerServiceCode::PADAccessCDA2400bps => "PAD access c.d.a 2400 b/s",
            BearerServiceCode::PADAccessCDA4800bps => "PAD access c.d.a 4800 b/s",
            BearerServiceCode::PADAccessCDA9600bps => "PAD access c.d.a 9600 b/s",
            BearerServiceCode::PADAccessCDAGeneral => "PAD access c.d.a general",
            BearerServiceCode::DataPDS => "Data p.d.s",
            BearerServiceCode::DataPDS2400bps => "Data p.d.s 2400 b/s",
            BearerServiceCode::DataPDS4800bps => "Data p.d.s 4800 b/s",
            BearerServiceCode::DataPDS9600bps => "Data p.d.s 9600 b/s",
            BearerServiceCode::DataPDSGeneral => "Data p.d.s general",
            BearerServiceCode::AlternateSpeechDataCDA => "Alternate speech/data c.d.a",
            BearerServiceCode::AlternateSpeechDataCDS => "Alternate speech/data c.d.s",
            BearerServiceCode::SpeechFollowedByDataCDA => "Speech followed by data c.d.a",
            BearerServiceCode::SpeechFollowedByDataCDS => "Speech followed by data c.d.s",
            BearerServiceCode::ServiceNotUsed => "Service not used",
        }
    }
}

impl TryFrom<u8> for BearerServiceCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(BearerServiceCode::AllBearerServices),
            0x10 => Ok(BearerServiceCode::KHz31Group),
            0x11 => Ok(BearerServiceCode::KHz31ExPLMN),
            0x12 => Ok(BearerServiceCode::AlternateSpeech),
            0x13 => Ok(BearerServiceCode::SpeechFollowedByKHz31),
            0x20 => Ok(BearerServiceCode::DataCDA),
            0x21 => Ok(BearerServiceCode::DataCDA300bps),
            0x22 => Ok(BearerServiceCode::DataCDA1200bps),
            0x23 => Ok(BearerServiceCode::DataCDA120075bps),
            0x24 => Ok(BearerServiceCode::DataCDA2400bps),
            0x25 => Ok(BearerServiceCode::DataCDA4800bps),
            0x26 => Ok(BearerServiceCode::DataCDA9600bps),
            0x27 => Ok(BearerServiceCode::DataCDAGeneral),
            0x30 => Ok(BearerServiceCode::DataCDS),
            0x32 => Ok(BearerServiceCode::DataCDS1200bps),
            0x34 => Ok(BearerServiceCode::DataCDS2400bps),
            0x35 => Ok(BearerServiceCode::DataCDS4800bps),
            0x36 => Ok(BearerServiceCode::DataCDS9600bps),
            0x37 => Ok(BearerServiceCode::DataCDSGeneral),
            0x40 => Ok(BearerServiceCode::PADAccessCDA),
            0x41 => Ok(BearerServiceCode::PADAccessCDA300bps),
            0x42 => Ok(BearerServiceCode::PADAccessCDA1200bps),
            0x43 => Ok(BearerServiceCode::PADAccessCDA120075bps),
            0x44 => Ok(BearerServiceCode::PADAccessCDA2400bps),
            0x45 => Ok(BearerServiceCode::PADAccessCDA4800bps),
            0x46 => Ok(BearerServiceCode::PADAccessCDA9600bps),
            0x47 => Ok(BearerServiceCode::PADAccessCDAGeneral),
            0x50 => Ok(BearerServiceCode::DataPDS),
            0x54 => Ok(BearerServiceCode::DataPDS2400bps),
            0x55 => Ok(BearerServiceCode::DataPDS4800bps),
            0x56 => Ok(BearerServiceCode::DataPDS9600bps),
            0x57 => Ok(BearerServiceCode::DataPDSGeneral),
            0x60 => Ok(BearerServiceCode::AlternateSpeechDataCDA),
            0x70 => Ok(BearerServiceCode::AlternateSpeechDataCDS),
            0x80 => Ok(BearerServiceCode::SpeechFollowedByDataCDA),
            0x90 => Ok(BearerServiceCode::SpeechFollowedByDataCDS),
            0xFF => Ok(BearerServiceCode::ServiceNotUsed),
            _ => Err(()),
        }
    }
}
#[derive(Debug)]
enum ChargingBlockSize {
    KB2 = 0x00,  // 2 kB
    KB8 = 0x01,  // 8 kB
    KB16 = 0x02, // 16 kB
    KB32 = 0x04, // 32 kB
    KB64 = 0x08, // 64 kB
}

impl ChargingBlockSize {
    // Method to return the string description of the enum variant
    pub fn value(&self) -> &'static str {
        match self {
            ChargingBlockSize::KB2 => "2 kB",
            ChargingBlockSize::KB8 => "8 kB",
            ChargingBlockSize::KB16 => "16 kB",
            ChargingBlockSize::KB32 => "32 kB",
            ChargingBlockSize::KB64 => "64 kB",
        }
    }
}

impl TryFrom<u8> for ChargingBlockSize {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(ChargingBlockSize::KB2),
            0x01 => Ok(ChargingBlockSize::KB8),
            0x02 => Ok(ChargingBlockSize::KB16),
            0x04 => Ok(ChargingBlockSize::KB32),
            0x08 => Ok(ChargingBlockSize::KB64),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum ChargeType {
    ChargeableCall = 0x00,                               // 00H chargeable call
    FreeFromAnalysis = 0x08,                             // 08H free from analysis
    FreeFromAddressCompleteMessage = 0x10,               // 10H free from address complete message
    FreeFromAnswerMessage = 0x20,                        // 20H free from answer message
    FreeFromAnalysisAndACM = 0x18,                       // 18H free from analysis and ACM
    FreeFromAnalysisAndAnswerMessage = 0x28, // 28H free from analysis and answer message
    FreeFromCallProgressMessage = 0x40,      // 40H free from call progress message
    FreeFromAnalysisAndCallProgressMessage = 0x48, // 48H free from analysis and call progress message
    FreeFromCDB = 0x80,                            // 80H free from CDB
    FreeFromAnalysisAndCDB = 0x88,                 // 88H free from analysis and CDB
    FreeFromACMandCDB = 0x90,                      // 90H free from ACM and CDB
    FreeFromAnalysisAndACMandCDB = 0x98,           // 98H free from analysis, ACM, and CDB
    FreeFromAnswerMessageAndCDB = 0xA0,            // A0H free from answer message and CDB
    FreeFromAnalysisAndAnswerMessageAndCDB = 0xA8, // A8H free from analysis, answer message, and CDB
    FreeFromCallProgressMessageAndCDB = 0xC0,      // C0H free from call progress message and CDB
    FreeFromAnalysisAndCallProgressMessageAndCDB = 0xC8, // C8H free from analysis, call progress message, and CDB
}

impl ChargeType {
    // Method to return the string description of the enum variant
    pub fn value(&self) -> &'static str {
        match self {
            ChargeType::ChargeableCall => "Chargeable call",
            ChargeType::FreeFromAnalysis => "Free from analysis",
            ChargeType::FreeFromAddressCompleteMessage => "Free from address complete message",
            ChargeType::FreeFromAnswerMessage => "Free from answer message",
            ChargeType::FreeFromAnalysisAndACM => "Free from analysis and ACM",
            ChargeType::FreeFromAnalysisAndAnswerMessage => "Free from analysis and answer message",
            ChargeType::FreeFromCallProgressMessage => "Free from call progress message",
            ChargeType::FreeFromAnalysisAndCallProgressMessage => {
                "Free from analysis and call progress message"
            }
            ChargeType::FreeFromCDB => "Free from CDB",
            ChargeType::FreeFromAnalysisAndCDB => "Free from analysis and CDB",
            ChargeType::FreeFromACMandCDB => "Free from ACM and CDB",
            ChargeType::FreeFromAnalysisAndACMandCDB => "Free from analysis, ACM, and CDB",
            ChargeType::FreeFromAnswerMessageAndCDB => "Free from answer message and CDB",
            ChargeType::FreeFromAnalysisAndAnswerMessageAndCDB => {
                "Free from analysis, answer message, and CDB"
            }
            ChargeType::FreeFromCallProgressMessageAndCDB => {
                "Free from call progress message and CDB"
            }
            ChargeType::FreeFromAnalysisAndCallProgressMessageAndCDB => {
                "Free from analysis, call progress message, and CDB"
            }
        }
    }
}

impl TryFrom<u8> for ChargeType {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(ChargeType::ChargeableCall),
            0x08 => Ok(ChargeType::FreeFromAnalysis),
            0x10 => Ok(ChargeType::FreeFromAddressCompleteMessage),
            0x20 => Ok(ChargeType::FreeFromAnswerMessage),
            0x18 => Ok(ChargeType::FreeFromAnalysisAndACM),
            0x28 => Ok(ChargeType::FreeFromAnalysisAndAnswerMessage),
            0x40 => Ok(ChargeType::FreeFromCallProgressMessage),
            0x48 => Ok(ChargeType::FreeFromAnalysisAndCallProgressMessage),
            0x80 => Ok(ChargeType::FreeFromCDB),
            0x88 => Ok(ChargeType::FreeFromAnalysisAndCDB),
            0x90 => Ok(ChargeType::FreeFromACMandCDB),
            0x98 => Ok(ChargeType::FreeFromAnalysisAndACMandCDB),
            0xA0 => Ok(ChargeType::FreeFromAnswerMessageAndCDB),
            0xA8 => Ok(ChargeType::FreeFromAnalysisAndAnswerMessageAndCDB),
            0xC0 => Ok(ChargeType::FreeFromCallProgressMessageAndCDB),
            0xC8 => Ok(ChargeType::FreeFromAnalysisAndCallProgressMessageAndCDB),
            _ => Err(()),
        }
    }
}
#[derive(Debug)]
struct CugInformation {
    value: String,
}

impl CugInformation {
    // Constructor that takes a u8 (byte) and returns the corresponding CUG information
    pub fn from_u8(byte: u8) -> CugInformation {
        let cug_value = match byte {
            0x00 => "00 - Not supported or available",
            0x01 => "01 - Subscribers belong to the same group",
            0x02 => "02 - Subscribers do not belong to the same group",
            0x03 => "03 - Subscribers may belong to the same group; this is not known in the originating side",
            _ => "FF - Invalid or unknown value",
        };

        CugInformation {
            value: cug_value.to_string(),
        }
    }
}

#[derive(Debug)]
struct CommandType {
    value: String,
}

impl CommandType {
    // Constructor for CommandType
    pub fn new(command_value: &str) -> Result<Self, &'static str> {
        // Validate if the input is a valid hexadecimal value (00H to FFH)
        let command_value = match u8::from_str_radix(command_value, 16) {
            Ok(value) => value,
            Err(_) => return Err("Invalid hex value. Must be a valid 2-digit hexadecimal number."),
        };

        // Match the command_value based on the given specification
        match command_value {
            0x00 => Ok(CommandType { value: "00 - Enquiry relating to previously submitted short message".to_string() }),  // Enquiry
            0x01 => Ok(CommandType { value: "01 - Cancel status report request relating to previously submitted short message".to_string() }),  // Cancel status report
            0x02 => Ok(CommandType { value: "02 - Delete previously submitted short message".to_string() }),  // Delete short message
            0x03 => Ok(CommandType { value: "03 - Enable status report request relating to previously submitted short message".to_string() }),  // Enable status report
            0x04..=0x1F => Ok(CommandType { value: format!("{:02X} - Reserved unspecified", command_value) }), // Reserved unspecified
            0x20..=0xDF => Ok(CommandType { value: format!("{:02X} - Not used", command_value) }), // Not used
            0xE0..=0xFF => Ok(CommandType { value: format!("{:02X} - Values specific for each SMSC", command_value) }), // Values specific for each SMSC
            _ => Err("Invalid command type: must be between 00H and FFH, with reserved or specific SMSC values."),
        }
    }
    // Method to get the command type value
    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug)]
struct CugOutgoingAccess {
    value: String,
}
impl CugOutgoingAccess {
    // Constructor that takes a u8 (byte) and returns the corresponding CUG outgoing access information
    pub fn from_u8(cug_access_byte: u8) -> Self {
        let access_value = match cug_access_byte {
            0x00 => "00 - SS did not find the field from network signal or CC tells SS not to put it there",
            0x02 => "02 - Field value unknown to SS (and to DX)",
            0x04 => "04 - Ordinary call",
            0x05 => "05 - Outgoing access allowed",
            0x06 => "06 - Outgoing access not allowed",
            _ => "FF - Invalid or unknown value",
        };

        CugOutgoingAccess {
            value: access_value.to_string(),
        }
    }
    // Method to get the CUG outgoing access description
    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug)]
enum BasicCallStateModel {
    NotDefined = 0x00,            // 00H Type of basic call state model not defined
    OriginatingSide = 0x01,       // 01H Basic call state model for originating side
    TerminatingSide = 0x02,       // 02H Basic call state model for terminating side
    TerminatingGatewayMSC = 0x04, // 04H Basic call state model for terminating gateway MSC
    OriginatingForCallForwarding = 0x05, // 05H Originating basic call state model for call forwarding
    OriginatingForCOBI = 0x06,           // 06H Originating side for COBI call
    TerminatingForCOBI = 0x07,           // 07H Terminating side for COBI call
    BasicForICA = 0x08,                  // 08H Basic call state model for ICA call
    Unknown = 0xFF,                      // FFH Unknown
    OriginatingSMS = 0x03,               // 03H Originating SMS state model
}

impl BasicCallStateModel {
    // Method to return the string description of the enum variant
    pub fn value(&self) -> &'static str {
        match self {
            BasicCallStateModel::NotDefined => "Type of basic call state model not defined",
            BasicCallStateModel::OriginatingSide => "Basic call state model for originating side",
            BasicCallStateModel::TerminatingSide => "Basic call state model for terminating side",
            BasicCallStateModel::TerminatingGatewayMSC => {
                "Basic call state model for terminating gateway MSC"
            }
            BasicCallStateModel::OriginatingForCallForwarding => {
                "Originating basic call state model for call forwarding"
            }
            BasicCallStateModel::OriginatingForCOBI => "Originating side for COBI call",
            BasicCallStateModel::TerminatingForCOBI => "Terminating side for COBI call",
            BasicCallStateModel::BasicForICA => "Basic call state model for ICA call",
            BasicCallStateModel::Unknown => "Unknown",
            BasicCallStateModel::OriginatingSMS => "Originating SMS state model",
        }
    }
}

impl TryFrom<u8> for BasicCallStateModel {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(BasicCallStateModel::NotDefined),
            0x01 => Ok(BasicCallStateModel::OriginatingSide),
            0x02 => Ok(BasicCallStateModel::TerminatingSide),
            0x04 => Ok(BasicCallStateModel::TerminatingGatewayMSC),
            0x05 => Ok(BasicCallStateModel::OriginatingForCallForwarding),
            0x06 => Ok(BasicCallStateModel::OriginatingForCOBI),
            0x07 => Ok(BasicCallStateModel::TerminatingForCOBI),
            0x08 => Ok(BasicCallStateModel::BasicForICA),
            0xFF => Ok(BasicCallStateModel::Unknown),
            0x03 => Ok(BasicCallStateModel::OriginatingSMS),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum BasicServiceType {
    Teleservice = 0x00,   // 00H Teleservice
    BearerService = 0x01, // 01H Bearer service
    NotUsed = 0xFF,       // FFH Not used
}

impl BasicServiceType {
    // Method to return the string description of the enum variant
    pub fn value(&self) -> &'static str {
        match self {
            BasicServiceType::Teleservice => "Teleservice",
            BasicServiceType::BearerService => "Bearer service",
            BasicServiceType::NotUsed => "Not used",
        }
    }
}

impl TryFrom<u8> for BasicServiceType {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(BasicServiceType::Teleservice),
            0x01 => Ok(BasicServiceType::BearerService),
            0xFF => Ok(BasicServiceType::NotUsed),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum BncConnectionType {
    NoConnection = 0x00,   // 00H No connection
    AAL1 = 0x01,           // 01H ATM Adaptation Layer 1 (AAL1)
    AAL2 = 0x02,           // 02H ATM Adaptation Layer 2 (AAL2)
    IP = 0x04,             // 04H Internet Protocol (IP)
    StructuredAAL1 = 0x05, // 05H Structured AAL1
    TDM = 0x08,            // 08H Time Division Multiplex (TDM)
    IPv4 = 0x10,           // 10H Internet Protocol version 4 (IPv4)
    IPv6 = 0x20,           // 20H Internet Protocol version 6 (IPv6)
    NotActive = 0x40,      // 40H Not active
    NotRegistered = 0x80,  // 80H Not registered
    NotDefined = 0xFF,     // FFH Not defined
}

impl BncConnectionType {
    // Method to return the string description of the enum variant
    pub fn value(&self) -> &'static str {
        match self {
            BncConnectionType::NoConnection => "No connection",
            BncConnectionType::AAL1 => "ATM Adaptation Layer 1 (AAL1)",
            BncConnectionType::AAL2 => "ATM Adaptation Layer 2 (AAL2)",
            BncConnectionType::IP => "Internet Protocol (IP)",
            BncConnectionType::StructuredAAL1 => "Structured AAL1",
            BncConnectionType::TDM => "Time Division Multiplex (TDM)",
            BncConnectionType::IPv4 => "Internet Protocol version 4 (IPv4)",
            BncConnectionType::IPv6 => "Internet Protocol version 6 (IPv6)",
            BncConnectionType::NotActive => "Not active",
            BncConnectionType::NotRegistered => "Not registered",
            BncConnectionType::NotDefined => "Not defined",
        }
    }
}

impl TryFrom<u8> for BncConnectionType {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(BncConnectionType::NoConnection),
            0x01 => Ok(BncConnectionType::AAL1),
            0x02 => Ok(BncConnectionType::AAL2),
            0x04 => Ok(BncConnectionType::IP),
            0x05 => Ok(BncConnectionType::StructuredAAL1),
            0x08 => Ok(BncConnectionType::TDM),
            0x10 => Ok(BncConnectionType::IPv4),
            0x20 => Ok(BncConnectionType::IPv6),
            0x40 => Ok(BncConnectionType::NotActive),
            0x80 => Ok(BncConnectionType::NotRegistered),
            0xFF => Ok(BncConnectionType::NotDefined),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum CallMedia {
    DoesNotExist = 0x00, // 00H Doesn't exist
    Speech = 0x01,       // 01H Speech
    Multimedia = 0x02,   // 02H Multimedia
}

impl CallMedia {
    // Method to return the string description of the enum variant
    pub fn value(&self) -> &'static str {
        match self {
            CallMedia::DoesNotExist => "Doesn't exist",
            CallMedia::Speech => "Speech",
            CallMedia::Multimedia => "Multimedia",
        }
    }
}

impl TryFrom<u8> for CallMedia {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(CallMedia::DoesNotExist),
            0x01 => Ok(CallMedia::Speech),
            0x02 => Ok(CallMedia::Multimedia),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum CallState {
    Setup = 0x00,                    // 00H Setup
    ASeized = 0x01,                  // 01H A seized
    BSeized = 0x02,                  // 02H B seized
    SignallingPhaseCompleted = 0x03, // 03H Signalling phase completed
}

impl CallState {
    // Method to return the string description of the enum variant
    pub fn value(&self) -> &'static str {
        match self {
            CallState::Setup => "Setup",
            CallState::ASeized => "A seized",
            CallState::BSeized => "B seized",
            CallState::SignallingPhaseCompleted => "Signalling phase completed",
        }
    }
}

impl TryFrom<u8> for CallState {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(CallState::Setup),
            0x01 => Ok(CallState::ASeized),
            0x02 => Ok(CallState::BSeized),
            0x03 => Ok(CallState::SignallingPhaseCompleted),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum CallType {
    Incoming = 0x00,                        // 00H incoming
    Forwarded = 0x01,                       // 01H forwarded
    ReRouted = 0x02,                        // 02H re-routed
    Outgoing = 0x03,                        // 03H outgoing
    Handover = 0x04,                        // 04H handover
    PortedOut = 0x05,                       // 05H ported-out
    FollowOn = 0x06,                        // 06H follow-on
    TerminatedToAnnouncementMachine = 0x10, // 10H terminated to the announcement machine
    ISUPorSIPTunneling = 0x11,              // 11H ISUP tunneling or SIP tunneling
    InternationalASubscriber = 0x20,        // 20H international A-subscriber
}

impl CallType {
    // Method to return the string description of the enum variant
    pub fn value(&self) -> &'static str {
        match self {
            CallType::Incoming => "incoming",
            CallType::Forwarded => "forwarded",
            CallType::ReRouted => "re-routed",
            CallType::Outgoing => "outgoing",
            CallType::Handover => "handover",
            CallType::PortedOut => "ported-out",
            CallType::FollowOn => "follow-on",
            CallType::TerminatedToAnnouncementMachine => "terminated to the announcement machine",
            CallType::ISUPorSIPTunneling => "ISUP tunneling or SIP tunneling",
            CallType::InternationalASubscriber => "international A-subscriber",
        }
    }
}

impl TryFrom<u8> for CallType {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(CallType::Incoming),
            0x01 => Ok(CallType::Forwarded),
            0x02 => Ok(CallType::ReRouted),
            0x03 => Ok(CallType::Outgoing),
            0x04 => Ok(CallType::Handover),
            0x05 => Ok(CallType::PortedOut),
            0x06 => Ok(CallType::FollowOn),
            0x10 => Ok(CallType::TerminatedToAnnouncementMachine),
            0x11 => Ok(CallType::ISUPorSIPTunneling),
            0x20 => Ok(CallType::InternationalASubscriber),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum CallingPSTNCategory {
    TUP10 = 0x14, // TUP 10 14H
    TUP12 = 0x19, // TUP 12 19H
    TUP14 = 0x00, // TUP 14 00H
    TUP18 = 0x18, // TUP 18 18H
    TUP19 = 0x04, // TUP 19 04H
}

impl CallingPSTNCategory {
    // Method to return the string description of the enum variant
    pub fn value(&self) -> &'static str {
        match self {
            CallingPSTNCategory::TUP10 => "TUP 10 14H",
            CallingPSTNCategory::TUP12 => "TUP 12 19H",
            CallingPSTNCategory::TUP14 => "TUP 14 00H",
            CallingPSTNCategory::TUP18 => "TUP 18 18H",
            CallingPSTNCategory::TUP19 => "TUP 19 04H",
        }
    }
}

impl TryFrom<u8> for CallingPSTNCategory {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x14 => Ok(CallingPSTNCategory::TUP10),
            0x19 => Ok(CallingPSTNCategory::TUP12),
            0x00 => Ok(CallingPSTNCategory::TUP14),
            0x18 => Ok(CallingPSTNCategory::TUP18),
            0x04 => Ok(CallingPSTNCategory::TUP19),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum CarrierSelection {
    NoFieldFound = 0x00, // SS did not find the field from the network signalling
    UnknownToSS = 0x02,  // Field value unknown to SS
    NoIndication = 0x04, // No indication
    PresubscribedNotInput = 0x05, // Presubscribed, not input by calling party
    PresubscribedInput = 0x06, // Presubscribed, input by calling party
    PresubscribedInputUndetermined = 0x07, // Presubscribed, input undetermined
    NotPresubscribedInput = 0x08, // Not presubscribed, input by calling party
}

impl CarrierSelection {
    pub fn description(&self) -> &'static str {
        match self {
            CarrierSelection::NoFieldFound => {
                "SS did not find the field from the network signalling"
            }
            CarrierSelection::UnknownToSS => "Field value unknown to SS",
            CarrierSelection::NoIndication => "No indication",
            CarrierSelection::PresubscribedNotInput => {
                "Selected carrier identification presubscribed and not input by calling party"
            }
            CarrierSelection::PresubscribedInput => {
                "Selected carrier identification presubscribed and input by calling party"
            }
            CarrierSelection::PresubscribedInputUndetermined => {
                "Selected carrier identification presubscribed, input by calling party undetermined"
            }
            CarrierSelection::NotPresubscribedInput => {
                "Selected carrier identification not presubscribed and input by calling party"
            }
        }
    }
}

impl std::convert::TryFrom<u8> for CarrierSelection {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(CarrierSelection::NoFieldFound),
            0x02 => Ok(CarrierSelection::UnknownToSS),
            0x04 => Ok(CarrierSelection::NoIndication),
            0x05 => Ok(CarrierSelection::PresubscribedNotInput),
            0x06 => Ok(CarrierSelection::PresubscribedInput),
            0x07 => Ok(CarrierSelection::PresubscribedInputUndetermined),
            0x08 => Ok(CarrierSelection::NotPresubscribedInput),
            _ => Err(()), // Invalid value
        }
    }
}

#[derive(Debug)]
enum Category {
    Ordinary = 0x00,             // Ordinary
    OrdinaryNoCharge = 0x02,     // Ordinary, no charge
    PayPhone = 0x05,             // Pay phone
    Priority = 0x08,             // Priority
    PriorityNoCharge = 0x0B,     // Priority, no charge
    TestEquipment = 0x10,        // Test equipment
    PrivateNumberService = 0x45, // Private number service (option)
    NotExist = 0xF0,             // Not exist
    Unknown = 0xFF,              // Unknown
}

impl Category {
    pub fn description(&self) -> &'static str {
        match self {
            Category::Ordinary => "Ordinary",
            Category::OrdinaryNoCharge => "Ordinary, no charge",
            Category::PayPhone => "Pay phone",
            Category::Priority => "Priority",
            Category::PriorityNoCharge => "Priority, no charge",
            Category::TestEquipment => "Test equipment",
            Category::PrivateNumberService => "Private number service (option)",
            Category::NotExist => "Not exist",
            Category::Unknown => "Unknown",
        }
    }
}

impl std::convert::TryFrom<u8> for Category {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(Category::Ordinary),
            0x02 => Ok(Category::OrdinaryNoCharge),
            0x05 => Ok(Category::PayPhone),
            0x08 => Ok(Category::Priority),
            0x0B => Ok(Category::PriorityNoCharge),
            0x10 => Ok(Category::TestEquipment),
            0x45 => Ok(Category::PrivateNumberService),
            0xF0 => Ok(Category::NotExist),
            0xFF => Ok(Category::Unknown),
            _ => Err(()), // Invalid value
        }
    }
}

#[derive(Debug)]
enum CauseForForwarding {
    Unconditional = 0x21,           // Unconditional
    CalledPartyBusy = 0x29,         // Called party busy
    NoReply = 0x2A,                 // No reply
    CalledPartyNotReachable = 0x2B, // Called party not reachable
    NoPageResponse = 0x2C,          // No page response
    RadioCongestion = 0x2D,         // Radio congestion
    ImsiDetached = 0x2E,            // IMSI detached
    NightService = 0x2F,            // Night service
    CallTransfer = 0x31,            // Call transfer
    CallDeflectionAlerting = 0x3A,  // Call deflection, alerting
    CallDeflectionImmediate = 0x3B, // Call deflection, immediate
    ScpInitiated = 0xF5,            // SCP initiated
}

impl CauseForForwarding {
    pub fn description(&self) -> &'static str {
        match self {
            CauseForForwarding::Unconditional => "Unconditional",
            CauseForForwarding::CalledPartyBusy => "Called party busy",
            CauseForForwarding::NoReply => "No reply",
            CauseForForwarding::CalledPartyNotReachable => "Called party not reachable",
            CauseForForwarding::NoPageResponse => "No page response",
            CauseForForwarding::RadioCongestion => "Radio congestion",
            CauseForForwarding::ImsiDetached => "IMSI detached",
            CauseForForwarding::NightService => "Night service",
            CauseForForwarding::CallTransfer => "Call transfer",
            CauseForForwarding::CallDeflectionAlerting => "Call deflection, alerting",
            CauseForForwarding::CallDeflectionImmediate => "Call deflection, immediate",
            CauseForForwarding::ScpInitiated => "SCP initiated",
        }
    }
}

impl std::convert::TryFrom<u8> for CauseForForwarding {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x21 => Ok(CauseForForwarding::Unconditional),
            0x29 => Ok(CauseForForwarding::CalledPartyBusy),
            0x2A => Ok(CauseForForwarding::NoReply),
            0x2B => Ok(CauseForForwarding::CalledPartyNotReachable),
            0x2C => Ok(CauseForForwarding::NoPageResponse),
            0x2D => Ok(CauseForForwarding::RadioCongestion),
            0x2E => Ok(CauseForForwarding::ImsiDetached),
            0x2F => Ok(CauseForForwarding::NightService),
            0x31 => Ok(CauseForForwarding::CallTransfer),
            0x3A => Ok(CauseForForwarding::CallDeflectionAlerting),
            0x3B => Ok(CauseForForwarding::CallDeflectionImmediate),
            0xF5 => Ok(CauseForForwarding::ScpInitiated),
            _ => Err(()), // Invalid value
        }
    }
}

#[derive(Debug)]
enum EllBand {
    NotDefined = 0x00,   // Not defined
    GSM = 0x01,          // GSM
    DCS = 0x02,          // DCS
    WCDMA = 0x03,        // WCDMA
    DoesNotExist = 0xFF, // Does not exist
}

impl EllBand {
    pub fn description(&self) -> &'static str {
        match self {
            EllBand::NotDefined => "Not defined",
            EllBand::GSM => "GSM",
            EllBand::DCS => "DCS",
            EllBand::WCDMA => "WCDMA",
            EllBand::DoesNotExist => "Does not exist",
        }
    }
}

impl std::convert::TryFrom<u8> for EllBand {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(EllBand::NotDefined),
            0x01 => Ok(EllBand::GSM),
            0x02 => Ok(EllBand::DCS),
            0x03 => Ok(EllBand::WCDMA),
            0xFF => Ok(EllBand::DoesNotExist),
            _ => Err(()), // Invalid value
        }
    }
}

#[derive(Debug)]
enum CfInformation {
    NotForwarded = 0x00, // Call has not been forwarded
    Forwarded = 0x01,    // Call has been forwarded
}

impl CfInformation {
    pub fn description(&self) -> &'static str {
        match self {
            CfInformation::NotForwarded => "Call has not been forwarded",
            CfInformation::Forwarded => "Call has been forwarded",
        }
    }
}

impl std::convert::TryFrom<u8> for CfInformation {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(CfInformation::NotForwarded),
            0x01 => Ok(CfInformation::Forwarded),
            _ => Err(()), // Invalid value
        }
    }
}

#[derive(Debug)]
enum ChangeDirection {
    Increased = 0x00, // The charge of the call is increased
    Decreased = 0x01, // The charge of the call is decreased
}

impl ChangeDirection {
    pub fn description(&self) -> &'static str {
        match self {
            ChangeDirection::Increased => "The charge of the call is increased",
            ChangeDirection::Decreased => "The charge of the call is decreased",
        }
    }
}

impl std::convert::TryFrom<u8> for ChangeDirection {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(ChangeDirection::Increased),
            0x01 => Ok(ChangeDirection::Decreased),
            _ => Err(()), // Invalid value
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
enum ChangePercent {
    NoChange = 0x00,    // No change in charge
    ValidDecreased(u8), // Valid value if the charge is decreased (01H to 64H)
    ValidIncreased(u8), // Valid value if the charge is increased (01H to FEH)
    Unused = 0xFF,      // Unused value
}

impl ChangePercent {
    pub fn description(&self) -> String {
        match self {
            ChangePercent::NoChange => String::from("No change in charge"),
            ChangePercent::ValidDecreased(percent) => {
                format!("Valid percent (charge decreased): {}", percent)
            }
            ChangePercent::ValidIncreased(percent) => {
                format!("Valid percent (charge increased): {}", percent)
            }
            ChangePercent::Unused => String::from("Unused"),
        }
    }
}

impl std::convert::TryFrom<u8> for ChangePercent {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(ChangePercent::NoChange),
            0xFF => Ok(ChangePercent::Unused),
            0x01..=0x64 => Ok(ChangePercent::ValidDecreased(byte)),
            0x01..=0xFE => Ok(ChangePercent::ValidIncreased(byte)),
            _ => Err(()), // Invalid value
        }
    }
}

#[derive(Debug)]
enum ChargeNature {
    NotFound = 0x00,        // SS did not find the field or CC told SS not to put it there
    Unknown = 0x02,         // Field value unknown to SS (and to DX)
    AniNotAvailable = 0x04, // Automatic Number Identification (ANI) not available or not provided
    AniCallingParty = 0x05, // ANI of the calling party
    AniCalledParty = 0x06,  // ANI of the called party
    OliAndCpnReceived = 0x07, // Originating Line Information (OLI) and CPN received, CN not received
}

impl ChargeNature {
    pub fn description(&self) -> String {
        match self {
            ChargeNature::NotFound => {
                String::from("SS did not find the field or CC told SS not to put it there")
            }
            ChargeNature::Unknown => String::from("Field value unknown to SS (and to DX)"),
            ChargeNature::AniNotAvailable => {
                String::from("Automatic Number Identification (ANI) not available or not provided")
            }
            ChargeNature::AniCallingParty => String::from("ANI of the calling party"),
            ChargeNature::AniCalledParty => String::from("ANI of the called party"),
            ChargeNature::OliAndCpnReceived => {
                String::from("Originating Line Information (OLI) and CPN received, CN not received")
            }
        }
    }
}

impl std::convert::TryFrom<u8> for ChargeNature {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(ChargeNature::NotFound),
            0x02 => Ok(ChargeNature::Unknown),
            0x04 => Ok(ChargeNature::AniNotAvailable),
            0x05 => Ok(ChargeNature::AniCallingParty),
            0x06 => Ok(ChargeNature::AniCalledParty),
            0x07 => Ok(ChargeNature::OliAndCpnReceived),
            _ => Err(()), // Invalid value
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
enum ChargingArea {
    DoesNotExist = 0x00, // Does not exist
    Valid(u8),           // Valid area range from 0x01 to 0x10
    Spare(u8),           // Spare area range from 0x11 to 0xFE
    Unused = 0xFF,       // Unused
}

impl ChargingArea {
    pub fn description(&self) -> String {
        match self {
            ChargingArea::DoesNotExist => String::from("Does not exist"),
            ChargingArea::Valid(_) => String::from("Valid"),
            ChargingArea::Spare(_) => String::from("Spare"),
            ChargingArea::Unused => String::from("Unused"),
        }
    }
}

impl std::convert::TryFrom<u8> for ChargingArea {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ChargingArea::DoesNotExist),
            0xFF => Ok(ChargingArea::Unused),
            0x01..=0x10 => Ok(ChargingArea::Valid(value)),
            0x11..=0xFE => Ok(ChargingArea::Spare(value)),
            _ => Err(()), // Invalid value
        }
    }
}

#[repr(u8)]
enum ChrgType {
    ChargeableCall = 0x00,                               // 00000000b
    FreeFromAnalysis = 0x08,                             // 00001000b
    FreeFromAddressCompleteMessage = 0x10,               // 00010000b
    FreeFromAnswerMessage = 0x20,                        // 00100000b
    FreeFromAnalysisAndACM = 0x18,                       // 00011000b
    FreeFromAnalysisAndAnswerMessage = 0x28,             // 00101000b
    FreeFromCallProgressMessage = 0x40,                  // 01000000b
    FreeFromAnalysisAndCallProgressMessage = 0x48,       // 01001000b
    FreeFromCDB = 0x80,                                  // 10000000b
    FreeFromAnalysisAndCDB = 0x88,                       // 10001000b
    FreeFromACMAndCDB = 0x90,                            // 10010000b
    FreeFromAnalysisAndACMAndCDB = 0x98,                 // 10011000b
    FreeFromAnswerMessageAndCDB = 0xA0,                  // 10100000b
    FreeFromAnalysisAndAnswerMessageAndCDB = 0xA8,       // 10101000b
    FreeFromCallProgressMessageAndCDB = 0xC0,            // 11000000b
    FreeFromAnalysisAndCallProgressMessageAndCDB = 0xC8, // 11001000b
}

impl ChrgType {
    pub fn description(&self) -> String {
        match self {
            ChrgType::ChargeableCall => String::from("Chargeable call"),
            ChrgType::FreeFromAnalysis => String::from("Free of charge from analysis"),
            ChrgType::FreeFromAddressCompleteMessage => {
                String::from("Free of charge from address complete message (ACM)")
            }
            ChrgType::FreeFromAnswerMessage => String::from("Free of charge from answer message"),
            ChrgType::FreeFromAnalysisAndACM => {
                String::from("Free of charge from analysis and ACM")
            }
            ChrgType::FreeFromAnalysisAndAnswerMessage => {
                String::from("Free of charge from analysis and answer message")
            }
            ChrgType::FreeFromCallProgressMessage => {
                String::from("Free of charge from call progress message")
            }
            ChrgType::FreeFromAnalysisAndCallProgressMessage => {
                String::from("Free of charge from analysis and call progress message")
            }
            ChrgType::FreeFromCDB => String::from("Free of charge from CDB"),
            ChrgType::FreeFromAnalysisAndCDB => {
                String::from("Free of charge from analysis and CDB")
            }
            ChrgType::FreeFromACMAndCDB => {
                String::from("Free of charge from address complete message (ACM) and CDB")
            }
            ChrgType::FreeFromAnalysisAndACMAndCDB => {
                String::from("Free of charge from analysis and ACM and CDB")
            }
            ChrgType::FreeFromAnswerMessageAndCDB => {
                String::from("Free of charge from answer message and CDB")
            }
            ChrgType::FreeFromAnalysisAndAnswerMessageAndCDB => {
                String::from("Free of charge from analysis and answer message and CDB")
            }
            ChrgType::FreeFromCallProgressMessageAndCDB => {
                String::from("Free of charge from call progress message and CDB")
            }
            ChrgType::FreeFromAnalysisAndCallProgressMessageAndCDB => {
                String::from("Free of charge from analysis and call progress message and CDB")
            }
        }
    }
}

impl std::convert::TryFrom<u8> for ChrgType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ChrgType::ChargeableCall),
            0x08 => Ok(ChrgType::FreeFromAnalysis),
            0x10 => Ok(ChrgType::FreeFromAddressCompleteMessage),
            0x20 => Ok(ChrgType::FreeFromAnswerMessage),
            0x18 => Ok(ChrgType::FreeFromAnalysisAndACM),
            0x28 => Ok(ChrgType::FreeFromAnalysisAndAnswerMessage),
            0x40 => Ok(ChrgType::FreeFromCallProgressMessage),
            0x48 => Ok(ChrgType::FreeFromAnalysisAndCallProgressMessage),
            0x80 => Ok(ChrgType::FreeFromCDB),
            0x88 => Ok(ChrgType::FreeFromAnalysisAndCDB),
            0x90 => Ok(ChrgType::FreeFromACMAndCDB),
            0x98 => Ok(ChrgType::FreeFromAnalysisAndACMAndCDB),
            0xA0 => Ok(ChrgType::FreeFromAnswerMessageAndCDB),
            0xA8 => Ok(ChrgType::FreeFromAnalysisAndAnswerMessageAndCDB),
            0xC0 => Ok(ChrgType::FreeFromCallProgressMessageAndCDB),
            0xC8 => Ok(ChrgType::FreeFromAnalysisAndCallProgressMessageAndCDB),
            _ => Err(()), // Invalid value
        }
    }
}

#[repr(u8)]
enum DefaultCallHandling {
    NotUsed = 0xFF,               // Not used
    NotUsedInCallHandling = 0x00, // Default call handling is not used
    UsedInCallHandling = 0x01,    // Default call handling is used
}

impl DefaultCallHandling {
    pub fn description(&self) -> String {
        match self {
            DefaultCallHandling::NotUsed => String::from("Not used"),
            DefaultCallHandling::NotUsedInCallHandling => {
                String::from("Default call handling is not used")
            }
            DefaultCallHandling::UsedInCallHandling => {
                String::from("Default call handling is used")
            }
        }
    }
}

impl std::convert::TryFrom<u8> for DefaultCallHandling {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0xFF => Ok(DefaultCallHandling::NotUsed),
            0x00 => Ok(DefaultCallHandling::NotUsedInCallHandling),
            0x01 => Ok(DefaultCallHandling::UsedInCallHandling),
            _ => Err(()), // Invalid value
        }
    }
}

#[repr(u8)]
enum DefaultSmsHandling {
    NotAvailable = 0xFF, // Not available
    NotUsed = 0x00,      // Default SMS handling is not used
    Used = 0x01,         // Default SMS handling is used
}

impl DefaultSmsHandling {
    pub fn description(&self) -> String {
        match self {
            DefaultSmsHandling::NotAvailable => String::from("Not available"),
            DefaultSmsHandling::NotUsed => String::from("Default SMS handling is not used"),
            DefaultSmsHandling::Used => String::from("Default SMS handling is used"),
        }
    }
}

impl std::convert::TryFrom<u8> for DefaultSmsHandling {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0xFF => Ok(DefaultSmsHandling::NotAvailable),
            0x00 => Ok(DefaultSmsHandling::NotUsed),
            0x01 => Ok(DefaultSmsHandling::Used),
            _ => Err(()), // Invalid value
        }
    }
}

#[repr(u8)]
enum DeviceIdentifier {
    Unknown = 0x00,                // Unknown device identifier
    ScfInitiated = 0x01,           // SCF initiated
    OnlineCallMonitoring = 0x02,   // Online call monitoring
    ExternalIp = 0x03,             // External IP
    ParallelRingingGroup = 0x06,   // Parallel Ringing group
    ExternalRingtoneServer = 0x07, // External ringtone server
    NotUsed = 0xFF,                // Device identifier not used
}

impl DeviceIdentifier {
    pub fn description(&self) -> String {
        match self {
            DeviceIdentifier::Unknown => String::from("Unknown device identifier"),
            DeviceIdentifier::ScfInitiated => String::from("SCF initiated"),
            DeviceIdentifier::OnlineCallMonitoring => String::from("Online call monitoring"),
            DeviceIdentifier::ExternalIp => String::from("External IP"),
            DeviceIdentifier::ParallelRingingGroup => String::from("Parallel Ringing group"),
            DeviceIdentifier::ExternalRingtoneServer => String::from("External ringtone server"),
            DeviceIdentifier::NotUsed => String::from("Device identifier not used"),
        }
    }
}

impl std::convert::TryFrom<u8> for DeviceIdentifier {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(DeviceIdentifier::Unknown),
            0x01 => Ok(DeviceIdentifier::ScfInitiated),
            0x02 => Ok(DeviceIdentifier::OnlineCallMonitoring),
            0x03 => Ok(DeviceIdentifier::ExternalIp),
            0x06 => Ok(DeviceIdentifier::ParallelRingingGroup),
            0x07 => Ok(DeviceIdentifier::ExternalRingtoneServer),
            0xFF => Ok(DeviceIdentifier::NotUsed),
            _ => Err(()), // Invalid value
        }
    }
}

#[repr(u8)]
enum DisconnectingParty {
    Unknown = 0x00,      // Release direction is unknown
    IncomingSide = 0x01, // Released from incoming side
    OutgoingSide = 0x02, // Released from outgoing side
    OwnSystem = 0x03,    // Released inside of own system
    MapInitiated = 0x04, // Release initiated from MAP
    ScpInitiated = 0x05, // Release initiated from SCP
}

impl DisconnectingParty {
    pub fn description(&self) -> String {
        match self {
            DisconnectingParty::Unknown => String::from("Release direction is unknown"),
            DisconnectingParty::IncomingSide => String::from("Released from incoming side"),
            DisconnectingParty::OutgoingSide => String::from("Released from outgoing side"),
            DisconnectingParty::OwnSystem => String::from("Released inside of own system"),
            DisconnectingParty::MapInitiated => String::from("Release initiated from MAP"),
            DisconnectingParty::ScpInitiated => String::from("Release initiated from SCP"),
        }
    }
}

impl std::convert::TryFrom<u8> for DisconnectingParty {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(DisconnectingParty::Unknown),
            0x01 => Ok(DisconnectingParty::IncomingSide),
            0x02 => Ok(DisconnectingParty::OutgoingSide),
            0x03 => Ok(DisconnectingParty::OwnSystem),
            0x04 => Ok(DisconnectingParty::MapInitiated),
            0x05 => Ok(DisconnectingParty::ScpInitiated),
            _ => Err(()), // Invalid value
        }
    }
}

#[repr(u8)]
enum DtmfIndicator {
    Off = 0x00, // DTMF is off
    On = 0x01,  // DTMF is on
}

impl DtmfIndicator {
    pub fn description(&self) -> String {
        match self {
            DtmfIndicator::Off => String::from("DTMF is off"),
            DtmfIndicator::On => String::from("DTMF is on"),
        }
    }
}

impl std::convert::TryFrom<u8> for DtmfIndicator {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(DtmfIndicator::Off),
            0x01 => Ok(DtmfIndicator::On),
            _ => Err(()), // Invalid value
        }
    }
}
