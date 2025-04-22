use crate::datatypes::charging_fields::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SMMO {
    pub calling_imsi: String,               //    C(  8)        25
    pub calling_imei: String,               //    C(  8)        33
    pub calling_number: String,             //    C( 10)        41
    pub calling_category: String,           //    C(  1)        51
    pub calling_ms_classmark: String,       //    C(  1)        52
    pub sms_centre: String,                 //    C( 10)        53
    pub calling_subs_lac: String,           //    W(  1)        63
    pub calling_subs_ci: String,            //    W(  1)        65
    pub incoming_time: String,              //    C(  7)        67
    pub cause_for_termination: String,      //   DW(  1)        74
    pub msc_type: String,                   //    C(  1)        78
    pub called_number: String,              //    C( 12)        79
    pub calling_number_ton: String,         //    C(  1)        91
    pub called_number_ton: String,          //    C(  1)        92
    pub calling_vmsc_number: String,        //    C( 10)        93
    pub sms_type: String,                   //    C(  1)       103
    pub dialled_digits: String,             //    C( 12)       104
    pub call_reference_time: String,        //    C(  7)       116
    pub tariff_class: String,               //  BCD(  3)       123
    pub pni: String,                        //    C(  3)       126
    pub hot_billing_record_number: String,  //  BCD(  4)       129
    pub sms_length: String,                 //    C(  1)       133
    pub command_type: String,               //    C(  1)       134
    pub message_reference: String,          //    C(  1)       135
    pub number_of_in_records: String,       //  BCD(  1)       136
    pub num_of_concatenated_sms: String,    //    C(  1)       137
    pub concatenated_record_number: String, //    C(  1)       138
    pub concatenated_sms_reference: String, //    W(  1)       139
    pub routing_category: String,           //    C(  1)       141
    pub application_info: String,           //    C(  1)       142
    pub dialled_digits_ton: String,         //    C(  1)       143
    pub default_sms_handling: String,       //    C(  1)       144
    pub camel_sms_modification: String,     //    C(  2)       145
    pub add_routing_category: String,       //    W(  1)       147
    pub radio_network_type: String,         //    C(  1)       149
}
impl SMMO {
    pub fn new(bytes: &[u8]) -> Self {
        let calling_imsi = IMSI::new(&bytes[25..33]).value;
        let calling_imei = IMEI::new(&bytes[33..41]).value;
        let calling_number = Number::new(&bytes[41..51]).value;
        let calling_category = Category::new(bytes[51]).value;
        let calling_ms_classmark = MSClassMark::new(bytes[52]).value;
        let sms_centre = SMSCentre::new(&bytes[53..63]).value;
        let calling_subs_lac = LAC::new(&bytes[63..65]).value;
        let calling_subs_ci = CI::new(&bytes[65..67]).value;
        let incoming_time = IncomingTime::new(&bytes[67..74]).value;
        let cause_for_termination = CauseForTermination::new(&bytes[74..78]).value;
        let msc_type = MSCType::new(bytes[78]).value;
        let called_number = Number::new(&bytes[79..91]).value;
        let calling_number_ton = TON::new(bytes[91]).value;
        let called_number_ton = TON::new(bytes[92]).value;
        let calling_vmsc_number = VMSCNumber::new(&bytes[93..103]).value;
        let sms_type = SMSType::new(bytes[103]).value;
        let dialled_digits = DialledDigits::new(&bytes[104..116]).value;
        let call_reference_time = CallReferenceTime::new(&bytes[116..123]).value;
        let tariff_class = TariffClass::new(&bytes[123..126]).value;
        let pni = PNI::new(&bytes[126..129]).value;
        let hot_billing_record_number = HotBilingRecordNumber::new(&bytes[129..133]).value;
        let sms_length = SMSLenght::new(bytes[133]).value;
        let command_type = CommandType::new(bytes[134]).value;
        let message_reference = MessageReference::new(bytes[135]).value;
        let number_of_in_records = NumberOfInRecords::new(bytes[136]).value;
        let num_of_concatenated_sms = NumOfConcatenatedSMS::new(bytes[137]).value;
        let concatenated_record_number = ConcatenatedRecordNumber::new(bytes[138]).value;
        let concatenated_sms_reference = ConcatenatedSMSReference::new(&bytes[139..141]).value;
        let routing_category = RoutingCategory::new(bytes[141]).value;
        let application_info = ApplicationInfo::new(bytes[142]).value;
        let dialled_digits_ton = TON::new(bytes[143]).value;
        let default_sms_handling = DefaultSmsHandling::new(bytes[144]).value;
        let camel_sms_modification = CamelSMSModification::new(&bytes[145..147]).value;
        let add_routing_category = AddRoutingCategory::new(&bytes[147..149]).value;
        let radio_network_type = RadioNetworkType::new(bytes[149]).value;

        Self {
            calling_imsi,
            calling_imei,
            calling_number,
            calling_category,
            calling_ms_classmark,
            sms_centre,
            calling_subs_lac,
            calling_subs_ci,
            incoming_time,
            cause_for_termination,
            msc_type,
            called_number,
            calling_number_ton,
            called_number_ton,
            calling_vmsc_number,
            sms_type,
            dialled_digits,
            call_reference_time,
            tariff_class,
            pni,
            hot_billing_record_number,
            sms_length,
            command_type,
            message_reference,
            number_of_in_records,
            num_of_concatenated_sms,
            concatenated_record_number,
            concatenated_sms_reference,
            routing_category,
            application_info,
            dialled_digits_ton,
            default_sms_handling,
            camel_sms_modification,
            add_routing_category,
            radio_network_type,
        }
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

// FORMAT TYPE:      8
// MESSAGE NUMBER:   efc9
// FORMAT TYPE NAME: SMMO
// RECORD LENGTH:    150
// Short message service (point-to-point), mobile-originated
// HEADER:
// FIELD NAME                                   DATA TYPE  POSITION

// record_length                                   W(  1)         0
// record_type                                   BCD(  1)         2
// record_number                                 BCD(  4)         3
// record_status                                   C(  1)         7
// check_sum                                       W(  1)         8
// call_reference                                  C(  5)        10
// exchange_id                                     C( 10)        15

// DATA:
// FIELD NAME                                   DATA TYPE  POSITION

//  calling_imsi                                    C(  8)        25
//  calling_imei                                    C(  8)        33
//  calling_number                                  C( 10)        41
//  calling_category                                C(  1)        51
//  calling_ms_classmark                            C(  1)        52
//  sms_centre                                      C( 10)        53
//  calling_subs_lac                                W(  1)        63
//  calling_subs_ci                                 W(  1)        65
//  incoming_time                                   C(  7)        67
//  cause_for_termination                          DW(  1)        74
//  msc_type                                        C(  1)        78
//  called_number                                   C( 12)        79
//  calling_number_ton                              C(  1)        91
//  called_number_ton                               C(  1)        92
//  calling_vmsc_number                             C( 10)        93
//  sms_type                                        C(  1)       103
//  dialled_digits                                  C( 12)       104
//  call_reference_time                             C(  7)       116
//  tariff_class                                  BCD(  3)       123
//  pni                                             C(  3)       126
//  hot_billing_record_number                     BCD(  4)       129
//  sms_length                                      C(  1)       133
//  command_type                                    C(  1)       134
//  message_reference                               C(  1)       135
//  number_of_in_records                          BCD(  1)       136
//  num_of_concatenated_sms                         C(  1)       137
//  concatenated_record_number                      C(  1)       138
//  concatenated_sms_reference                      W(  1)       139
//  routing_category                                C(  1)       141
//  application_info                                C(  1)       142
//  dialled_digits_ton                              C(  1)       143
//  default_sms_handling                            C(  1)       144
//  camel_sms_modification                          C(  2)       145
//  add_routing_category                            W(  1)       147
//  radio_network_type                              C(  1)       149
