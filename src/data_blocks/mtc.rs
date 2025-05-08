use crate::datatypes::charging_fields::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MTC {
    pub intermediate_record_number: String,
    pub intermediate_charging_ind: String,
    pub number_of_ss_records: String,
    pub calling_number: String,
    pub called_imsi: String,
    pub called_imei: String,
    pub called_number: String,
    pub called_category: String,
    pub called_ms_classmark: String,
    pub in_circuit_group: String,
    pub in_circuit: String,
    pub called_subs_first_lac: String,
    pub called_subs_first_ci: String,
    pub called_subs_last_ex_id: String,
    pub called_subs_last_lac: String,
    pub called_subs_last_ci: String,
    pub basic_service_type: String,
    pub basic_service_code: String,
    pub facility_usage: String,
    pub non_transparency_indicator: String,
    pub channel_rate_indicator: String,
    pub in_channel_allocated_time: String,
    pub charging_start_time: String,
    pub charging_end_time: String,
    pub cause_for_termination: String,
    pub call_type: String,
    pub term_mcz_chrg_type: String,
    pub term_mcz_duration: String,
    pub term_mcz_tariff_class: String,
    pub term_mcz_pulses: String,
    pub calling_number_ton: String,
    pub called_number_ton: String,
    pub intermediate_chrg_cause: String,
    pub called_modify_parameters: String,
    pub term_mcz_modify_percent: String,
    pub term_mcz_modify_direction: String,
    pub leg_call_reference: String,
    pub out_channel_allocated_time: String,
    pub call_reference_time: String,
    pub speech_version: String,
    pub b_idle_time: String,
    pub pni: String,
    pub term_mcz_change_percent: String,
    pub term_mcz_change_direction: String,
    pub req_fixed_nw_user_rate: String,
    pub req_other_modem_type: String,
    pub acceptable_channel_codings: String,
    pub req_number_of_channels: String,
    pub req_air_interface_user_rate: String,
    pub req_user_initiated_mod_ind: String,
    pub used_number_of_channels: String,
    pub used_other_modem_type: String,
    pub used_fixed_nw_user_rate: String,
    pub used_channel_coding: String,
    pub camel_call_reference: String,
    pub camel_exchange_id_ton: String,
    pub camel_exchange_id: String,
    pub calling_charging_area: String,
    pub called_charging_area: String,
    pub cug_interlock: String,
    pub cug_outgoing_access: String,
    pub cug_information: String,
    pub called_cell_band: String,
    pub routing_category: String,
    pub ms_classmark3: String,
    pub loc_routing_number: String,
    pub ported_in: String,
    pub scp_connection: String,
    pub hot_billing_record_number: String,
    pub number_of_all_in_records: String,
    pub number_of_in_records: String,
    pub loc_routing_number_ton: String,
    pub add_routing_category: String,
    pub in_bnc_connection_type: String,
    pub inside_user_plane_index: String,
    pub inside_control_plane_index: String,
    pub rate_adaption: String,
    pub collect_call_indicator: String,
    pub redirecting_number: String,
    pub called_subs_last_ex_id_ton: String,
    pub radio_network_type: String,
    pub used_air_interface_user_rate: String,
}

