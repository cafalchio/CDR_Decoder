use crate::datatypes::charging_fields::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SMMT {
    pub called_imsi: String,                // NEW (SMMO has calling_imsi instead)
    pub called_imei: String,                // NEW (SMMO has calling_imei instead)
    pub called_number: String, // WIDTH differs: C(12) here vs C(10) in SMMO’s calling_number
    pub called_category: String, // NEW (SMMO has calling_category, but at a different offset)
    pub called_ms_classmark: String, // NEW (SMMO has calling_ms_classmark)
    pub called_subs_lac: String, // NEW (SMMO has calling_subs_lac)
    pub called_subs_ci: String, // NEW (SMMO has calling_subs_ci)
    pub sms_centre: String,    // same name but OFFSET differs (59 vs 53)
    pub incoming_time: String, // same
    pub cause_for_termination: String, // same
    pub msc_type: String,      // same
    pub calling_number: String, // OFFSET differs (81 vs 41)
    pub called_vmsc_number: String, // NEW (SMMO has calling_vmsc_number and called_vmsc_number at different offsets)
    pub calling_number_ton: String, // same name but OFFSET differs (102 vs 91)
    pub called_number_ton: String,  // same name but OFFSET differs (103 vs 92)
    pub sms_type: String,           // same name but OFFSET differs (104 vs 103)
    pub call_reference_time: String, // OFFSET differs (105 vs 116)
    pub hot_billing_record_number: String, // same name but OFFSET differs (112 vs 129)
    pub tariff_class: String,       // same name but OFFSET differs (116 vs 123)
    pub calling_vmsc_number: String, // NEW (SMMO has calling_vmsc_number at 93..103)
    pub sms_length: String,         // same name but OFFSET differs (129 vs 133)
    pub number_of_in_records: String, // same name but OFFSET differs (130 vs 136)
    pub num_of_concatenated_sms: String, // same name but OFFSET differs (131 vs 137)
    pub concatenated_record_number: String, // same name but OFFSET differs (132 vs 138)
    pub concatenated_sms_reference: String, // same name but OFFSET differs (133 vs 139)
    pub application_info: String,   // same name but OFFSET differs (135 vs 142)
    pub routing_category: String,   // same name but OFFSET differs (136 vs 141)
    pub add_routing_category: String, // same name but OFFSET differs (137 vs 147)
    pub radio_network_type: String, // same name but OFFSET differs (139 vs 149)
}
impl SMMT {
    pub fn new(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 139 {
            return Err("SMMT: insufficient data".into());
        }
        let called_imsi = IMSI::new(&bytes[25..33]).value; //    C(  8)        25
        let called_imei = IMEI::new(&bytes[33..41]).value; //    C(  8)        33
        let called_number = NUMBER::new(&bytes[41..53]).value; //    C( 12)        41
        let called_category = Category::new(bytes[53]).value; //    C(  1)        53
        let called_ms_classmark = MSClassMark::new(bytes[54]).value; //    C(  1)        54
        let called_subs_lac = LAC::new(&bytes[55..57]).value; //    W(  1)        55
        let called_subs_ci = CI::new(&bytes[57..59]).value; //    W(  1)        57
        let sms_centre = SMSCentre::new(&bytes[59..69]).value; //    C( 10)        59
        let incoming_time = IncomingTime::new(&bytes[69..76]).value; //    C(  7)        69
        let cause_for_termination = CauseForTermination::new(&bytes[76..80]).value; //   DW(  1)        76
        let msc_type = MSCType::new(bytes[80]).value; //    C(  1)        80
        let calling_number = NUMBER::new(&bytes[81..92]).value; //    C( 11)        81
        let called_vmsc_number = VMSCNumber::new(&bytes[92..102]).value; //    C( 10)        92
        let calling_number_ton = TON::new(bytes[102]).value; //    C(  1)       102
        let called_number_ton = TON::new(bytes[103]).value; //    C(  1)       103
        let sms_type = SMSType::new(bytes[104]).value; //    C(  1)       104
        let call_reference_time = CallReferenceTime::new(&bytes[105..112]).value; //    C(  7)       105
        let hot_billing_record_number = HotBilingRecordNumber::new(&bytes[112..116]).value; //  BCD(  4)       112
        let tariff_class = TariffClass::new(&bytes[116..119]).value; //  BCD(  3)       116
        let calling_vmsc_number = VMSCNumber::new(&bytes[119..129]).value; //    C( 10)       119
        let sms_length = SMSLenght::new(bytes[129]).value; //    C(  1)       129
        let number_of_in_records = NumberOfInRecords::new(bytes[130]).value; //  BCD(  1)       130
        let num_of_concatenated_sms = NumOfConcatenatedSMS::new(bytes[131]).value; //    C(  1)       131
        let concatenated_record_number = ConcatenatedRecordNumber::new(bytes[132]).value; //    C(  1)       132
        let concatenated_sms_reference = ConcatenatedSMSReference::new(&bytes[133..135]).value; //    W(  1)       133
        let application_info = ApplicationInfo::new(bytes[135]).value; //    C(  1)       135
        let routing_category = RoutingCategory::new(bytes[136]).value; //    C(  1)       136
        let add_routing_category = AddRoutingCategory::new(&bytes[137..139]).value; //    W(  1)       137
        let radio_network_type = RadioNetworkType::new(bytes[139]).value; //    C(  1)       139

        Ok(Self {
            called_imsi,
            called_imei,
            called_number,
            called_category,
            called_ms_classmark,
            called_subs_lac,
            called_subs_ci,
            sms_centre,
            incoming_time,
            cause_for_termination,
            msc_type,
            calling_number,
            called_vmsc_number,
            calling_number_ton,
            called_number_ton,
            sms_type,
            call_reference_time,
            hot_billing_record_number,
            tariff_class,
            calling_vmsc_number,
            sms_length,
            number_of_in_records,
            num_of_concatenated_sms,
            concatenated_record_number,
            concatenated_sms_reference,
            application_info,
            routing_category,
            add_routing_category,
            radio_network_type,
        })
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}
