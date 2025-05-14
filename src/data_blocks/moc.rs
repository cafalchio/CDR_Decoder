#![allow(dead_code)]
#![allow(unused_variables)]

use crate::datatypes::charging_fields::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Moc {
    intermediate_record_number: String,
    intermediate_charging_ind: String,
    number_of_ss_records: String,
    calling_imsi: String,
    calling_imei: String,
    calling_number: String,
    calling_category: String,
    calling_ms_classmark: String,
    called_imsi: String,
    called_imei: String,
    called_number_ton: String,
    called_number: String,
    called_ms_classmark: String,
    dialled_digits_ton: String,
    dialled_digits: String,
    calling_subs_first_lac: String,
    calling_subs_first_ci: String,
    calling_subs_last_ex_id: String,
    calling_subs_last_lac: String,
    calling_subs_last_ci: String,
    out_circuit_group: String,
    out_circuit: String,
    basic_service_type: String,
    basic_service_code: String,
    facility_usage: String,
    non_transparency_indicator: String,
    channel_rate_indicator: String,
    in_channel_allocated_time: String,
    charging_start_time: String,
    charging_end_time: String,
    cause_for_termination: String,
    call_type: String,
    orig_mcz_chrg_type: String,
    orig_mcz_duration: String,
    orig_mcz_tariff_class: String,
    orig_mcz_pulses: String,
    called_msrn_ton: String,
    called_msrn: String,
    calling_number_ton: String,
    intermediate_chrg_cause: String,
    calling_modify_parameters: String,
    orig_mcz_modify_percent: String,
    orig_mcz_modify_direction: String,
    orig_dialling_class: String,
    leg_call_reference: String,
    call_reference_time: String,
    speech_version: String,
    b_idle_time: String,
    pni: String,
    redirected_indicator: String,
    orig_mcz_change_percent: String,
    orig_mcz_change_direction: String,
    calling_charging_area: String,
    called_charging_area: String,
    connected_to_number_ton: String,
    connected_to_number: String,
    cug_interlock: String,
    cug_outgoing_access: String,
    cug_information: String,
    hot_billing_record_number: String,
    number_of_in_records: String,
    regional_subs_indicator: String,
    regional_subs_location_type: String,
    tns_carrier_code: String,
    carrier_selection: String,
    pic: String,
    routing_category: String,
    called_category: String,
    calling_cell_band: String,
    req_fixed_nw_user_rate: String,
    req_other_modem_type: String,
    acceptable_channel_codings: String,
    req_number_of_channels: String,
    req_air_interface_user_rate: String,
    req_user_initiated_mod_ind: String,
    used_number_of_channels: String,
    used_other_modem_type: String,
    used_fixed_nw_user_rate: String,
    used_channel_coding: String,
    camel_call_reference: String,
    camel_exchange_id_ton: String,
    camel_exchange_id: String,
    npdb_query_status: String,
    scp_connection: String,
    number_of_all_in_records: String,
    loc_routing_number: String,
    loc_routing_number_ton: String,
    add_routing_category: String,
    out_bnc_connection_type: String,
    outside_user_plane_index: String,
    outside_control_plane_index: String,
    emergency_call_category: String,
    rate_adaption: String,
    ms_classmark3: String,
    collect_call_indicator: String,
    calling_subs_last_ex_id_ton: String,
    called_subs_last_ex_id_ton: String,
    radio_network_type: String,
    used_air_interface_user_rate: String,
}

