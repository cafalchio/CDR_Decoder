use crate::datatypes::primitives::*;

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

// Charging data fields
pub struct IntermediateChargingInd{
    pub value: String,
}
pub struct RecordType{
    pub value: String,
}
pub struct RecordStatus{
    pub value: String,
}
pub struct SelectedCodec{
    pub value: String,
}
pub struct ApplicationInfo{
    pub value: String,
}
pub struct Action{
    pub value: String,
}
pub struct TeleserviceCode{
    pub value: String,
}
pub struct ChargingBlockSize{
    pub value: String,
}
pub struct ChargeType{
    pub value: String,
}
pub struct BearerServiceCode{
    pub value: String,
}
pub struct CugInformation{
    pub value: String,
}
pub struct CommandType{
    value: String,
}
pub struct CugOutgoingAccess{
    pub value: String,
}
pub struct BasicCallStateModel{
    pub value: String,
}
pub struct BasicServiceType{
    pub value: String,
}
pub struct BncConnectionType{
    pub value: String,
}
pub struct CallMedia{
    pub value: String,
}
pub struct CallState{
    pub value: String,
}
pub struct CallType{
    pub value: String,
}
pub struct CallingPSTNCategory{
    pub value: String,
}
pub struct CarrierSelection{
    pub value: String,
}
pub struct Category{
    pub value: String,
}
pub struct CauseForForwarding{
    pub value: String,
}
pub struct EllBand{
    pub value: String,
}
pub struct CfInformation{
    pub value: String,
}
pub struct CallReference {
    // word + word + byte
    pub value: String,
}
pub struct ChangeDirection{
    pub value: String,
}

pub struct ChangePercent{
    pub value: String,
}

pub struct DtmfIndicator{
    pub value: String,
}


pub struct DisconnectingParty{
    pub value: String,
}

pub struct DeviceIdentifier{
    pub value: String,
}

pub struct DefaultSmsHandling{
    pub value: String,
}


pub struct DefaultCallHandling{
    pub value: String,
}

pub struct ChrgType{
    pub value: String,
}

pub struct ChargingArea{
    pub value: String,
}


pub struct ChargeNature{
    pub value: String,
}