impl MTC {
    pub fn new(bytes: &[u8]) -> Self {
        let intermediate_record_number = IntermediateRecordNumber::new(&bytes[25..26]).value; //  BCD(  1)        25
        let intermediate_charging_ind = IntermediateChargingInd::new(bytes[26]).value; //    C(  1)        26
        let number_of_ss_records = NumberOfSSRecords::new(bytes[27]).value; //  BCD(  1)        27
        let calling_number = CallingNumber::new(&bytes[28..38]).value; //    C( 10)        28
        let called_imsi = IMSI::new(&bytes[38..46]).value; //    C(  8)        38
        let called_imei = IMEI::new(&bytes[46..54]).value; //    C(  8)        46
        let called_number = NUMBER::new(&bytes[54..66]).value; //    C( 12)        54
        let called_category = Category::new(bytes[66]).value; //    C(  1)        66
        let called_ms_classmark = MSClassMark::new(bytes[67]).value; //    C(  1)        67
        let in_circuit_group = InCircuitGroup::new(&bytes[68..70]).value; //  BCD(  2)        68
        let in_circuit = InCircuit::new(&bytes[70..72]).value; //  BCD(  2)        70
        let called_subs_first_lac = LAC::new(&bytes[72..74]).value; //    W(  1)        72
        let called_subs_first_ci = CI::new(&bytes[74..76]).value; //    W(  1)        74
        let called_subs_last_ex_id = LastExId::new(&bytes[76..86]).value; //    C( 10)        76
        let called_subs_last_lac = LAC::new(&bytes[86..88]).value; //    W(  1)        86
        let called_subs_last_ci = CI::new(&bytes[88..90]).value; //    W(  1)        88
        let basic_service_type = BasicServiceType::new(bytes[90]).value; //    C(  1)        90
        let basic_service_code = BasicServiceCode::new(bytes[91], &basic_service_type).value; //    C(  1)        91
        let facility_usage = FacilityUsage::new(&bytes[92..96]).value; //    C(  4)        92
        let non_transparency_indicator = NonTrasnparencyIndicator::new(bytes[96]).value; //    C(  1)        96
        let channel_rate_indicator = ChannelRateIndicator::new(bytes[97]).value; //    C(  1)        97
        let in_channel_allocated_time = InChannelAllocatedTime::new(&bytes[98..105]).value; //    C(  7)        98
        let charging_start_time = ChargingStartTime::new(&bytes[105..112]).value; //    C(  7)       105
        let charging_end_time = ChargingEndtime::new(&bytes[112..119]).value; //    C(  7)       112
        let cause_for_termination = CauseForTermination::new(&bytes[119..123]).value; //   DW(  1)       119
        let call_type = CallType::new(bytes[123]).value; //    C(  1)       123
        let term_mcz_chrg_type = ChargeType::new(bytes[124]).value; //    C(  1)       124
        let term_mcz_duration = Duration::new(&bytes[125..128]).value; //  BCD(  3)       125
        let term_mcz_tariff_class = TariffClass::new(&bytes[128..131]).value; //  BCD(  3)       128
        let term_mcz_pulses = Pulses::new(&bytes[131..133]).value; //  BCD(  2)       131
        let calling_number_ton = TON::new(bytes[133]).value; //    C(  1)       133
        let called_number_ton = TON::new(bytes[134]).value; //    C(  1)       134
        let intermediate_chrg_cause = IntermediateChrgCause::new(&bytes[135..137]).value; //    C(  2)       135
        let called_modify_parameters = ModifyParameters::new(&bytes[137..151]).value; //    C( 14)       137
        let term_mcz_modify_percent = ModifyPercent::new(&bytes[151..153]).value; //    W(  1)       151
        let term_mcz_modify_direction = ModifyDirection::new(bytes[153]).value; //    C(  1)       153
                                                                                // let leg_call_reference = LegCallReference::new(&bytes[154..159]).value; //    C(  5)       154
        let leg_call_reference = "not implemented".to_string();
        let out_channel_allocated_time = OutChannelAllocatedTime::new(&bytes[159..166]).value; //    C(  7)       159
        let call_reference_time = CallReferenceTime::new(&bytes[166..173]).value; //    C(  7)       166
        let speech_version = SpeechVersion::new(bytes[173]).value; //    C(  1)       173
        let b_idle_time = BIdleTime::new(&bytes[174..181]).value; //    C(  7)       174
        let pni = PNI::new(&bytes[181..184]).value; //    C(  3)       181
        let term_mcz_change_percent = ChangePercent::new(bytes[184]).value; //    C(  1)       184
        let term_mcz_change_direction = ChangeDirection::new(bytes[185]).value; //    C(  1)       185
        let req_fixed_nw_user_rate = FixedNWUserRate::new(bytes[186]).value; //    C(  1)       186
        let req_other_modem_type = OtherModemType::new(bytes[187]).value; //    C(  1)       187
        let acceptable_channel_codings = AcceptableChannelCodings::new(bytes[188]).value; //    C(  1)       188
        let req_number_of_channels = ReqNumberOfChannels::new(bytes[189]).value; //    C(  1)       189
        let req_air_interface_user_rate = ReqAirInterfaceUserRate::new(bytes[190]).value; //    C(  1)       190
        let req_user_initiated_mod_ind = ReqUserInitiatedModInd::new(bytes[191]).value; //    C(  1)       191
        let used_number_of_channels = UsedNumberOfChannels::new(bytes[192]).value; //    C(  1)       192
        let used_other_modem_type = OtherModemType::new(bytes[193]).value; //    C(  1)       193
        let used_fixed_nw_user_rate = FixedNWUserRate::new(bytes[194]).value; //    C(  1)       194
        let used_channel_coding = UsedChannelCoding::new(bytes[195]).value; //    C(  1)       195
        let camel_call_reference = CamelCallReference::new(&bytes[196..204]).value; //    C(  8)       196
        let camel_exchange_id_ton = TON::new(bytes[204]).value; //    C(  1)       204
        let camel_exchange_id = CamelExchangeId::new(&bytes[205..214]).value; //    C(  9)       205
        let calling_charging_area = ChargingArea::new(&bytes[214..216]).value; //    W(  1)       214
        let called_charging_area = ChargingArea::new(&bytes[216..218]).value; //    W(  1)       216
        let cug_interlock = CugInterlock::new(&bytes[218..222]).value; //    C(  4)       218
        let cug_outgoing_access = CugOutgoingAccess::new(bytes[222]).value; //    C(  1)       222
        let cug_information = CugInformation::new(bytes[223]).value; //    C(  1)       223
        let called_cell_band = CellBand::new(bytes[224]).value; //    C(  1)       224
        let routing_category = RoutingCategory::new(bytes[225]).value; //    C(  1)       225
        let ms_classmark3 = MSClassMark3::new(bytes[226]).value; //    C(  1)       226
        let loc_routing_number = NUMBER::new(&bytes[227..239]).value; //    C( 12)       227
        let ported_in = PortedIn::new(bytes[239]).value; //    C(  1)       239
        let scp_connection = SCPConnection::new(bytes[240]).value; //    C(  1)       240
        let hot_billing_record_number = HotBilingRecordNumber::new(&bytes[241..245]).value; //  BCD(  4)       241
        let number_of_all_in_records = NumberOfAllInRecords::new(bytes[245]).value; //  BCD(  1)       245
        let number_of_in_records = NumberOfInRecords::new(bytes[246]).value; //  BCD(  1)       246
        let loc_routing_number_ton = TON::new(bytes[247]).value; //    C(  1)       247
        let add_routing_category = AddRoutingCategory::new(&bytes[248..250]).value; //    W(  1)       248
        let in_bnc_connection_type = BncConnectionType::new(bytes[250]).value; //    C(  1)       250
        let inside_user_plane_index = UserPlaneIndex::new(&bytes[251..253]).value; //  BCD(  2)       251
        let inside_control_plane_index = ControlPlaneIndex::new(&bytes[253..255]).value; //  BCD(  2)       253
        let rate_adaption = RateAdaption::new(bytes[255]).value; //    C(  1)       255
                                                                 // let collect_call_indicator = CollectCallIndicator::new(bytes[256]).value; //    C(  1)       256
        let collect_call_indicator = "not implemented".to_string();
        let redirecting_number = NUMBER::new(&bytes[257..269]).value; //    C( 12)       257
        let called_subs_last_ex_id_ton = TON::new(bytes[269]).value; //    C(  1)       269
        let radio_network_type = RadioNetworkType::new(bytes[270]).value; //    C(  1)       270
        let used_air_interface_user_rate = UsedAirInterfaceUserRate::new(bytes[271]).value; //    C(  1)       271
        Self {
            intermediate_record_number,
            intermediate_charging_ind,
            number_of_ss_records,
            calling_number,
            called_imsi,
            called_imei,
            called_number,
            called_category,
            called_ms_classmark,
            in_circuit_group,
            in_circuit,
            called_subs_first_lac,
            called_subs_first_ci,
            called_subs_last_ex_id,
            called_subs_last_lac,
            called_subs_last_ci,
            basic_service_type,
            basic_service_code,
            facility_usage,
            non_transparency_indicator,
            channel_rate_indicator,
            in_channel_allocated_time,
            charging_start_time,
            charging_end_time,
            cause_for_termination,
            call_type,
            term_mcz_chrg_type,
            term_mcz_duration,
            term_mcz_tariff_class,
            term_mcz_pulses,
            calling_number_ton,
            called_number_ton,
            intermediate_chrg_cause,
            called_modify_parameters,
            term_mcz_modify_percent,
            term_mcz_modify_direction,
            leg_call_reference,
            out_channel_allocated_time,
            call_reference_time,
            speech_version,
            b_idle_time,
            pni,
            term_mcz_change_percent,
            term_mcz_change_direction,
            req_fixed_nw_user_rate,
            req_other_modem_type,
            acceptable_channel_codings,
            req_number_of_channels,
            req_air_interface_user_rate,
            req_user_initiated_mod_ind,
            used_number_of_channels,
            used_other_modem_type,
            used_fixed_nw_user_rate,
            used_channel_coding,
            camel_call_reference,
            camel_exchange_id_ton,
            camel_exchange_id,
            calling_charging_area,
            called_charging_area,
            cug_interlock,
            cug_outgoing_access,
            cug_information,
            called_cell_band,
            routing_category,
            ms_classmark3,
            loc_routing_number,
            ported_in,
            scp_connection,
            hot_billing_record_number,
            number_of_all_in_records,
            number_of_in_records,
            loc_routing_number_ton,
            add_routing_category,
            in_bnc_connection_type,
            inside_user_plane_index,
            inside_control_plane_index,
            rate_adaption,
            collect_call_indicator,
            redirecting_number,
            called_subs_last_ex_id_ton,
            radio_network_type,
            used_air_interface_user_rate,
        }
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}
// intermediate_record_number                    BCD(  1)        25
// intermediate_charging_ind                       C(  1)        26
// number_of_ss_records                          BCD(  1)        27
// calling_number                                  C( 10)        28
// called_imsi                                     C(  8)        38
// called_imei                                     C(  8)        46
// called_number                                   C( 12)        54
// called_category                                 C(  1)        66
// called_ms_classmark                             C(  1)        67
// in_circuit_group                              BCD(  2)        68
// in_circuit                                    BCD(  2)        70
// called_subs_first_lac                           W(  1)        72
// called_subs_first_ci                            W(  1)        74
// called_subs_last_ex_id                          C( 10)        76
// called_subs_last_lac                            W(  1)        86
// called_subs_last_ci                             W(  1)        88
// basic_service_type                              C(  1)        90
// basic_service_code                              C(  1)        91
// facility_usage                                  C(  4)        92
// non_transparency_indicator                      C(  1)        96
// channel_rate_indicator                          C(  1)        97
// in_channel_allocated_time                       C(  7)        98
// charging_start_time                             C(  7)       105
// charging_end_time                               C(  7)       112
// cause_for_termination                          DW(  1)       119
// call_type                                       C(  1)       123
// term_mcz_chrg_type                              C(  1)       124
// term_mcz_duration                             BCD(  3)       125
// term_mcz_tariff_class                         BCD(  3)       128
// term_mcz_pulses                               BCD(  2)       131
// calling_number_ton                              C(  1)       133
// called_number_ton                               C(  1)       134
// intermediate_chrg_cause                         C(  2)       135
// called_modify_parameters                        C( 14)       137
// term_mcz_modify_percent                         W(  1)       151
// term_mcz_modify_direction                       C(  1)       153
// leg_call_reference                              C(  5)       154
// out_channel_allocated_time                      C(  7)       159
// call_reference_time                             C(  7)       166
// speech_version                                  C(  1)       173
// b_idle_time                                     C(  7)       174
// pni                                             C(  3)       181
// term_mcz_change_percent                         C(  1)       184
// term_mcz_change_direction                       C(  1)       185
// req_fixed_nw_user_rate                          C(  1)       186
// req_other_modem_type                            C(  1)       187
// acceptable_channel_codings                      C(  1)       188
// req_number_of_channels                          C(  1)       189
// req_air_interface_user_rate                     C(  1)       190
// req_user_initiated_mod_ind                      C(  1)       191
// used_number_of_channels                         C(  1)       192
// used_other_modem_type                           C(  1)       193
// used_fixed_nw_user_rate                         C(  1)       194
// used_channel_coding                             C(  1)       195
// camel_call_reference                            C(  8)       196
// camel_exchange_id_ton                           C(  1)       204
// camel_exchange_id                               C(  9)       205
// calling_charging_area                           W(  1)       214
// called_charging_area                            W(  1)       216
// cug_interlock                                   C(  4)       218
// cug_outgoing_access                             C(  1)       222
// cug_information                                 C(  1)       223
// called_cell_band                                C(  1)       224
// routing_category                                C(  1)       225
// ms_classmark3                                   C(  1)       226
// loc_routing_number                              C( 12)       227
// ported_in                                       C(  1)       239
// scp_connection                                  C(  1)       240
// hot_billing_record_number                     BCD(  4)       241
// number_of_all_in_records                      BCD(  1)       245
// number_of_in_records                          BCD(  1)       246
// loc_routing_number_ton                          C(  1)       247
// add_routing_category                            W(  1)       248
// in_bnc_connection_type                          C(  1)       250
// inside_user_plane_index                       BCD(  2)       251
// inside_control_plane_index                    BCD(  2)       253
// rate_adaption                                   C(  1)       255
// collect_call_indicator                          C(  1)       256
// redirecting_number                              C( 12)       257
// called_subs_last_ex_id_ton                      C(  1)       269
// radio_network_type                              C(  1)       270
// used_air_interface_user_rate                    C(  1)       271