impl Moc {
    pub fn new(bytes: &[u8]) -> Self {
        let intermediate_record_number = IntermediateRecordNumber::new(&bytes[25..26]).value; //BCD(  1)        25
        let intermediate_charging_ind: String = IntermediateChargingInd::new(bytes[26]).value; //  C(  1)        26 IntermediateChargingInd
        let number_of_ss_records: String = NumberOfSSRecords::new(bytes[27]).value; //BCD(  1)        27
        let calling_imsi: String = IMSI::new(&bytes[28..36]).value; //  C(  8)        28
        let calling_imei: String = IMSI::new(&bytes[36..44]).value; //  C(  8)        36
        let calling_number: String = CallingNumber::new(&bytes[44..54]).value; //  C( 10)        44
        let calling_category: String = Category::new(bytes[54]).value; //  C(  1)        54
        let calling_ms_classmark: String = MSClassMark::new(bytes[55]).value; //  C(  1)        55
        let called_imsi: String = IMSI::new(&bytes[56..64]).value; //  C(  8)        56
        let called_imei: String = IMEI::new(&bytes[64..72]).value; //  C(  8)        64
        let called_number_ton: String = TON::new(bytes[72]).value; //  C(  1)        72
        let called_number: String = NUMBER::new(&bytes[73..85]).value; //  C( 12)        73
        let called_ms_classmark: String = MSClassMark::new(bytes[85]).value; //  C(  1)        85
        let dialled_digits_ton: String = TON::new(bytes[86]).value; //  C(  1)        86
        let dialled_digits: String = DialledDigits::new(&bytes[87..99]).value; //  C( 12)        87
        let calling_subs_first_lac: String = LAC::new(&bytes[99..101]).value; //  W(  1)        99
        let calling_subs_first_ci: String = CI::new(&bytes[101..103]).value; //  W(  1)       101
        let calling_subs_last_ex_id: String = LastExId::new(&bytes[103..113]).value; //  C( 10)       103
        let calling_subs_last_lac: String = LAC::new(&bytes[113..115]).value; //  W(  1)       113
        let calling_subs_last_ci: String = CI::new(&bytes[115..117]).value; //  W(  1)       115
        let out_circuit_group: String = OutCircuitGroup::new(&bytes[117..119]).value; //BCD(  2)       117
        let out_circuit: String = OutCircuit::new(&bytes[119..121]).value; //BCD(  2)       119
        let basic_service_type: String = BasicServiceType::new(bytes[121]).value; //  C(  1)       121
        let basic_service_code: String =
            BasicServiceCode::new(bytes[122], &basic_service_type).value; //  C(  1)       122
        let facility_usage: String = FacilityUsage::new(&bytes[123..127]).value; //  C(  4)       123
        let non_transparency_indicator: String = NonTrasnparencyIndicator::new(bytes[127]).value; //  C(  1)       127
        let channel_rate_indicator: String = ChannelRateIndicator::new(bytes[128]).value; //  C(  1)       128
        let in_channel_allocated_time: String = InChannelAllocatedTime::new(&bytes[129..136]).value; //  C(  7)       129
        let charging_start_time: String = ChargingStartTime::new(&bytes[136..143]).value; //  C(  7)       136
        let charging_end_time: String = ChargingEndtime::new(&bytes[143..150]).value; //  C(  7)       143
        let cause_for_termination: String = CauseForTermination::new(&bytes[150..154]).value; // DW(  1)       150
        let call_type: String = CallType::new(bytes[154]).value; //  C(  1)       154
        let orig_mcz_chrg_type: String = ChargeType::new(bytes[155]).value; //  C(  1)       155
        let orig_mcz_duration: String = Duration::new(&bytes[156..159]).value; //BCD(  3)       156
        let orig_mcz_tariff_class: String = TariffClass::new(&bytes[159..162]).value; //BCD(  3)       159
        let orig_mcz_pulses: String = Pulses::new(&bytes[162..164]).value; //BCD(  2)       162
        let called_msrn_ton: String = TON::new(bytes[164]).value; //  C(  1)       164
        let called_msrn: String = MSRN::new(&bytes[165..177]).value; //  C( 12)       165
        let calling_number_ton: String = TON::new(bytes[177]).value; //  C(  1)       177
        let intermediate_chrg_cause: String = IntermediateChrgCause::new(&bytes[178..180]).value; //  C(  2)       178
        let calling_modify_parameters: String = ModifyParameters::new(&bytes[180..194]).value; //  C( 14)       180
        let orig_mcz_modify_percent: String = ModifyPercent::new(&bytes[192..196]).value; //  W(  1)       194
        let orig_mcz_modify_direction: String = ModifyDirection::new(bytes[196]).value; //  C(  1)       196
        let orig_dialling_class = OrigDiallingClass::new(&bytes[197..199]).value;
        // let leg_call_reference = LegCallReference::new(&bytes[199..204]).value;
        let leg_call_reference = "<not implemented>".to_string();
        let call_reference_time = CallReferenceTime::new(&bytes[204..211]).value;
        let speech_version = SpeechVersion::new(bytes[211]).value;
        let b_idle_time = BIdleTime::new(&bytes[212..219]).value;
        let pni = PNI::new(&bytes[219..222]).value;
        let redirected_indicator = RedirectedIndicator::new(bytes[222]).value;
        let orig_mcz_change_percent = ChangePercent::new(bytes[223]).value;
        let orig_mcz_change_direction = ChangeDirection::new(bytes[224]).value;
        let calling_charging_area = ChargingArea::new(&bytes[225..227]).value;
        let called_charging_area = ChargingArea::new(&bytes[227..229]).value;
        let connected_to_number_ton = TON::new(bytes[229]).value;
        let connected_to_number = NUMBER::new(&bytes[230..242]).value;
        let cug_interlock = CugInterlock::new(&bytes[242..246]).value;
        let cug_outgoing_access = CugOutgoingAccess::new(bytes[246]).value;
        let cug_information = CugInformation::new(bytes[247]).value;
        let hot_billing_record_number = HotBilingRecordNumber::new(&bytes[248..252]).value;
        let number_of_in_records = NumberOfInRecords::new(bytes[252]).value;
        let regional_subs_indicator = RegionalSubsIndicator::new(bytes[253]).value;
        let regional_subs_location_type = RegionalSubsLocationType::new(bytes[254]).value;
        let tns_carrier_code = TNSCarrierCode::new(&bytes[255..257]).value;
        let carrier_selection = CarrierSelection::new(bytes[257]).value;
        let pic = PIC::new(&bytes[258..260]).value;
        let routing_category = RoutingCategory::new(bytes[260]).value;
        let called_category = Category::new(bytes[261]).value;
        let calling_cell_band = CellBand::new(bytes[262]).value;
        let req_fixed_nw_user_rate = FixedNWUserRate::new(bytes[263]).value;
        let req_other_modem_type = OtherModemType::new(bytes[264]).value;
        let acceptable_channel_codings = AcceptableChannelCodings::new(bytes[265]).value;
        let req_number_of_channels = ReqNumberOfChannels::new(bytes[266]).value;
        let req_air_interface_user_rate = ReqAirInterfaceUserRate::new(bytes[267]).value;
        let req_user_initiated_mod_ind = ReqUserInitiatedModInd::new(bytes[268]).value;
        let used_number_of_channels = UsedNumberOfChannels::new(bytes[269]).value;
        let used_other_modem_type = OtherModemType::new(bytes[270]).value;
        let used_fixed_nw_user_rate = FixedNWUserRate::new(bytes[271]).value;
        let used_channel_coding = UsedChannelCoding::new(bytes[272]).value;
        let camel_call_reference = CamelCallReference::new(&bytes[273..281]).value;
        let camel_exchange_id_ton = TON::new(bytes[281]).value;
        let camel_exchange_id = CamelExchangeId::new(&bytes[282..291]).value;
        let npdb_query_status = NPDBQueryStatus::new(bytes[291]).value;
        let scp_connection = SCPConnection::new(bytes[292]).value;
        let number_of_all_in_records = NumberOfAllInRecords::new(bytes[293]).value;
        let loc_routing_number = NUMBER::new(&bytes[294..306]).value;
        let loc_routing_number_ton = TON::new(bytes[306]).value;
        let add_routing_category = AddRoutingCategory::new(&bytes[307..309]).value;
        let out_bnc_connection_type = BncConnectionType::new(bytes[309]).value;
        let outside_user_plane_index = UserPlaneIndex::new(&bytes[310..312]).value;
        let outside_control_plane_index = ControlPlaneIndex::new(&bytes[312..314]).value;
        let emergency_call_category = EmergencyCallCategory::new(bytes[314]).value;
        let rate_adaption = RateAdaption::new(bytes[315]).value;
        let ms_classmark3 = MSClassMark3::new(bytes[316]).value;
        // let collect_call_indicator = CollectCallIndicator::new(&bytes[317..318]).value;
        let collect_call_indicator = "<not implemented>".to_string();
        let calling_subs_last_ex_id_ton = TON::new(bytes[318]).value;
        let called_subs_last_ex_id_ton = TON::new(bytes[319]).value;
        let radio_network_type = RadioNetworkType::new(bytes[320]).value;
        let used_air_interface_user_rate = UsedAirInterfaceUserRate::new(bytes[321]).value;

        Self {
            intermediate_record_number,
            intermediate_charging_ind,
            number_of_ss_records,
            calling_imsi,
            calling_imei,
            calling_number,
            calling_category,
            calling_ms_classmark,
            called_imsi,
            called_imei,
            called_number_ton,
            called_number,
            called_ms_classmark,
            dialled_digits_ton,
            dialled_digits,
            calling_subs_first_lac,
            calling_subs_first_ci,
            calling_subs_last_ex_id,
            calling_subs_last_lac,
            calling_subs_last_ci,
            out_circuit_group,
            out_circuit,
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
            orig_mcz_chrg_type,
            orig_mcz_duration,
            orig_mcz_tariff_class,
            orig_mcz_pulses,
            called_msrn_ton,
            called_msrn,
            calling_number_ton,
            intermediate_chrg_cause,
            calling_modify_parameters,
            orig_mcz_modify_percent,
            orig_mcz_modify_direction,
            orig_dialling_class,
            leg_call_reference,
            call_reference_time,
            speech_version,
            b_idle_time,
            pni,
            redirected_indicator,
            orig_mcz_change_percent,
            orig_mcz_change_direction,
            calling_charging_area,
            called_charging_area,
            connected_to_number_ton,
            connected_to_number,
            cug_interlock,
            cug_outgoing_access,
            cug_information,
            hot_billing_record_number,
            number_of_in_records,
            regional_subs_indicator,
            regional_subs_location_type,
            tns_carrier_code,
            carrier_selection,
            pic,
            routing_category,
            called_category,
            calling_cell_band,
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
            npdb_query_status,
            scp_connection,
            number_of_all_in_records,
            loc_routing_number,
            loc_routing_number_ton,
            add_routing_category,
            out_bnc_connection_type,
            outside_user_plane_index,
            outside_control_plane_index,
            emergency_call_category,
            rate_adaption,
            ms_classmark3,
            collect_call_indicator,
            calling_subs_last_ex_id_ton,
            called_subs_last_ex_id_ton,
            radio_network_type,
            used_air_interface_user_rate,
        }
    }
    pub fn to_json_str(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
    pub fn to_json(&self) -> serde_json::Result<Value> {
        serde_json::to_value(self)
    }
}