impl IntermediateChargingInd {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0 => "Normal",
            1 => "Intermediate",
            2 => "Last Partial",
            0xFF => "NotUsed",
            _ => "",
        };
    Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl RecordType {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0 => "Header",
            1 => "Mobile-originated call",
            2 => "Mobile-terminated call",
            3 => "Forwarded call",
            4 => "Call to a roaming subscriber",
            5 => "Supplementary service",
            6 => "HLR interrogation",
            7 => "Location update",
            8 => "Short message service (point-to-point), mobile-originated",
            9 => "Short message service (point-to-point), mobile-terminated",
            10 => "Trailer",
            11 => "PSTN-originated call",
            12 => "PSTN-terminated call",
            13 => "PBX-originated call",
            14 => "PBX-terminated call",
            15 => "Use of hardware",
            16 => "Intelligent network data 1",
            17 => "Unsuccessful call attempt",
            18 => "Intelligent network data 2",
            19 => "Intelligent network data 3",
            20 => "Device-originated call",
            22 => "Remote charging control",
            23 => "IN-forwarde Sms",
            24 => "Camel-originated call",
            25 => "Camel-terminated call",
            26 => "Intelligent network data 4",
            27 => "Location service",
            28 => "Intelligent network data 5",
            29 => "Unstructured supplementary service data",
            30 => "SIP-originated call",
            31 => "SIP-terminated call",
            32 => "SIP-originating message",
            33 => "SIP-terminating message",
            35 => "SIP CDR for registration",
            _ => "",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl RecordStatus {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0 => "normal ok",
            1 => "synchronising error",
            2 => "different contents",
            _ => "",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
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

impl SelectedCodec {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x00 => "Full rate codec for GSM",
            0x01 => "Half rate codec for GSM",
            0x02 => "Enhanced full rate codec for GSM",
            0x03 => "Narrowband full rate AMR codec for GSM",
            0x04 => "Narrowband half rate AMR codec for GSM",
            0x05 => "Narrowband AMR codec for UMTS with 20 ms Codec Mode",
            0x06 => "Narrowband AMR codec for UMTS with 40 ms Codec Mode",
            0x0E => "Spare",
            0x0F => "Spare",
            0x10 => "64 kbps PCM coding with A-law",
            0x11 => "64 kbps PCM coding with U-law",
            0x12 => "ITU-T specified dual-rate speech codec at 5.3 and 6.3 kbit/s",
            0x13 => "ITU-T dual-rate speech codec at 5.3 and 6.3 kbit/s with G.723.1",
            0x14 => "ITU-T widely used 8 kbit/s codec",
            0x15 => "ITU-T widely used 8 kbit/s codec with G.729A",
            0x16 => "Internet low bit-rate codec",
            0x17 => "Comfort noise",
            0xF0 => "FCH Real-time Transport Protocol",
            0xFD => "FDHClearmode",
            _ => "",
        };
            Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Action {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x00 => "Registration",
            0x01 => "Erasure",
            0x02 => "Activation",
            0x03 => "Deactivation",
            0x04 => "Interrogation",
            0x05 => "Invocation",
            0x06 => "Password registration",
            0x07 => "Phase 1 process unstructured SS data",
            0x08 => "Phase 2 process unstructured SS data request",
            0x09 => "Phase 2 process unstructured SS data notify",
            _ => "",
        };
Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ApplicationInfo {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0 => "NormalShortMessage",
            1 => "PictureMessage",
            0xFF => "NotKnown",
            _ => "",
        };
Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl TeleserviceCode {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x00 => "All teleservices",
            0x10 => "Speech transmission",
            0x11 => "Telephony",
            0x12 => "Emergency calls",
            0x20 => "Short message services",
            0x21 => "Short message MT/PP",
            0x22 => "Short message MO/PP",
            0x30 => "Data MHS",
            0x31 => "Advanced MHS access",
            0x40 => "Videotex access services",
            0x41 => "Videotex access profile 1",
            0x42 => "Videotex access profile 2",
            0x43 => "Videotex access profile 3",
            0x50 => "Teletex service",
            0x51 => "Teletex CS",
            0x52 => "Teletex PS",
            0x60 => "Facsimile",
            0x61 => "Facsimile Group 3 and alter speech",
            0x62 => "Automatic facsimile Group 3",
            0xD1 => "Dual numbering (alternate line service)",
            _ => "",
        };
Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl BearerServiceCode {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x00 => "All bearer services",
            0x10 => "3.1 kHz group",
            0x11 => "3.1 kHz ex PLMN",
            0x12 => "Alternate/speech",
            0x13 => "Speech followed by 3.1 kHz",
            0x20 => "Data c.d.a",
            0x21 => "Data c.d.a 300 b/s",
            0x22 => "Data c.d.a 1200 b/s",
            0x23 => "Data c.d.a 1200-75 b/s",
            0x24 => "Data c.d.a 2400 b/s",
            0x25 => "Data c.d.a 4800 b/s",
            0x26 => "Data c.d.a 9600 b/s",
            0x27 => "Data c.d.a general",
            0x30 => "Data c.d.s",
            0x32 => "Data c.d.s 1200 b/s",
            0x34 => "Data c.d.s 2400 b/s",
            0x35 => "Data c.d.s 4800 b/s",
            0x36 => "Data c.d.s 9600 b/s",
            0x37 => "Data c.d.s general",
            0x40 => "PAD access c.d.a",
            0x41 => "PAD access c.d.a 300 b/s",
            0x42 => "PAD access c.d.a 1200 b/s",
            0x43 => "PAD access c.d.a 1200-75 b/s",
            0x44 => "PAD access c.d.a 2400 b/s",
            0x45 => "PAD access c.d.a 4800 b/s",
            0x46 => "PAD access c.d.a 9600 b/s",
            0x47 => "PAD access c.d.a general",
            0x50 => "Data p.d.s",
            0x54 => "Data p.d.s 2400 b/s",
            0x55 => "Data p.d.s 4800 b/s",
            0x56 => "Data p.d.s 9600 b/s",
            0x57 => "Data p.d.s general",
            0x60 => "Alternate speech/data c.d.a",
            0x70 => "Alternate speech/data c.d.s",
            0x80 => "Speech followed by data c.d.a",
            0x90 => "Speech followed by data c.d.s",
            0xFF => "Service not used",
            _ => "",
        };
Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ChargingBlockSize {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x00 => "2 kB",
            0x01 => "8 kB",
            0x02 => "16 kB",
            0x04 => "32 kB",
            0x08 => "64 kB",
            _ => "",
        };
Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ChargeType {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x00 => "00H chargeable call",
            0x08 => "08H free from analysis",
            0x10 => "10H free from address complete message",
            0x20 => "20H free from answer message",
            0x18 => "18H free from analysis and ACM",
            0x28 => "28H free from analysis and answer message",
            0x40 => "40H free from call progress message",
            0x48 => "48H free from analysis and call progress message",
            0x80 => "80H free from CDB",
            0x88 => "88H free from analysis and CDB",
            0x90 => "90H free from ACM and CDB",
            0x98 => "98H free from analysis, ACM, and CDB",
            0xA0 => "A0H free from answer message and CDB",
            0xA8 => "A8H free from analysis, answer message, and CDB",
            0xC0 => "C0H free from call progress message and CDB",
            0xC8 => "C8H free from analysis, call progress message, and CDB",
            _ => "",
        };
Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CugInformation {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x00 => "00 - Not supported or available",
            0x01 => "01 - Subscribers belong to the same group",
            0x02 => "02 - Subscribers do not belong to the same group",
            0x03 => "03 - Subscribers may belong to the same group; this is not known in the originating side",
            _ => "FF - Invalid or unknown value",
        };
Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CommandType {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x00 => "00 - Enquiry relating to previously submitted short message".to_string(),
            0x01 => {
                "01 - Cancel status report request relating to previously submitted short message"
                    .to_string()
            }
            0x02 => "02 - Delete previously submitted short message".to_string(),
            0x03 => {
                "03 - Enable status report request relating to previously submitted short message"
                    .to_string()
            }
            0x04..=0x1F => format!("{:02X} - Reserved unspecified", value),
            0x20..=0xDF => format!("{:02X} - Not used", value),
            0xE0..=0xFF => format!("{:02X} - Values specific for each SMSC", value),
            _ => "".to_string(),
        };
        Self {
            value,
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CugOutgoingAccess {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x00 => "00 - SS did not find the field from network signal or CC tells SS not to put it there",
            0x02 => "02 - Field value unknown to SS (and to DX)",
            0x04 => "04 - Ordinary call",
            0x05 => "05 - Outgoing access allowed",
            0x06 => "06 - Outgoing access not allowed",
            _ => "FF - Invalid or unknown value",
        };
Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl BasicCallStateModel {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x00 => "00H Type of basic call state model not defined",
            0x01 => "01H Basic call state model for originating side",
            0x02 => "02H Basic call state model for terminating side",
            0x04 => "04H Basic call state model for terminating gateway MSC",
            0x05 => "05H Originating basic call state model for call forwarding",
            0x06 => "06H Originating side for COBI call",
            0x07 => "07H Terminating side for COBI call",
            0x08 => "08H Basic call state model for ICA call",
            0xFF => "FFH Unknown",
            0x03 => "03H Originating SMS state model",
            _ => "",
        };
Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl BasicServiceType {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0 => "Teleservice",
            1 => "Bearer service",
            0xFF => "FFH Not used",
            _ => "",
        };
Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl BncConnectionType {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x00 => "No connection",
            0x01 => "ATM Adaptation Layer 1 (AAL1)",
            0x02 => "ATM Adaptation Layer 2 (AAL2)",
            0x04 => "Internet Protocol (IP)",
            0x05 => "Structured AAL1",
            0x08 => "Time Division Multiplex (TDM)",
            0x10 => "Internet Protocol version 4 (IPv4)",
            0x20 => "Internet Protocol version 6 (IPv6)",
            0x40 => "Not active",
            0x80 => "Not registered",
            0xFF => "Not defined",
            _ => "",
        };
Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CallMedia {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0 => "Doesn't exist",
            1 => "Speech",
            2 => "Multimedia",
            _ => "Unknown",
        };
Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CallState {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x00 => "Setup",
            0x01 => "A seized",
            0x02 => "B seized",
            0x03 => "Signalling phase completed",
            _ => "",
        };
Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CallType {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x00 => "incoming",
            0x01 => "forwarded",
            0x02 => "re-routed",
            0x03 => "outgoing",
            0x04 => "handover",
            0x05 => "ported-out",
            0x06 => "follow-on",
            0x10 => "terminated to the announcement machine",
            0x11 => "ISUP tunneling or SIP tunneling",
            0x20 => "international A-subscriber",
            _ => "",
        };
Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CallingPSTNCategory {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x14 => "TUP 10 14H",
            0x19 => "TUP 12 19H",
            0x00 => "TUP 14 00H",
            0x18 => "TUP 18 18H",
            0x04 => "TUP 19 04H",
            _ => "",
        };
Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CarrierSelection {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x00 => "SS did not find the field from the network signalling",
            0x02 => "Field value unknown to SS",
            0x04 => "No indication",
            0x05 => "Selected carrier identification presubscribed and not input by calling party",
            0x06 => "Selected carrier identification presubscribed and input by calling party",
            0x07 => {
                "Selected carrier identification presubscribed, input by calling party undetermined"
            }
            0x08 => "Selected carrier identification not presubscribed and input by calling party",
            _ => "",
        };
Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Category {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x00 => "Ordinary",
            0x02 => "Ordinary, no charge",
            0x05 => "Pay phone",
            0x08 => "Priority",
            0x0B => "Priority, no charge",
            0x10 => "Test equipment",
            0x45 => "Private number service (option)",
            0xF0 => "Not exist",
            0xFF => "Unknown",
            _ => "",
        };
Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CauseForForwarding {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x21 => "Unconditional",
            0x29 => "Called party busy",
            0x2A => "No reply",
            0x2B => "Called party not reachable",
            0x2C => "No page response",
            0x2D => "Radio congestion",
            0x2E => "IMSI detached",
            0x2F => "Night service",
            0x31 => "Call transfer",
            0x3A => "Call deflection, alerting",
            0x3B => "Call deflection, immediate",
            0xF5 => "SCP initiated",
            _ => "",
        };
Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl EllBand {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x00 => "Not defined",
            0x01 => "GSM",
            0x02 => "DCS",
            0x03 => "WCDMA",
            0xFF => "Does not exist",
            _ => "Unknown",
        };
Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CfInformation {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0 => "Call has not been forwarded",
            1 => "Call has been forwarded",
            _ => "",
        };
Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ChangeDirection {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0 => "The charge of the call is increased",
            1 => "The charge of the call is decreased",
            _ => "",
        };
Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ChangePercent {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x00 => "Normal".to_string(),
            0x01..0x64 => format!("Valid percent (charge decreased): {}", value),
            0xFF => "Unused".to_string(),
            _ => "".to_string(),
        };
        Self {
            value: value,
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

// #[repr(u8)]
// #[derive(Debug)]
// enum ChangePercent {
//     NoChange = 0x00,    // No change in charge
//     ValidDecreased(u8), // Valid value if the charge is decreased (01H to 64H)
//     ValidIncreased(u8), // Valid value if the charge is increased (01H to FEH)
//     Unused = 0xFF,      // Unused value
// }

// impl ChangePercent {
//     pub fn description(&self) -> String {
//         match self {
//             ChangePercent::NoChange => String::from("No change in charge"),
//             ChangePercent::ValidDecreased(percent) => {
//                 format!("Valid percent (charge decreased): {}", percent)
//             }
//             ChangePercent::ValidIncreased(percent) => {
//                 format!("Valid percent (charge increased): {}", percent)
//             }
//             ChangePercent::Unused => String::from("Unused"),
//         }
//     }
// }

// impl std::convert::TryFrom<u8> for ChangePercent {
//     type Error = ();

//     fn try_from(byte: u8) -> Result<Self, Self::Error> {
//         match byte {
//             0x00 => Ok(ChangePercent::NoChange),
//             0xFF => Ok(ChangePercent::Unused),
//             0x01..=0x64 => Ok(ChangePercent::ValidDecreased(byte)),
//             0x01..=0xFE => Ok(ChangePercent::ValidIncreased(byte)),
//             _ => Err(()), // Invalid value
//         }
//     }
// }





impl ChargeNature {
    pub fn new(value: u8) -> Self {
       let value = match value {
        0x00 =>"SS did not find the field or CC told SS not to put it there",
        0x02 =>"Field value unknown to SS (and to DX)",
        0x04 =>"Automatic Number Identification (ANI) not available or not provided",
        0x05 =>"ANI of the calling party",
        0x06 =>"ANI of the called party",
        0x07 =>"Originating Line Information (OLI) and CPN received, CN not received",
            _ => "",
        };
    Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}



impl ChargingArea {
    pub fn new(hword: &[u8]) -> Self {
        let hword_val = HWord::new(&hword).value;
        let value = match hword_val {
            0x0000 => "Does not exist",
            0x0001..=0x2710 => "Valid",
            0x2711..=0xFFFE => "Spare",
            0xFFFF => "Unused",
            _ => "",
        };
    Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}




impl ChrgType {
    pub fn new(value: u8) -> Self {
       let value = match value {
        0x00 => "Chargeable call",
        0x08 => "Free of charge from analysis",
        0x10 => "Free of charge from address complete message (ACM)",
        0x20 => "Free of charge from answer message",
        0x18 => "Free of charge from analysis and ACM",
        0x28 => "Free of charge from analysis and answer message",
        0x40 => "Free of charge from call progress message",
        0x48 => "Free of charge from analysis and call progress message",
        0x80 => "Free of charge from CDB",
        0x88 => "Free of charge from analysis and CDB",
        0x90 => "Free of charge from address complete message (ACM) and CDB",
        0x98 => "Free of charge from analysis and ACM and CDB",
        0xA0 => "Free of charge from answer message and CDB",
        0xA8 => "Free of charge from analysis and answer message and CDB",
        0xC0 => "Free of charge from call progress message and CDB",
        0xC8 => "Free of charge from analysis and call progress message and CDB",
        _ => "",
        };
    Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}




impl DefaultCallHandling {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0 => "Default call handling is not used",
            1 => "Default call handling is used",
            0xFF => "Not used",
            _ => "",
        };
    Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}




impl DefaultSmsHandling {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0 => "Default SMS handling is not used",
            1 => "Default SMS handling is used",
            0xFF => "Not available",
            _ => "",
        };
    Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}


impl DeviceIdentifier {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0x00 => "Unknown device identifier", 
            0x01 => "SCF initiated",
            0x02 => "Online call monitoring",
            0x03 => "External IP",
            0x06 => "Parallel Ringing group",
            0x07 => "External ringtone server",
            0xFF => "Device identifier not used",
            _ => ""
        };
    Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}


impl DisconnectingParty {
    pub fn new(value: u8) -> Self {
       let value = match value {
        0x00 => "Release direction is unknown",
        0x01 => "Released from incoming side",
        0x02 => "Released from outgoing side",
        0x03 => "Released inside of own system",
        0x04 => "Release initiated from MAP",
        0x05 => "Release initiated from SCP",
         _ => ""
        };
    Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}


impl DtmfIndicator {
    pub fn new(value: u8) -> Self {
       let value = match value {
            0 => "DTMF is off",
            1 => "DTMF is on",
            _ => "",
        };
    Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

