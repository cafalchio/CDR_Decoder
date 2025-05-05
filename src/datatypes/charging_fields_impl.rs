#![allow(dead_code)]
#![allow(unused_variables)]

use crate::datatypes::charging_fields::*;
use crate::datatypes::primitives::*;

fn bcd_to_decimal(bcd: u8) -> u8 {
    ((bcd >> 4) & 0x0F) * 10 + (bcd & 0x0F)
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

pub fn decode_hexs(hex_bytes: &[u8]) -> String {
    let mut decoded = String::new();
    for &byte in hex_bytes.iter() {
        if byte == 0xFF {
            continue; // skip 0xFF
        }
        decoded.push_str(&format!("{}", &byte));
    }
    decoded
}

fn bcd_nibble_to_char(nibble: u8) -> Option<char> {
    match nibble {
        0x0..=0x9 => Some((b'0' + nibble) as char),
        0xA => Some('A'),
        0xB => Some('*'),
        0xC => Some('#'),
        0xD => Some('B'),
        0xE => Some('C'),
        0xF => None, // filler
        _ => None,
    }
}

pub fn decode_number(bytes: &[u8]) -> String {
    let mut result = String::new();
    for &b in bytes {
        let low = b & 0x0F;
        let high = (b >> 4) & 0x0F;

        if let Some(ch) = bcd_nibble_to_char(low) {
            result.push(ch);
        }
        if let Some(ch) = bcd_nibble_to_char(high) {
            result.push(ch);
        }
    }
    result
}

impl AgeOfEstimate {
    pub fn new(bytes: &[u8]) -> Self {
        let mut val = HDWord::new(bytes).value;
        if val > 32767 {
            val = 32767;
        }
        Self {
            value: format!("{} min", val),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl AnswerTime {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: BcdTimestamp::new(bytes).value,
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl AocIndicator {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0 => "no AoC",
            1 => "AoC",
            _ => "ERROR",
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
            _ => "ERROR",
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
        let value = match bcd_to_decimal(value) {
            0 => "Header".to_string(),
            1 => "Mobile-originated call".to_string(),
            2 => "Mobile-terminated call".to_string(),
            3 => "Forwarded call".to_string(),
            4 => "Call to a roaming subscriber".to_string(),
            5 => "Supplementary service".to_string(),
            6 => "HLR interrogation".to_string(),
            7 => "Location update".to_string(),
            8 => "Short message service (point-to-point), mobile-originated".to_string(),
            9 => "Short message service (point-to-point), mobile-terminated".to_string(),
            10 => "Trailer".to_string(),
            11 => "PSTN-originated call".to_string(),
            12 => "PSTN-terminated call".to_string(),
            13 => "PBX-originated call".to_string(),
            14 => "PBX-terminated call".to_string(),
            15 => "Use of hardware".to_string(),
            16 => "Intelligent network data 1".to_string(),
            17 => "Unsuccessful call attempt".to_string(),
            18 => "Intelligent network data 2".to_string(),
            19 => "Intelligent network data 3".to_string(),
            20 => "Device-originated call".to_string(),
            22 => "Remote charging control".to_string(),
            23 => "IN-forwarde Sms".to_string(),
            24 => "Camel-originated call".to_string(),
            25 => "Camel-terminated call".to_string(),
            26 => "Intelligent network data 4".to_string(),
            27 => "Location service".to_string(),
            28 => "Intelligent network data 5".to_string(),
            29 => "Unstructured supplementary service data".to_string(),
            30 => "SIP-originated call".to_string(),
            31 => "SIP-terminated call".to_string(),
            32 => "SIP-originating message".to_string(),
            33 => "SIP-terminating message".to_string(),
            35 => "SIP CDR for registration".to_string(),
            _ => format!("Not found code -> {}, byte {}", bcd_to_decimal(value), {
                value
            }),
        };
        Self { value: value }
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
            _ => "ERROR",
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
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CallReferenceTime {
    pub fn new(bytes: &[u8]) -> CallReferenceTime {
        let val = BcdTimestamp::new(bytes).value;
        Self { value: val }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
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
            _ => "ERROR",
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
            _ => "ERROR",
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
            _ => "ERROR",
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
            _ => "ERROR",
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
            _ => "ERROR",
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
        };
        Self { value: value }
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
            _ => "ERROR",
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
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl BasicServiceCode {
    pub fn new(byte: u8, basic_service_type: &str) -> Self {
        let bearer_code = byte & 0b1111_0000; // bits 7...4
        let description = match basic_service_type {
            "Teleservice" => match byte {
                0x00 => "All teleservices",
                0x10 => "Speech transmission",
                0x11 => "Telephony",
                0x12 => "Emergency calls",
                0x20 => "Short messages services",
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
                0xFF => "Not Used",
                _ => "ERROR",
            }
            .to_string(),

            "Bearer service" => match bearer_code {
                0x00 => format!("Rate: {}", byte & 0b0000_0111),
                0x10 => "3.1 kHz group".to_string(),
                0x11 => "3.1 kHz ex PLMN".to_string(),
                0x12 => "alternate/speech".to_string(),
                0x13 => "speech followed by 3.1 kHz".to_string(),
                0x20 => "Data c.d.a".to_string(),
                0x30 => "Data c.d.s".to_string(),
                0x40 => "PAD access c.d.a".to_string(),
                0x50 => "Data p.d.s".to_string(),
                0x60 => "Alternate speech/data c.d.a".to_string(),
                0x70 => "Alternate speech/ data c.d.s".to_string(),
                0x80 => "Speech followed by data c.d.a".to_string(),
                0x90 => "Speech followed by data c.d.s".to_string(),
                0xF0 => "Service not used".to_string(),
                _ => "ERROR".to_string(),
            },

            _ => "".to_string(),
        };

        Self { value: description }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl BatchSeqNumber {
    pub fn new(bytes: &[u8]) -> Self {
        let val = BCDWord::new(bytes).value;
        Self {
            value: format!("{}", val),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl BlockSeqNumber {
    pub fn new(bytes: &[u8]) -> Self {
        let val = BCDWord::new(bytes).value;
        Self {
            value: format!("{}", val),
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
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl BIdleTime {
    pub fn new(bytes: &[u8]) -> Self {
        let val = BcdTimestamp::new(bytes).value;
        Self { value: val }
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
            _ => "ERROR",
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
            _ => "ERROR",
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
            _ => "ERROR",
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
            _ => "ERROR",
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
            _ => "ERROR",
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
            _ => "ERROR",
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
            _ => "ERROR",
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
            _ => "ERROR",
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
            0x00 => "No change in charge".to_string(),
            0x01..0xFD => format!("{}%", value),
            0xFF => "NotUsed".to_string(),
            _ => "".to_string(),
        };
        Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ChargeNature {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "SS did not find the field or CC told SS not to put it there",
            0x02 => "Field value unknown to SS (and to DX)",
            0x04 => "Automatic Number Identification (ANI) not available or not provided",
            0x05 => "ANI of the calling party",
            0x06 => "ANI of the called party",
            0x07 => "Originating Line Information (OLI) and CPN received, CN not received",
            _ => "ERROR",
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
            _ => "ERROR",
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
            _ => "ERROR",
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
            _ => "ERROR",
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
            _ => "ERROR",
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
            _ => "ERROR",
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
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CallingNumber {
    pub fn new(bytes: &[u8]) -> Self {
        CallingNumber {
            value: decode_number(bytes),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CamelCallReference {
    pub fn new(bytes: &[u8]) -> Self {
        CamelCallReference {
            value: decode_hexs(bytes),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CamelExchangeId {
    pub fn new(bytes: &[u8]) -> Self {
        CamelExchangeId {
            value: decode_bcds(bytes),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CamelModifyParameters {
    pub fn new(bytes: &[u8]) -> Self {
        CamelModifyParameters {
            value: decode_hexs(bytes),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CamelModification {
    pub fn new(bytes: &[u8]) -> Self {
        let mut modifications = vec![];

        let byte0 = bytes[3];
        let byte1 = bytes[2];
        let byte2 = bytes[1];
        // let byte3 = bytes[0]; // unused

        if byte0 & 0b0000_0001 != 0 {
            modifications.push("calling category");
        }
        if byte0 & 0b0000_0010 != 0 {
            modifications.push("original called number");
        }
        if byte0 & 0b0000_0100 != 0 {
            modifications.push("additional Calling line identity");
        }
        if byte0 & 0b0000_1000 != 0 {
            modifications.push("redirecting number");
        }
        if byte0 & 0b0001_0000 != 0 {
            modifications.push("redirection counter");
        }
        if byte0 & 0b0010_0000 != 0 {
            modifications.push("carrier information");
        }
        if byte0 & 0b0100_0000 != 0 {
            modifications.push("originating line information");
        }
        if byte0 & 0b1000_0000 != 0 {
            modifications.push("charge number");
        }

        if byte1 & 0b0000_0001 != 0 {
            modifications.push("forward conference");
        }
        if byte1 & 0b0000_0010 != 0 {
            modifications.push("call diversion");
        }
        if byte1 & 0b0000_0100 != 0 {
            modifications.push("calling party restriction");
        }
        if byte1 & 0b0000_1000 != 0 {
            modifications.push("backward conference");
        }
        if byte1 & 0b0001_0000 != 0 {
            modifications.push("connected number");
        }
        if byte1 & 0b0010_0000 != 0 {
            modifications.push("hold");
        }
        if byte1 & 0b0100_0000 != 0 {
            modifications.push("call wait");
        }
        if byte1 & 0b1000_0000 != 0 {
            modifications.push("explicit call transfer");
        }

        if byte2 & 0b0000_0001 != 0 {
            modifications.push("call completion");
        }
        if byte2 & 0b0000_0010 != 0 {
            modifications.push("CUG interlock code");
        }
        if byte2 & 0b0000_0100 != 0 {
            modifications.push("CUG outgoing access");
        }
        if byte2 & 0b0000_1000 != 0 {
            modifications.push("non CUG-call");
        }
        if byte2 & 0b0001_0000 != 0 {
            modifications.push("destination routing number");
        }

        let value = modifications.join(", ");
        Self { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CamelServiceKey {
    pub fn new(bytes: &[u8]) -> Self {
        let key_value = HDWord::new(bytes).value;
        let value = match key_value {
            0x00..0x7FFFFFFF => format!("{}", key_value),
            _ => "".to_string(),
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CamelSMSModification {
    pub fn new(bytes: &[u8]) -> Self {
        let mut modifications = vec![];

        let byte0 = bytes[1];
        // let byte1 = bytes[0]; Not used
        if byte0 & 0b0000_0001 != 0 {
            modifications.push("calling party number is modified");
        }
        if byte0 & 0b0000_0010 != 0 {
            modifications.push("called party number is modified");
        }
        if byte0 & 0b0000_0100 != 0 {
            modifications.push("SMSC address is modified");
        }
        let value = modifications.join(", ");
        Self { value }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CauseForTermination {
    pub fn new(bytes: &[u8]) -> Self {
        CauseForTermination {
            value: format!("{}", HDWord::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CellBand {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "Not defined",
            0x01 => "GSM",
            0x02 => "DCS",
            0x03 => "WCDMA",
            0xFF => "Does not exist",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CDBIndicator {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "call drop back not used",
            0x01 => "call drop back used",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ChannelRateIndicator {
    pub fn new(byte: u8) -> Self {
        let low = byte & 0x0F;
        let high = (byte >> 4) & 0x0F;

        let requested = match high {
            0x0 => "half rate",
            0x1 => "full rate",
            0x2 => "dual rate half rate preferred",
            0x3 => "dual rate full rate preferred",
            0xF => "not used",
            _ => "unknown",
        };

        let used = match low {
            0x0 => "not exist",
            0x1 => "sdcch",
            0x8 => "full rate",
            0x9 => "half rate",
            0xF => "not used",
            _ => "unknown",
        };

        let value = format!("requested: {}, used: {}", requested, used);

        Self { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ChargingEndtime {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: BcdTimestamp::new(bytes).value,
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ChargingStartTime {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: BcdTimestamp::new(bytes).value,
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ChargingTime {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: BcdTimestamp::new(bytes).value,
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CheckSum {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", HWord::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CI {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", HWord::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl CIPCarrierCode {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_hexs(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ClientExternalId {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_hexs(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ConcatenatedRecordNumber {
    pub fn new(byte: u8) -> Self {
        Self {
            value: format!("{}", HByte::new(byte).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ConcatenatedSMSReference {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", HWord::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ControlPlaneIndex {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BCDWord::new(&bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl DataLengthInBlock {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", HWord::new(&bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl DataVolume {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BCDWord::new(&bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl DeliveryTime {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: BcdTimestamp::new(bytes).value,
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl DialledDigits {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_hexs(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Duration {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{} seconds", decode_bcds(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl DurationBeforeAnswer {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{} seconds", decode_bcds(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl DurationBeforeAnswerTenMs {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{} seconds", decode_bcds(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl DurationTenMs {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{} seconds", decode_bcds(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl EmergencyCallCategory {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0b0000_0001 => "Police",
            0b0000_0010 => "Ambulance",
            0b0000_0100 => "Fire Brigade",
            0b0000_1000 => "Marine Guard",
            0b0001_0000 => "Mountain Rescue",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl EndTime {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: BcdTimestamp::new(bytes).value,
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl EquipmentType {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x01 => "Conference equipment",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl EquipmentId {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_hexs(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ExchangeId {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_bcds(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ExitMSGTime {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_bcds(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ExitMSGTrunkGroup {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_hexs(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl FacilityUsage {
    pub fn new(bytes: &[u8]) -> Self {
        let value: u32 = HDWord::new(&bytes).value;

        let mapping = [
            (1, "aoc - charging"),
            (2, "aoc - charging info at end of call"),
            (3, "aoc - information"),
            (4, "calling line ID presentation"),
            (5, "calling line ID restriction"),
            (6, "call hold"),
            (7, "call wait"),
            (8, "multiparty"),
            (9, "intelligent network"),
            (10, "call transfer"),
            (11, "call transfer recall"),
            (12, "call drop back"),
            (13, "forwarding"),
            (14, "call-forwarding overdrive"),
            // 15 and 16 = spare
            (17, "completion of calls to busy subscribers"),
            (18, "CAMEL"),
            (19, "ported in"),
            (20, "connected line ID presentation"),
            (21, "connected line ID restriction"),
            (22, "UUS1 - origination/release of call"),
            (23, "UUS2 - ringing phase"),
            (24, "UUS3 - during connection"),
            (25, "aoc - during the call"),
            (26, "multicall"),
            (27, "eMLPP"),
            (28, "TTY"),
            // 29 to 32 = spare
        ];

        let value = mapping
            .iter()
            .filter_map(|(bit, desc)| {
                if (value >> (bit - 1)) & 1 == 1 {
                    Some(*desc)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join(", ");

        Self { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn is_used(&self) -> bool {
        !self.value.is_empty()
    }
}

impl FirstRecordNumber {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BCDWord::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl FixedNWUserRate {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x01 => "User rate 9,6 kbit/s",
            0x02 => "User rate 14,4 kbit/s",
            0x03 => "User rate 19,2 kbit/s",
            0x04 => "User rate 28,8 kbit/s",
            0x05 => "User rate 38,4 kbit/s",
            0x06 => "User rate 48,0 kbit/s",
            0x07 => "User rate 56,0 kbit/s",
            0x08 => "User rate 64,0 kbit/s",
            0x09 => "User rate 33,6 kbit/s",
            0x0A => "User rate 32,0 kbit/s",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl FormatVersion {
    /// Creates a new `FormatVersion` from a slice of 6 bytes.
    ///
    /// The expected layout is:
    /// - Bytes 0-1: Customer code in ASCII (HEX word).
    /// - Bytes 2-4: Version info in BCD format (version, edition, repair).
    /// - Byte 5: Must be 0xFF.
    pub fn new(bytes: &[u8]) -> Self {
        // Check for proper length.

        // First 2 bytes: customer code in ASCII.
        let customer = std::str::from_utf8(&bytes[0..2]).unwrap_or("??");

        // Next 3 bytes: version info.
        // Here we assume each byte is a BCD value.
        // The sample uses 03, 01, 00 → interpreted as version=3, edition=1, repair=0.
        let version = bytes[2];
        let edition = bytes[3];
        let repair = bytes[4];

        // The final byte should be 0xFF.
        if bytes[5] != 0xFF {
            panic!("Final byte is not 0xFF");
        }

        // Format the version string. Depending on your requirements you could trim any leading zeros.
        let value = format!("{} {}.{}-{}", customer, version, edition, repair);

        Self { value }
    }

    /// Returns the parsed format version as a string.
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ForwardedToSMSC {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_bcds(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl IntermediateChrgCause {
    pub fn new(bytes: &[u8]) -> Self {
        // Define mapping: (bit position, corresponding description)
        // Note: Only bits 1 through 21 have defined meanings.
        let value = HDWord::new(&bytes).value;
        let mapping = [
            (1, "Value at the end of the call"),
            (
                2,
                "Intermediate charging because time limit has been reached",
            ),
            (
                3,
                "Intermediate charging because pulse limit has been reached",
            ),
            (4, "The change of the used data rate in user plane"),
            (5, "Call re-establishment"),
            (
                6,
                "Chargeable IN user interaction ended and charging has ended",
            ),
            (7, "Handover has changed the channel-related parameters"),
            (
                8,
                "Handover has changed the band of air interface (Not used)",
            ),
            (9, "Tariff change"),
            (
                10,
                "SCP originating the charging change by means of the SCI information",
            ),
            (11, "Inter-MSC handover"),
            (12, "Follow on call"),
            (13, "Changing of localised service identity"),
            (14, "Call drop back"),
            (15, "Inter-PLMN handover"),
            (16, "Inter-system handover"),
            (17, "Disconnect leg A"),
            (18, "Disconnect leg B"),
            (19, "End of Camel user interaction"),
            (20, "Call type of SIP (speech/multimedia)"),
            (21, "codec change"),
        ];

        // Iterate through the mapping,
        // if a bit is set (starting with LSB as bit 1) then include its description.
        let causes = mapping
            .iter()
            .filter_map(|(bit, desc)| {
                if (value >> (bit - 1)) & 1 == 1 {
                    Some(*desc)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join(", ");

        Self { value: causes }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl GlobalCallReference {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!(
                "{}{}",
                decode_bcds(&bytes[0..16]),
                decode_hexs(&bytes[15..21])
            ),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl GMLCAddress {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_bcds(&bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl GMSCAddress {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_bcds(&bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl GPSData {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_hexs(&bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl GPSDataLength {
    pub fn new(byte: u8) -> Self {
        Self {
            value: format!("{}", HByte::new(byte).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl HorizontalAccuracy {
    pub fn new(byte: u8) -> Self {
        Self {
            value: format!("{}", HByte::new(byte).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl HotBilingRecordNumber {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BCDWord::new(&bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ICID {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_hexs(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ICIDLength {
    pub fn new(byte: u8) -> Self {
        Self {
            value: format!("{}", HByte::new(byte).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ICIDOverflow {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "No ICID overflow",
            0x01 => "ICID overflow",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl IMEI {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_bcds(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl IMEISV {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_bcds(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl IMSI {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_bcds(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl InCategoryKey {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_bcds(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl InChannelAllocatedTime {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BcdTimestamp::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl InCircuitGroup {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BCDWord::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl InCircuitGroupName {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_hexs(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl InCircuit {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BCDWord::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl InData {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_hexs(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl InDataSpare {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", HWord::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl InDataLength {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", HWord::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl InDataLength2 {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", HWord::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl InLegId {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "Does not exist",
            0x01 => "Incoming",
            0x02 => "First outgoing",
            0xEF => "Collect call",
            0xF0 => "Both (incoming and outgoing)",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl InServices {
    /// Creates a new `InServices` from a slice of 50 bytes.
    /// Each 5-byte record is interpreted as:
    /// - The first 4 bytes are the service key.
    /// - The 5th byte is the status.
    ///
    /// The final string will be in the format:
    /// "KEY: status, KEY: status, ..."
    pub fn new(bytes: &[u8]) -> Self {
        if bytes.len() != 50 {
            panic!("Expected 50 bytes for IN_SERVICES, got {}", bytes.len());
        }

        let mut pairs = Vec::new();

        for chunk in bytes.chunks_exact(5) {
            // Format the service key: print each byte in uppercase hexadecimal.
            let key = format!(
                "{:02X}{:02X}{:02X}{:02X}",
                chunk[0], chunk[1], chunk[2], chunk[3]
            );
            let status = Self::get_status(chunk[4]);
            let pair = format!("{}: {}", key, status);
            pairs.push(pair);
        }

        // Join all key: status pairs with a comma and space.
        let value = pairs.join(", ");
        Self { value }
    }

    /// Returns the final string value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Converts a status byte into a status string.
    fn get_status(status_byte: u8) -> String {
        match status_byte {
            0x00 => "Service was failed".to_string(),
            0x01 => "Service was successful".to_string(),
            0xFF => "Not available".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}

impl IncomingTime {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BcdTimestamp::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Initiator {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "Action by network (SCP)",
            0x01 => "Action by user (mobile station)",
            0xFF => "Not available",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl InRecordNumber {
    pub fn new(bytes: &u8) -> Self {
        Self {
            value: format!("{}", BCD::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl IntermediateChargingInd {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "normal",
            0x01 => "intermediate (partial)",
            0x02 => "last partial",
            0xFF => "Not used",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl IntermediateRecordNumber {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_bcds(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl JIP {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_hexs(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl LAC {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", HWord::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl LastExId {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_bcds(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl LastRecordNumber {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BCDWord::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl LevelOfCamelService {
    pub fn new(byte: u8) -> Self {
        let mut parts = vec![];

        if byte & 0b0000_0001 != 0 {
            parts.push("Basic camel");
        }
        if byte & 0b0000_0010 != 0 {
            parts.push("On line charging");
        }
        if byte & 0b0000_0100 != 0 {
            parts.push("Call duration control");
        }
        let value = parts.join(", ");
        Self { value }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl LocUpIndicator {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "Location updating",
            0x01 => "GPRS location update",
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

impl LocationEstimate {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_hexs(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl LocationRequestType {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "Concurrent",
            0x01 => "Mobile-terminated, call-unrelated",
            0x02 => "Mobile-originated for location estimate",
            0x03 => "Network-initiated emergency (request)",
            0x04 => "Network-initiated emergency (release)",
            0x05 => "Network-initiated",
            0x06 => "Mobile-terminated for PLMN operator",
            0x07 => "Mobile-originated for assistance data",
            0x08 => "Mobile-originated for deciphering keys",
            0x09 => "Mobile-terminated, call-related",
            0x10 => "Deferred mobile-terminated",
            0xFF => "Not known",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl MCC {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_hexs(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl MessageReference {
    pub fn new(byte: u8) -> Self {
        Self {
            value: format!("{}", HByte::new(byte).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl MessageSize {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", HDWord::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl MNC {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_hexs(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ModifyDirection {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "The charge of call is increased",
            0x01 => "The charge of call is decreased",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ModifyParameters {
    pub fn new(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), 14, "Expected 14 bytes for ModifyParameters");

        let fields = [
            ("e1", HWord::new(&bytes[0..2]).value),
            ("e2", HWord::new(&bytes[2..4]).value),
            ("e3", HWord::new(&bytes[4..6]).value),
            ("e4", HWord::new(&bytes[6..8]).value),
            ("e5", HWord::new(&bytes[8..10]).value),
            ("e6", HWord::new(&bytes[10..12]).value),
            ("e7", HWord::new(&bytes[12..14]).value),
        ];

        let value = fields
            .iter()
            .filter(|(_, val)| *val != 0)
            .map(|(label, val)| format!("{}={:X}", label, val)) // format as uppercase hex
            .collect::<Vec<_>>()
            .join(", ");

        ModifyParameters { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ModifyPercent {
    pub fn new(bytes: &[u8]) -> Self {
        let value: u16 = u16::from_le_bytes([bytes[0], bytes[1]]);
        let value = match value {
            0x0000 => "No change".to_string(),
            0x0001..=0xFFFE => format!("{}%", value),
            0xFFFF => "Unused".to_string(),
        };
        Self { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl MSCType {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "interworking",
            0x01 => "visited",
            0x02 => "gateway",
            0x03 => "transit",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl MSRN {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: decode_hexs(bytes),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl MSClassMark3 {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "not exist",
            0x01 => "single band",
            0x02 => "dual band",
            0x10 => "UMTS",
            0x11 => "UMTS + single band",
            0x12 => "UMTS + dual band",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl MSClassMark {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "class 1, vehicle and portable",
            0x01 => "class 2, portable",
            0x02 => "class 3, handheld",
            0x03 => "class 4, handheld",
            0x04 => "class 5, handheld",
            0x05..=0x06 => "unknown values",
            0x07 => "UMTS",
            0x08..=0xFE => "unknown values",
            0xFF => "",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl NonTrasnparencyIndicator {
    pub fn new(value: u8) -> Self {
        let value = match value {
            00 => "transparent",
            01 => "nontransparent",
            02 => "transparent, no IWF",
            0xFF => "not used",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl NPDBQueryStatus {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "Information is not available.",
            0x01 => "Query is not done.",
            0x02 => "Query is done and number is not ported.",
            0x03 => "Query is done and number is ported.",
            0x04 => "Query is done and failed.",
            0x05 => "Indicator is set to done, but query has not been performed.",
            0x06 => "Query is done, not known to be ported.",
            0x07 => "Query is done, ported out.",
            0x08 => "Query is done, ported between foreign national network.",
            0x09 => "Query is done, unknown.",
            0x10 => "Query is done, subsequent query allowed.",
            0x0B => "Query is done, subsequent query denied.",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl NPI {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "Information not available",
            0x02 => "Field value unknown to SS (and to DX)",
            0x04 => "Unknown network dialling plan",
            0x05 => "ISDN telephony",
            0x06 => "Data",
            0x07 => "Telex",
            0x08 => "National standard",
            0x09 => "Private",
            0x0A => "Network service access point (NSAP)",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl NumOfConcatenatedSMS {
    pub fn new(byte: u8) -> Self {
        Self {
            value: format!("{}", HByte::new(byte).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl NUMBER {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: decode_number(bytes),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl NumberOfForwardings {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "The call has not been forwarded.",
            0x01..=0x05 => "Possible values (number of forwardings)",
            0xFF => "The information is not available.",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl NumberOfAllInRecords {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BCD::new(&bytes[0]).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl NumberOfInRecords {
    pub fn new(byte: u8) -> Self {
        Self {
            value: format!("{}", BCD::new(&byte).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl NumberOfSSRecords {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BCD::new(&bytes[0]).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl NumberOfTransactions {
    pub fn new(byte: u8) -> Self {
        Self {
            value: format!("{}", HByte::new(byte).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl OLI {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "Information is not available.",
            0x02 => "Field value unknown to SS (and to DX)",
            0x04 => "Plain Old Telephone Service (POTS)",
            0x05 => "Multiparty line (more than 2)",
            0x06 => "Automatic Number Identification (ANI) failure (unavailable)",
            0x07 => "Station Level Rating (Hotel/Motel, without room identification)",
            0x08 => "Special operator handling required",
            0x09 => "Automatic Identified Outward Dialling (AIOD) listed DN sent",
            0x0A => "Coin or non-coin on-calls using database access",
            0x0B => "800 service call",
            0x0C => "Coin",
            0x0D => "Prison/inmate service",
            0x0E => "Intercept (blank)",
            0x0F => "Intercept (trouble)",
            0x10 => "Intercept (regular)",
            0x11 => "Telco operator handled call",
            0x12 => "OUTward Wide Area Telecommunications Service (OUTWATS)",
            0x13 => "TRS (unrestricted line)",
            0x14 => "Cellular service (type 1) -Cellular Carrier identified",
            0x15 => "Cellular service (type 2) -Mobile DN identified",
            0x16 => "Cellular service (roaming)",
            0x17 => "TRS (Hotel/Motel)",
            0x18 => "TRS (restricted)",
            0x19 => "Private paystations",
            0x1A => "Access for private virtual network type of services",
            0x1B => "Inter LATA restricted",
            0x1C => "Inter LATA restricted (hotel/motel)",
            0x1D => "Inter LATA restricted (coin)",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl OptimalRoutingIndicator {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "Optimal routing has not happened.",
            0x01 => "Optimal routing has happened.",
            0xFF => "Not used",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl OrigDiallingClass {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", HWord::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl SubId {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_bcds(bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl TON {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "Information is not available.",
            0x01 => "Value is not allowed to use.",
            0x02 => "Field value unknown to SS (and to DX)",
            0x04 => "Unknown number from destination exchange to user",
            0x05 => "International",
            0x06 => "National",
            0x07 => "Network specified",
            0x08 => "Subscriber",
            0x09 => "Abbreviated",
            0x0A => "Dedicated PAD Access",
            0x0B => "Undefined Carrier Access Code",
            0x0C => "Carrier Access Code of national operator",
            0x0D => "Carrier Access Code of operator",
            0x0E => "International number, operator requested",
            0x0F => "National number, operator requested",
            0x10 => "Level 2 regional number",
            0x11 => "Level 1 regional number",
            0x12 => "Level 0 regional number",
            0x13 => "PISN-specific number",
            0x14 => "National network routing number",
            0x15 => "Network-specific network routing number",
            0x16 => "Concatenated network routing number",
            0x17 => "Ported number",
            0x18 => "Ascii",
            0xFE => "Dialled with Escape code (option, only dialled_digits_ton- field)",
            0xFF => "Does not exist",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}
impl SMSCentre {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_bcds(&bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}
impl VMSCNumber {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_bcds(&bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl SMSType {
    pub fn new(value: u8) -> Self {
        let value = match value {
            0x00 => "Mobile originated short message",
            0x01 => "Mobile originated command short message",
            0x02 => "Mobile terminated short message",
            0x03 => "Mobile terminated status report",
            0x05 => "Forwarded short message",
            0x0C => "SIP originated message",
            0x0D => "SIP terminated message",
            0x0E => "SIP forwarded message",
            0x0F => "SIP direct delivery",
            0x47 => "Mobile-originated short message with status report request",
            _ => "not found",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl TariffClass {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_bcds(&bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl PNI {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", decode_bcds(&bytes)),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl SMSLenght {
    pub fn new(byte: u8) -> Self {
        Self {
            value: format!("{}", HByte::new(byte).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl RoutingCategory {
    pub fn new(byte: u8) -> Self {
        Self {
            value: format!("{}", HByte::new(byte).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl AddRoutingCategory {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", HWord::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl RadioNetworkType {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x01 => "GSM",     //2g
            0x02 => "UMTS",    // 3G
            0x03 => "SIP",     // voip
            0x04 => "UMA",     //  (Unlicensed Mobile Access, used for Wi-Fi/GSM handover)
            0x05 => "VTE 4g?", // I suppose this is New radio 5g, but it needs to be confirmed , it could be 4G
            0xFF => "not used",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ProtocolIdentification {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => "Not used",
            0x01 => "Mobile application part",
            0x02 => "CORE INAP - CS1",
            0x03 => "CAMEL application part",
            0x04 => "Completion of calls to the busy subscriber",
            0x05 => "Calling name presentation - TCAP query",
            0x06 => "Number portability - TCAP query",
            0x07 => "The follow on call process",
            0x08 => "Cx application part",
            0x09 => "Direct collect call dialling",
            0x10 => "SSP routing based on IN categories",
            0x11 => "Sequential hunting",
            0x12 => "Selective ring back tone",
            0x13 => "Parallel hunting",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Pulses {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BCDWord::new(&bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}
impl RegionalSubsLocationType {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => {
                "The calling subscriber's location is unknown, since the call
                    originates from a trunk circuit or from a PBX."
            }
            0xFF => {
                "The subscriber is not a regional one or no charging area was
                    found for the originating cell."
            }
            0x01 => "The call was initiated from inside the calling party's home area.",
            0x02 => "The call was initiated from outside the calling party's home area.",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl RegionalSubsIndicator {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => "no home area charging",
            0x01 => "home area charging",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl RedirectedIndicator {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => "Not used",
            0x01 => "Used",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}
impl ReleaseTime {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BcdTimestamp::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}
impl ReqAirInterfaceUserRate {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => "Not applicable; No meaning is associated with this value.",
            0x01 => "User rate 9,6 kbit/s",
            0x02 => "User rate 14,4 kbit/s",
            0x03 => "User rate 19,2 kbit/s",
            0x04 => "User rate 21,6 kbit/s",
            0x05 => "User rate 28,8 kbit/s",
            0x06 => "User rate 38,4 kbit/s",
            0x07 => "User rate 43,2 kbit/s",
            0x08 => "User rate 57,6 kbit/s",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}
impl ReqNumberOfChannels {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => "One traffic channel",
            0x01 => "Two traffic channels",
            0x02 => "Three traffic channels",
            0x03 => "Four traffic channels",
            0x04 => "Five traffic channels",
            0x05 => "Six traffic channels",
            0x06 => "Seven traffic channels",
            0x07 => "Eight traffic channels",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}
impl ReqUserInitiatedModInd {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => "Not required",
            0x01 => "Up to one traffic channel may be requested",
            0x02 => "Up to two traffic channels may be requested",
            0x03 => "Up to three traffic channels may be requested",
            0x04 => "Up to four traffic channels may be requested",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}
impl ResponseTime {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => "Low delay",
            0x01 => "Delay in tolerance",
            0xFF => "Not known",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}
impl ResultIndicator {
    pub fn new(bytes: &[u8]) -> Self {
        let decoded_value = u16::from_le_bytes([bytes[0], bytes[1]]);
        let value = match decoded_value {
            0x0000 => "Low delay",
            0xFFFF => "Not known",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl OtherModemType {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => "Does not exist.",
            0x01 => "V.32bis modem is requested.",
            0x02 => "V.34 modem is requested",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl OutChannelAllocatedTime {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BcdTimestamp::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl OutCircuitGroup {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BCDWord::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl OutCircuitGroupName {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: decode_hexs(bytes),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl OutCircuit {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BCDWord::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ParametersLength {
    pub fn new(byte: u8) -> Self {
        Self {
            value: format!("{}", HByte::new(byte).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl PartyToCharge {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => "Does not exist.",
            0x01 => "Incoming",
            0x02 => "First outgoing",
            0x03..0xFE => "Other outgoing defined by SCP",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl PIC {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: decode_hexs(bytes),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl PortedIn {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => "Not ported",
            0x01 => "Ported",
            0xFF => "Not used",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl RateAdaption {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => "no rate adaption",
            0x01 => "V.110, X.30 rate adaptation",
            0x02 => "ITU-T X.31 flag stuffing",
            0x03 => "V.120 rate adaption",
            0x13 => "H.223 & H.245 (multimedia)",
            0xFF => "Information is not available.",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl RecordLength {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BCD2uword::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl RecordNumber {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BCD2uword::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl SCFAddress {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: decode_bcds(bytes),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl VirtualMSCId {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: decode_hexs(bytes),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl VerticalAccuracy {
    pub fn new(byte: u8) -> Self {
        Self {
            value: format!("{}", HByte::new(byte).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl UserPlaneIndex {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BCDWord::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl UsedNumberOfChannels {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => "None",
            0x01 => "Stand Alone Dedicated Control Channel (SDCCH)",
            0x04 => "Eight full rate traffic channels",
            0x08 => "One full rate traffic channel",
            0x09 => "One half rate traffic channel",
            0x0A => "Two full rate traffic channels",
            0x0B => "Three full rate traffic channels",
            0x0C => "Four full rate traffic channels",
            0x0D => "Five full rate traffic channels",
            0x0E => "Six full rate traffic channels",
            0x0F => "Seven full rate traffic channels",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl UsedChannelCoding {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x01 => "28,8 kbit/s (The radio interface rate is 29,0 kbit/s)",
            0x02 => "32,0 kbit/s (The radio interface rate is 32,0 kbit/s)",
            0x03 => "43,2 kbit/s (The radio interface rate is 43,5 kbit/s)",
            0x0B => "9,6 kbit/s (The radio interface rate is 12,0 kbit/s)",
            0x0C => "4,8 kbit/s (The radio interface rate is 6,0 kbit/s)",
            0x0E => "14,4 kbit/s (The radio interface rate is 14,5 kbit/s)",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl UsedAirInterfaceUserRate {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => "Not used",
            0x01 => "14,4 kbit/s",
            0x02 => "28,8 kbit/s",
            0x03 => "32,0 kbit/s",
            0x04 => "57,6 kbit/s",
            0x05 => "64,0 kbit/s",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl SIPSigMode {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => "Unknown",
            0x01 => "SIP ISUP tunnelling: SIP-I, SIP-T",
            0x02 => "Media gateway control function (MGCF)",
            0x03 => "SIP Access interface of MSS",
            0x04 => "SIP Trunk interface of MSS",
            0x05 => "ISC Interface of NVS for originating services",
            0x06 => "ISC Interface of NVS for terminating services",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ServiceTime {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BcdTimestamp::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ShortenedInServices {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: decode_hexs(bytes),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ServiceIdentifier {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => "Unknown service identifier",
            0x01 => "Service identifier not used",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ServiceCode {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: decode_hexs(bytes),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl SCPConnection {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => "IN service connection failed",
            0x01 => "IN service connection successful",
            0xFF => "Not used",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl SpeechVersion {
    pub fn new(byte: u8) -> Self {
        Self {
            value: format!("{}", HByte::new(byte).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl StartTime {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: format!("{}", BcdTimestamp::new(bytes).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl StreamIdentifier {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => "No bearer",
            0x01 => "Only one ongoing call",
            _ => "Multicall",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl SSRecordNumber {
    pub fn new(byte: u8) -> Self {
        Self {
            value: format!("{}", BCD::new(&byte).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl TariffChangeCNT {
    pub fn new(byte: u8) -> Self {
        Self {
            value: format!("{}", BCD::new(&byte).value),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl TapeBlockType {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: decode_hexs(bytes),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl SupplementaryServicecode {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => "all supplementary services",
            0x10 => "all number identification services",
            0x11 => "calling number identification presentation",
            0x12 => "calling number identification restriction",
            0x13 => "called number identification presentation",
            0x14 => "called number identification restriction",
            0x15 => "malicious call identification",
            0x20 => "call forwarding services",
            0x21 => "call forwarding unconditional",
            0x28 => "all conditional forwarding services",
            0x29 => "call forwarding on mobile subscriber busy",
            0x2A => "call forwarding on no reply",
            0x2B => "call forwarding on mobile subscriber not reachable",
            0x2C => "operator controlled call forwarding (not defined in GSM 09.02)",
            0x2D => "night_service (not defined in GSM 09.02)",
            0x30 => "all call offering services",
            0x31 => "call transfer",
            0x32 => "mobile access hunting",
            0x3A => "Call deflection, alerting",
            0x3B => "Call deflection, immediate",
            0x40 => "all call completion services",
            0x41 => "call waiting",
            0x42 => "call hold",
            0x43 => "completion of calls to busy subscribers",
            0x45 => "multicall",
            0x4A => "completion of calls to not reachable subscribers",
            0x50 => "all multiparty service",
            0x51 => "multiparty service",
            0x60 => "all community of interest services",
            0x61 => "closed user group",
            0x70 => "all charging services",
            0x71 => "AoC-I",
            0x72 => "AoC-C",
            0x73 => "reverse charging",
            0x80 => "all additional information transfer services",
            0x81 => "user to user signalling 1 (origination/release)",
            0x82 => "user to user signalling 2 (ringing)",
            0x83 => "user to user signalling 3 (connection)",
            0x90 => "all call restriction services",
            0x91 => "barring of outgoing calls",
            0x92 => "barring of all outgoing calls",
            0x93 => "barring of all outgoing international calls",
            0x94 => "barring of all outgoing international non-HPLMN directed calls",
            0x95 => "barring of all outgoing calls when outside HPLMN (not defined in 09.02)",
            0x99 => "barring of incoming calls",
            0x9A => "barring of all incoming calls",
            0x9B => "barring of all incoming calls when outside HPLMN",
            0xEE => "AoC-D",
            0xEF => "AoC-E",
            0xF1 => "call transfer recall (not defined in GSM 09.02)",
            0xF2 => "USSD (not defined in GSM 09.02)",
            0xF5 => "IN service (not defined in GSM 09.02)",
            0xFF => "not used",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl SubsRoamingStatus {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x31 => "Home country and network",
            0x32 => "Home country and other network",
            0x42 => "Other country and network",
            0xFF => "Not known",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl SubsNewExId {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: decode_bcds(bytes),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl TNSCarrierCode {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            value: decode_hexs(bytes),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl TNSCircuitCode {
    pub fn new(byte: u8) -> Self {
        let value = match byte {
            0x00 => "Unspecified",
            0x01 => "International call, no operator requested",
            0x02 => "International call, operator requested",
            0x03 => "Network-specific value",
            0x04 => "Network-specific value",
            0x05 => "Network-specific value",
            0x06 => "Network-specific value",
            0x07 => "Network-specific value",
            0x08 => "Network-specific value",
            0x09 => "Network-specific value",
            0x0A => "Network-specific value",
            _ => "ERROR",
        };
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl RountingInfo {
    pub fn new(bytes: &[u8]) -> Self {
        // 2 Hex bytes
        // get most and least significant
        let least: u8 = bytes[0];
        let most: u8 = bytes[1];
        let incoming = most & 0b0000_1111;
        let outgoing = most >> 4 & 0b0000_1111;
        let outgoing_value = match outgoing {
            0x01 => "Own mobile station",
            0x02 => "Visitor mobile station",
            0x03 => "Network (trunk signalling)",
            0x04 => "PBX",
            0x05 => "Voice processing system",
            0x06 => "PDN (PAD network)",
            0x07 => "DOC (Device-originated Call)",
            0x08 => "Announcement",
            0x09 => "IP (Internet Protocol)",
            _ => "ERROR",
        };
        let incoming_value = match incoming {
            0x01 => "Own mobile station",
            0x02 => "Visitor mobile station",
            0x03 => "Network (trunk signalling)",
            0x04 => "PBX",
            0x05 => "Voice processing system",
            0x06 => "PDN (PAD network)",
            0x07 => "DOC (Device-originated Call)",
            0x08 => "Announcement",
            0x09 => "IP (Internet Protocol)",
            _ => "ERROR",
        };

        let roaming = least & 0b0000_0011;
        let roaming_value = match roaming {
            0x01 => "MS (called party) is roaming in own PLMN.",
            0x02 => "MS (called party) is roaming in visitor PLMN.",
            0x03 => "Information from IN charging operation in the GMSC call model",
            _ => "ERROR",
        };
        let charging = (least & 0b0110_0000) >> 5;
        let charging_value = match charging {
            0x01 => "Disabled",
            0x02 => "Additional charging",
            0x03 => "Replace charging",
            _ => "ERROR",
        };
        let mut other_values: Vec<&str> = vec![];

        if least & 0b0001_0000 == 0b0001_0000 {
            other_values.push("Announcement for called ('B') subscriber");
        }
        if least & 0b0000_1000 == 0b0000_1000 {
            other_values.push("Announcement for calling ('A') subscriber");
        }
        if least & 0b0000_0100 == 0b0000_0100 {
            other_values.push("Roaming info (see the values below)");
        }

        let other_bits = other_values.join(", ");

        let value = format!(
            "Incoming: {}, Outgoing: {}, roaming: {}, charging: {}, {}",
            incoming, outgoing, roaming, charging, other_bits
        );
        Self { value: value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl UsedPositionMethod {
    pub fn new(bytes: &[u8]) -> Self {
        // from right to left
        let mut results: Vec<String> = vec![];
        for byte in bytes.iter().rev() {
            if *byte == 0xFF {
                continue;
            }
            let positioning: u8 = byte & 0b1111_1000 >> 3;
            let pos_method: u8 = byte & 0b0000_0111;

            let positioning_value = match positioning {
                0x00 => "Timing advance",
                0x01 => "Time of arrival",
                0x02 => "Angle of arrival",
                0x03 => "Mobile-assisted enhanced observed time difference",
                0x04 => "Mobile-based enhanced observed time difference",
                0x05 => "Mobile-assisted global positioning system",
                0x06 => "Mobile-based global positioning system",
                0x07 => "Conventional global positioning system",
                0x08 => "Mobile-based observed time difference of arrival",
                0x09 => "Mobile-assisted observed time difference of arrival",
                _ => "Error",
            };

            let pos_method_value = match pos_method {
                0x00 => "unsuccessful",
                0x01 => "not used",
                0x02 => "used to verify but not generate location",
                0x03 => "used to generate location",
                0x04 => "undetermined",
                _ => "Error",
            };
            results.push(format!("{} - {}", positioning_value, pos_method_value));
        }
        Self {
            value: results.join(" | "),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl UsedUtranPosMethod {
    pub fn new(bytes: &[u8]) -> Self {
        let mut results: Vec<String> = vec![];
        for byte in bytes.iter().rev() {
            if *byte == 0xFF {
                continue;
            }
            let pos_method: u8 = byte & 0b1111_1000;
            let use_pos_method: u8 = byte & 0b0000_0111;

            let pos_method_value = match pos_method {
                0x05 => "Mobile assisted GPS",
                0x06 => "Mobile based GPS",
                0x07 => "Conventional GPS",
                0x09 => "OTDOA",
                0x0A => "IPDL",
                0x0B => "RTT",
                0x0C => "Cell ID",
                _ => "Error",
            };

            let use_pos_method = match pos_method {
                0x00 => "unsuccessful",
                0x01 => "not used",
                0x02 => "used to verify but not generate location",
                0x03 => "used to generate location",
                0x04 => "undetermined",
                _ => "Error",
            };
            results.push(format!("{} - {}", pos_method_value, use_pos_method));
        }
        Self {
            value: results.join(" | "),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}
