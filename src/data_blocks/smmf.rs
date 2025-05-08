// FORMAT TYPE:      23
// MESSAGE NUMBER:   efca
// FORMAT TYPE NAME: SMMF
// RECORD LENGTH:    151
use crate::datatypes::charging_fields::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SMMF {
    // Different from SMMO
    pub called_imsi: String,         // C(8)   25
    pub called_imei: String,         // C(8)   33
    pub called_number: String,       // C(12)  41
    pub called_category: String,     // C(1)   53
    pub called_ms_classmark: String, // C(1)   54
    pub called_subs_lac: String,     // W(1)   55
    pub called_subs_ci: String,      // W(1)   57

    pub sms_centre: String,            // C(10)  59
    pub incoming_time: String,         // C(7)   69
    pub cause_for_termination: String, // DW(1)  76
    pub msc_type: String,              // C(1)   80
    pub calling_number: String,        // C(11)  81

    // Different from SMMO
    pub called_vmsc_number: String, // C(10)  92

    pub calling_number_ton: String,        // C(1)   102
    pub called_number_ton: String,         // C(1)   103
    pub sms_type: String,                  // C(1)   104
    pub call_reference_time: String,       // C(7)   105
    pub hot_billing_record_number: String, // BCD(4) 112
    pub tariff_class: String,              // BCD(3) 116

    // Different from SMMO
    pub calling_vmsc_number: String, // C(10)  119

    pub sms_length: String,                 // C(1)   129
    pub number_of_in_records: String,       // BCD(1) 130
    pub num_of_concatenated_sms: String,    // C(1)   131
    pub concatenated_record_number: String, // C(1)   132
    pub concatenated_sms_reference: String, // W(1)   133
    pub application_info: String,           // C(1)   135
    pub routing_category: String,           // C(1)   136
    pub add_routing_category: String,       // W(1)   137

    // Different from SMMO
    pub radio_network_type: String, // C(1)   139
}

impl SMMF {
    pub fn new(bytes: &[u8]) -> Self {
        let called_imsi = IMSI::new(&bytes[25..33]).value;
        let called_imei = IMEI::new(&bytes[33..41]).value;
        let called_number = NUMBER::new(&bytes[41..53]).value;
        let called_category = Category::new(bytes[53]).value;
        let called_ms_classmark = MSClassMark::new(bytes[54]).value;
        let called_subs_lac = LAC::new(&bytes[55..57]).value;
        let called_subs_ci = CI::new(&bytes[57..59]).value;
        let sms_centre = SMSCentre::new(&bytes[59..69]).value;
        let incoming_time = IncomingTime::new(&bytes[69..76]).value;
        let cause_for_termination = CauseForTermination::new(&bytes[76..80]).value;
        let msc_type = MSCType::new(bytes[80]).value;
        let calling_number = NUMBER::new(&bytes[81..92]).value;
        let called_vmsc_number = VMSCNumber::new(&bytes[92..102]).value;
        let calling_number_ton = TON::new(bytes[102]).value;
        let called_number_ton = TON::new(bytes[103]).value;
        let sms_type = SMSType::new(bytes[104]).value;
        let call_reference_time = CallReferenceTime::new(&bytes[105..112]).value;
        let hot_billing_record_number = HotBilingRecordNumber::new(&bytes[112..116]).value;
        let tariff_class = TariffClass::new(&bytes[116..119]).value;
        let calling_vmsc_number = VMSCNumber::new(&bytes[119..129]).value;
        let sms_length = SMSLenght::new(bytes[129]).value;
        let number_of_in_records = NumberOfInRecords::new(bytes[130]).value;
        let num_of_concatenated_sms = NumOfConcatenatedSMS::new(bytes[131]).value;
        let concatenated_record_number = ConcatenatedRecordNumber::new(bytes[132]).value;
        let concatenated_sms_reference = ConcatenatedSMSReference::new(&bytes[133..135]).value;
        let application_info = ApplicationInfo::new(bytes[135]).value;
        let routing_category = RoutingCategory::new(bytes[136]).value;
        let add_routing_category = AddRoutingCategory::new(&bytes[137..139]).value;
        let radio_network_type = RadioNetworkType::new(bytes[139]).value;

        Self {
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
        }
    }
}
// DATA:
// FIELD NAME                                   DATA TYPE  POSITION

// called_imsi                                     C(  8)        25
// called_imei                                     C(  8)        33
// called_number_ton                               C(  1)        41
// called_number                                   C( 12)        42
// called_category                                 C(  1)        54
// sms_centre                                      C( 10)        55
// incoming_time                                   C(  7)        65
// delivery_time                                   C(  7)        72
// cause_for_termination                          DW(  1)        79
// basic_service_type                              C(  1)        83
// basic_service_code                              C(  1)        84
// msc_type                                        C(  1)        85
// calling_number_ton                              C(  1)        86
// calling_number                                  C( 11)        87
// called_vmsc_number                              C( 10)        98
// sms_type                                        C(  1)       108
// sms_length                                      C(  1)       109
// number_of_in_records                          BCD(  1)       110
// forwarded_to_number_ton                         C(  1)       111
// forwarded_to_number                             C( 12)       112
// forwarded_to_smsc                               C( 12)       124
// num_of_concatenated_sms                         C(  1)       136
// concatenated_record_number                      C(  1)       137
// concatenated_sms_reference                      W(  1)       138
// routing_category                                C(  1)       140
// application_info                                C(  1)       141
// add_routing_category                            W(  1)       142
// call_reference_time                             C(  7)       144
