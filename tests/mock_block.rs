use cdr_decoder::datatypes::charging_fields::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct MockBlock {
    pub record_length: String,
    pub record_type: String,
    pub record_number: String,
    pub record_status: String,
    pub check_sum: String,
    pub call_reference: String,
    pub exchange_id: String,
    pub intermediate_record_number: String,
    pub intermediate_charging_ind: String,
    pub number_of_ss_records: String,
    pub calling_imsi: String,
    pub calling_imei: String,
    pub calling_number: String,
    pub calling_category: String,
    pub calling_ms_classmark: String,
    pub called_imsi: String,
    pub called_imei: String,
    pub called_number_ton: String,
    pub called_number: String,
    pub called_category: String,
    pub called_ms_classmark: String,
    pub dialled_digits_ton: String,
    pub dialled_digits: String,
    pub calling_subs_first_lac: String,
    pub calling_subs_first_ci: String,
    pub calling_subs_last_ex_id: String,
    pub calling_subs_last_lac: String,
    pub calling_subs_last_ci: String,
    pub called_subs_first_lac: String,
    pub called_subs_first_ci: String,
    pub called_subs_last_ex_id: String,
    pub called_subs_last_lac: String,
    pub called_subs_last_ci: String,
    pub out_circuit_group: String,
    pub out_circuit: String,
    pub basic_service_type: String,
    pub basic_service_code: String,
    pub facility_usage: String,
    pub non_transparency_indicator: String,
    pub channel_rate_indicator: String,
    pub set_up_start_time: String,
    pub in_channel_allocated_time: String,
    pub b_idle_time: String,
    pub charging_start_time: String,
    pub charging_end_time: String,
    pub cause_for_termination: String,
    pub data_volume: String,
    pub call_type: String,
    pub dtmf_indicator: String,
    pub aoc_indicator: String,
    pub redirected_indicator: String,
    pub cdb_indicator: String,
    pub orig_mcz_chrg_type: String,
    pub orig_mcz_duration: String,
    pub orig_mcz_duration_ten_ms: String,
    pub orig_mcz_tariff_class: String,
    pub orig_mcz_pulses: String,
    pub orig_mcz_change_percent: String,
    pub orig_mcz_change_direction: String,
    pub called_msrn_ton: String,
    pub called_msrn: String,
    pub calling_charging_area: String,
    pub called_charging_area: String,
    pub called_msrn_npi: String,
    pub calling_number_ton: String,
    pub calling_number_npi: String,
    pub called_number_npi: String,
    pub dialled_digits_npi: String,
    pub connected_to_number_ton: String,
    pub connected_to_number_npi: String,
    pub connected_to_number: String,
    pub cug_interlock: String,
    pub cug_outgoing_access: String,
    pub hot_billing_record_number: String,
    pub number_of_in_records: String,
    pub regional_subs_indicator: String,
    pub regional_subs_location_type: String,
    pub leg_call_reference: String,
    pub call_reference_time: String,
    pub char_band_chrg_type: String,
    pub char_band_duration: String,
    pub char_band_duration_ten_ms: String,
    pub char_band_tariff_class: String,
    pub charg_band_pulses: String,
    pub char_band_change_percent: String,
    pub char_band_change_direction: String,
    pub charge_number_ton: String,
    pub charge_number_npi: String,
    pub charge_number: String,
    pub charge_nature: String,
    pub tns_circuit_code: String,
    pub tns_carrier_code: String,
    pub cip_carrier_code: String,
    pub carrier_selection: String,
    pub routing_category: String,
    pub speech_version: String,
    pub ms_classmark3: String,
    pub calling_cell_band: String,
    pub req_fixed_network_user_rate: String,
    pub req_other_modem_type: String,
    pub acceptable_channel_codings: String,
    pub req_number_of_channels: String,
    pub req_air_interface_user_rate: String,
    pub req_user_initiated_mod_ind: String,
    pub used_number_of_channels: String,
    pub used_other_modem_type: String,
    pub used_fixed_nw_user_rate: String,
    pub used_channel_coding: String,
    pub intermediate_chrg_cause: String,
    pub cug_information: String,
    pub in_category_key: String,
    pub camel_call_reference: String,
    pub camel_exchange_id_ton: String,
    pub camel_exchange_id: String,
    pub orig_mcz_tariff_change_cnt: String,
    pub char_band_tariff_change_cnt: String,
    pub calling_modify_parameters: String,
    pub orig_mcz_modify_percent: String,
    pub orig_mcz_modify_direction: String,
    pub orig_dialling_class: String,
    pub npdb_query_status: String,
    pub loc_routing_number: String,
    pub scp_connection: String,
    pub number_of_all_in_records: String,
    pub loc_routing_number_ton: String,
    pub add_routing_category: String,
    pub calling_subs_last_ex_id_ton: String,
    pub called_subs_last_ex_id_ton: String,
    pub calling_subs_first_mcc: String,
    pub calling_subs_first_mnc: String,
    pub calling_subs_last_mcc: String,
    pub calling_subs_last_mnc: String,
    pub called_subs_first_mcc: String,
    pub called_subs_first_mnc: String,
    pub called_subs_last_mcc: String,
    pub called_subs_last_mnc: String,
    pub radio_network_type: String,
    pub used_air_interface_user_rate: String,
    pub stream_identifier: String,
    pub outside_user_plane_index: String,
    pub outside_control_plane_index: String,
    pub out_bnc_connection_type: String,
    pub emergency_call_category: String,
    pub rate_adaption: String,
    pub global_call_reference: String,
    pub virtual_msc_id: String,
    pub scf_address_ton: String,
    pub scf_address: String,
    pub destination_number_ton: String,
    pub destination_number_npi: String,
    pub destination_number: String,
    pub outpulsed_number_ton: String,
    pub outpulsed_number_npi: String,
    pub outpulsed_number: String,
    pub optimal_routing_indicator: String,
    pub calling_imeisv: String,
    pub called_imeisv: String,
    pub out_circuit_group_name: String,
    pub in_circuit_group: String,
    pub in_circuit: String,
    pub virtual_msc_id_ton: String,
    pub virtual_msc_id_npi: String,
    pub in_circuit_group_name: String,
    pub disconnecting_party: String,
    pub location_number_ton: String,
    pub location_number_npi: String,
    pub location_number: String,
    pub fci_data: String,
    pub selected_codec: String,
    pub number_of_in_announcements: String,
    pub nbr_of_orig_cap_in_recs: String,
    pub default_call_handling: String,
}
impl MockBlock {
    pub fn new(bytes: &[u8]) -> Self {
        // Existing fields (already implemented in previous response)
        let record_length = RecordLength::new(&bytes[0..2]).value;
        let record_type = RecordType::new(bytes[2]).value;
        let record_number = RecordNumber::new(&bytes[3..5]).value;
        let record_status = RecordStatus::new(bytes[5]).value;
        let check_sum = CheckSum::new(&bytes[6..8]).value;
        let call_reference = CallReference::new(&bytes[8..16]).value;
        let exchange_id = ExchangeId::new(&bytes[16..25]).value;
        let intermediate_record_number = IntermediateRecordNumber::new(&bytes[25..26]).value;
        let intermediate_charging_ind = IntermediateChargingInd::new(bytes[26]).value;
        let number_of_ss_records = NumberOfSSRecords::new(bytes[27]).value;
        let calling_imsi = IMSI::new(&bytes[28..36]).value;
        let calling_imei = IMEI::new(&bytes[36..44]).value;
        let calling_number = CallingNumber::new(&bytes[44..54]).value;
        let calling_category = Category::new(bytes[54]).value;
        let calling_ms_classmark = MSClassMark::new(bytes[55]).value;
        let called_imsi = IMSI::new(&bytes[56..64]).value;
        let called_imei = IMEI::new(&bytes[64..72]).value;
        let called_number_ton = TON::new(bytes[72]).value;
        let called_number = NUMBER::new(&bytes[73..85]).value;
        let called_category = Category::new(bytes[261]).value;
        let called_ms_classmark = MSClassMark::new(bytes[85]).value;
        let dialled_digits_ton = TON::new(bytes[86]).value;
        let dialled_digits = DialledDigits::new(&bytes[87..99]).value;
        let calling_subs_first_lac = LAC::new(&bytes[99..101]).value;
        let calling_subs_first_ci = CI::new(&bytes[101..103]).value;
        let calling_subs_last_ex_id = LastExId::new(&bytes[103..113]).value;
        let calling_subs_last_lac = LAC::new(&bytes[113..115]).value;
        let calling_subs_last_ci = CI::new(&bytes[115..117]).value;
        let called_subs_first_lac = LAC::new(&bytes[117..119]).value;
        let called_subs_first_ci = CI::new(&bytes[119..121]).value;
        let called_subs_last_ex_id = LastExId::new(&bytes[121..131]).value;
        let called_subs_last_lac = LAC::new(&bytes[131..133]).value;
        let called_subs_last_ci = CI::new(&bytes[133..135]).value;
        let out_circuit_group = OutCircuitGroup::new(&bytes[117..119]).value;
        let out_circuit = OutCircuit::new(&bytes[119..121]).value;
        let basic_service_type = BasicServiceType::new(bytes[121]).value;
        let basic_service_code = BasicServiceCode::new(bytes[122], &basic_service_type).value;
        let facility_usage = FacilityUsage::new(&bytes[123..127]).value;
        let non_transparency_indicator = NonTrasnparencyIndicator::new(bytes[127]).value;
        let channel_rate_indicator = ChannelRateIndicator::new(bytes[128]).value;
        let set_up_start_time = StartTime::new(&bytes[128..135]).value;
        let in_channel_allocated_time = InChannelAllocatedTime::new(&bytes[129..136]).value;
        let b_idle_time = BIdleTime::new(&bytes[212..219]).value;
        let charging_start_time = ChargingStartTime::new(&bytes[136..143]).value;
        let charging_end_time = ChargingEndtime::new(&bytes[143..150]).value;
        let cause_for_termination = CauseForTermination::new(&bytes[150..154]).value;
        let data_volume = DataVolume::new(&bytes[154..158]).value;
        let call_type = CallType::new(bytes[154]).value;
        let dtmf_indicator = DtmfIndicator::new(bytes[155]).value;
        let aoc_indicator = AocIndicator::new(bytes[156]).value;
        let redirected_indicator = RedirectedIndicator::new(bytes[222]).value;
        let cdb_indicator = CDBIndicator::new(bytes[157]).value;
        let orig_mcz_chrg_type = ChargeType::new(bytes[155]).value;
        let orig_mcz_duration = Duration::new(&bytes[156..159]).value;
        let orig_mcz_duration_ten_ms = DurationTenMs::new(&bytes[159..161]).value;
        let orig_mcz_tariff_class = TariffClass::new(&bytes[159..162]).value;
        let orig_mcz_pulses = Pulses::new(&bytes[162..164]).value;
        let orig_mcz_change_percent = ChangePercent::new(bytes[223]).value;
        let orig_mcz_change_direction = ChangeDirection::new(bytes[224]).value;
        let called_msrn_ton = TON::new(bytes[164]).value;
        let called_msrn = MSRN::new(&bytes[165..177]).value;
        let calling_charging_area = ChargingArea::new(&bytes[225..227]).value;
        let called_charging_area = ChargingArea::new(&bytes[227..229]).value;
        let called_msrn_npi = NPI::new(bytes[177]).value;
        let calling_number_ton = TON::new(bytes[177]).value;
        let calling_number_npi = NPI::new(bytes[179]).value;
        let called_number_npi = NPI::new(bytes[180]).value;
        let dialled_digits_npi = NPI::new(bytes[181]).value;
        let connected_to_number_ton = TON::new(bytes[229]).value;
        let connected_to_number_npi = NPI::new(bytes[182]).value;
        let connected_to_number = NUMBER::new(&bytes[230..242]).value;
        let cug_interlock = CugInterlock::new(&bytes[242..246]).value;
        let cug_outgoing_access = CugOutgoingAccess::new(bytes[246]).value;
        let hot_billing_record_number = HotBilingRecordNumber::new(&bytes[248..252]).value;
        let number_of_in_records = NumberOfInRecords::new(bytes[252]).value;
        let regional_subs_indicator = RegionalSubsIndicator::new(bytes[253]).value;
        let regional_subs_location_type = RegionalSubsLocationType::new(bytes[254]).value;
        let leg_call_reference = "<not implemented>".to_string();
        let call_reference_time = CallReferenceTime::new(&bytes[204..211]).value;
        let char_band_chrg_type = ChargeType::new(bytes[183]).value;
        let char_band_duration = Duration::new(&bytes[184..187]).value;
        let char_band_duration_ten_ms = DurationTenMs::new(&bytes[187..189]).value;
        let char_band_tariff_class = TariffClass::new(&bytes[189..192]).value;
        let charg_band_pulses = Pulses::new(&bytes[192..194]).value;
        let char_band_change_percent = ChangePercent::new(bytes[194]).value;
        let char_band_change_direction = ChangeDirection::new(bytes[195]).value;
        let charge_number_ton = TON::new(bytes[196]).value;
        let charge_number_npi = NPI::new(bytes[197]).value;
        let charge_number = NUMBER::new(&bytes[198..210]).value;
        let charge_nature = ChargeNature::new(bytes[210]).value;
        let tns_circuit_code = TNSCircuitCode::new(bytes[211]).value;
        let tns_carrier_code = TNSCarrierCode::new(&bytes[255..257]).value;
        let cip_carrier_code = CIPCarrierCode::new(&bytes[213..215]).value;
        let carrier_selection = CarrierSelection::new(bytes[257]).value;
        let routing_category = RoutingCategory::new(bytes[260]).value;
        let speech_version = SpeechVersion::new(bytes[211]).value;
        let ms_classmark3 = MSClassMark3::new(bytes[316]).value;
        let calling_cell_band = CellBand::new(bytes[262]).value;
        let req_fixed_network_user_rate = FixedNWUserRate::new(bytes[263]).value;
        let req_other_modem_type = OtherModemType::new(bytes[264]).value;
        let acceptable_channel_codings = AcceptableChannelCodings::new(bytes[265]).value;
        let req_number_of_channels = ReqNumberOfChannels::new(bytes[266]).value;
        let req_air_interface_user_rate = ReqAirInterfaceUserRate::new(bytes[267]).value;
        let req_user_initiated_mod_ind = ReqUserInitiatedModInd::new(bytes[268]).value;
        let used_number_of_channels = UsedNumberOfChannels::new(bytes[269]).value;
        let used_other_modem_type = OtherModemType::new(bytes[270]).value;
        let used_fixed_nw_user_rate = FixedNWUserRate::new(bytes[271]).value;
        let used_channel_coding = UsedChannelCoding::new(bytes[272]).value;
        let intermediate_chrg_cause = IntermediateChrgCause::new(&bytes[178..180]).value;
        let cug_information = CugInformation::new(bytes[247]).value;
        let in_category_key = InCategoryKey::new(&bytes[215..216]).value;
        let camel_call_reference = CamelCallReference::new(&bytes[273..281]).value;
        let camel_exchange_id_ton = TON::new(bytes[281]).value;
        let camel_exchange_id = CamelExchangeId::new(&bytes[282..291]).value;
        let orig_mcz_tariff_change_cnt = TariffChangeCNT::new(bytes[216]).value;
        let char_band_tariff_change_cnt = TariffChangeCNT::new(bytes[217]).value;
        let calling_modify_parameters = ModifyParameters::new(&bytes[180..194]).value;
        let orig_mcz_modify_percent = ModifyPercent::new(&bytes[192..196]).value;
        let orig_mcz_modify_direction = ModifyDirection::new(bytes[196]).value;
        let orig_dialling_class = OrigDiallingClass::new(&bytes[197..199]).value;
        let npdb_query_status = NPDBQueryStatus::new(bytes[291]).value;
        let loc_routing_number = NUMBER::new(&bytes[294..306]).value;
        let scp_connection = SCPConnection::new(bytes[292]).value;
        let number_of_all_in_records = NumberOfAllInRecords::new(bytes[293]).value;
        let loc_routing_number_ton = TON::new(bytes[306]).value;
        let add_routing_category = AddRoutingCategory::new(&bytes[307..309]).value;
        let calling_subs_last_ex_id_ton = TON::new(bytes[318]).value;
        let called_subs_last_ex_id_ton = TON::new(bytes[319]).value;
        let calling_subs_first_mcc = MCC::new(&bytes[218..220]).value;
        let calling_subs_first_mnc = MNC::new(&bytes[220..222]).value;
        let calling_subs_last_mcc = MCC::new(&bytes[222..224]).value;
        let calling_subs_last_mnc = MNC::new(&bytes[224..226]).value;
        let called_subs_first_mcc = MCC::new(&bytes[226..228]).value;
        let called_subs_first_mnc = MNC::new(&bytes[228..230]).value;
        let called_subs_last_mcc = MCC::new(&bytes[230..232]).value;
        let called_subs_last_mnc = MNC::new(&bytes[232..234]).value;
        let radio_network_type = RadioNetworkType::new(bytes[320]).value;
        let used_air_interface_user_rate = UsedAirInterfaceUserRate::new(bytes[321]).value;
        let stream_identifier = StreamIdentifier::new(bytes[234]).value;
        let outside_user_plane_index = UserPlaneIndex::new(&bytes[310..312]).value;
        let outside_control_plane_index = ControlPlaneIndex::new(&bytes[312..314]).value;
        let out_bnc_connection_type = BncConnectionType::new(bytes[309]).value;
        let emergency_call_category = EmergencyCallCategory::new(bytes[314]).value;
        let rate_adaption = RateAdaption::new(bytes[315]).value;
        let global_call_reference = GlobalCallReference::new(&bytes[235..243]).value;
        let virtual_msc_id = VirtualMSCId::new(&bytes[243..253]).value;
        let scf_address_ton = TON::new(bytes[253]).value;
        let scf_address = SCFAddress::new(&bytes[254..266]).value;
        let destination_number_ton = TON::new(bytes[266]).value;
        let destination_number_npi = NPI::new(bytes[267]).value;
        let destination_number = NUMBER::new(&bytes[268..280]).value;
        let outpulsed_number_ton = TON::new(bytes[280]).value;
        let outpulsed_number_npi = NPI::new(bytes[281]).value;
        let outpulsed_number = NUMBER::new(&bytes[282..294]).value;
        let optimal_routing_indicator = OptimalRoutingIndicator::new(bytes[294]).value;
        let calling_imeisv = IMEISV::new(&bytes[295..303]).value;
        let called_imeisv = IMEISV::new(&bytes[303..311]).value;
        let out_circuit_group_name = OutCircuitGroupName::new(&bytes[311..321]).value;
        let in_circuit_group = InCircuitGroup::new(&bytes[321..323]).value;
        let in_circuit = InCircuit::new(&bytes[323..325]).value;
        let virtual_msc_id_ton = TON::new(bytes[325]).value;
        let virtual_msc_id_npi = NPI::new(bytes[326]).value;
        let in_circuit_group_name = InCircuitGroupName::new(&bytes[327..337]).value;
        let disconnecting_party = DisconnectingParty::new(bytes[337]).value;
        let location_number_ton = TON::new(bytes[338]).value;
        let location_number_npi = NPI::new(bytes[339]).value;
        let location_number = NUMBER::new(&bytes[340..352]).value;
        let fci_data = "<not implemented>".to_string();
        let selected_codec = SelectedCodec::new(bytes[362]).value;
        let number_of_in_announcements = "<not implemented>".to_string();
        let nbr_of_orig_cap_in_recs = "<not implemented>".to_string();
        let default_call_handling = DefaultCallHandling::new(bytes[365]).value;
        Self {
            record_length,
            record_type,
            record_number,
            record_status,
            check_sum,
            call_reference,
            exchange_id,
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
            called_category,
            called_ms_classmark,
            dialled_digits_ton,
            dialled_digits,
            calling_subs_first_lac,
            calling_subs_first_ci,
            calling_subs_last_ex_id,
            calling_subs_last_lac,
            calling_subs_last_ci,
            called_subs_first_lac,
            called_subs_first_ci,
            called_subs_last_ex_id,
            called_subs_last_lac,
            called_subs_last_ci,
            out_circuit_group,
            out_circuit,
            basic_service_type,
            basic_service_code,
            facility_usage,
            non_transparency_indicator,
            channel_rate_indicator,
            set_up_start_time,
            in_channel_allocated_time,
            b_idle_time,
            charging_start_time,
            charging_end_time,
            cause_for_termination,
            data_volume,
            call_type,
            dtmf_indicator,
            aoc_indicator,
            redirected_indicator,
            cdb_indicator,
            orig_mcz_chrg_type,
            orig_mcz_duration,
            orig_mcz_duration_ten_ms,
            orig_mcz_tariff_class,
            orig_mcz_pulses,
            orig_mcz_change_percent,
            orig_mcz_change_direction,
            called_msrn_ton,
            called_msrn,
            calling_charging_area,
            called_charging_area,
            called_msrn_npi,
            calling_number_ton,
            calling_number_npi,
            called_number_npi,
            dialled_digits_npi,
            connected_to_number_ton,
            connected_to_number_npi,
            connected_to_number,
            cug_interlock,
            cug_outgoing_access,
            hot_billing_record_number,
            number_of_in_records,
            regional_subs_indicator,
            regional_subs_location_type,
            leg_call_reference,
            call_reference_time,
            char_band_chrg_type,
            char_band_duration,
            char_band_duration_ten_ms,
            char_band_tariff_class,
            charg_band_pulses,
            char_band_change_percent,
            char_band_change_direction,
            charge_number_ton,
            charge_number_npi,
            charge_number,
            charge_nature,
            tns_circuit_code,
            tns_carrier_code,
            cip_carrier_code,
            carrier_selection,
            routing_category,
            speech_version,
            ms_classmark3,
            calling_cell_band,
            req_fixed_network_user_rate,
            req_other_modem_type,
            acceptable_channel_codings,
            req_number_of_channels,
            req_air_interface_user_rate,
            req_user_initiated_mod_ind,
            used_number_of_channels,
            used_other_modem_type,
            used_fixed_nw_user_rate,
            used_channel_coding,
            intermediate_chrg_cause,
            cug_information,
            in_category_key,
            camel_call_reference,
            camel_exchange_id_ton,
            camel_exchange_id,
            orig_mcz_tariff_change_cnt,
            char_band_tariff_change_cnt,
            calling_modify_parameters,
            orig_mcz_modify_percent,
            orig_mcz_modify_direction,
            orig_dialling_class,
            npdb_query_status,
            loc_routing_number,
            scp_connection,
            number_of_all_in_records,
            loc_routing_number_ton,
            add_routing_category,
            calling_subs_last_ex_id_ton,
            called_subs_last_ex_id_ton,
            calling_subs_first_mcc,
            calling_subs_first_mnc,
            calling_subs_last_mcc,
            calling_subs_last_mnc,
            called_subs_first_mcc,
            called_subs_first_mnc,
            called_subs_last_mcc,
            called_subs_last_mnc,
            radio_network_type,
            used_air_interface_user_rate,
            stream_identifier,
            outside_user_plane_index,
            outside_control_plane_index,
            out_bnc_connection_type,
            emergency_call_category,
            rate_adaption,
            global_call_reference,
            virtual_msc_id,
            scf_address_ton,
            scf_address,
            destination_number_ton,
            destination_number_npi,
            destination_number,
            outpulsed_number_ton,
            outpulsed_number_npi,
            outpulsed_number,
            optimal_routing_indicator,
            calling_imeisv,
            called_imeisv,
            out_circuit_group_name,
            in_circuit_group,
            in_circuit,
            virtual_msc_id_ton,
            virtual_msc_id_npi,
            in_circuit_group_name,
            disconnecting_party,
            location_number_ton,
            location_number_npi,
            location_number,
            fci_data,
            selected_codec,
            number_of_in_announcements,
            nbr_of_orig_cap_in_recs,
            default_call_handling,
        }
    }
    pub fn to_json_str(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
    pub fn to_json(&self) -> serde_json::Result<Value> {
        serde_json::to_value(self)
    }
}
