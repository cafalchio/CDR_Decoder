use crate::datatypes::charging_fields::*;
use crate::datatypes::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ROAM {
    pub intermediate_record_number: String,
    pub intermediate_charging_ind: String,
    pub number_of_ss_records: String,
    pub calling_number: String,
    pub called_imsi: String,
    pub called_number: String,
    pub called_msrn: String,
    pub in_circuit_group: String,
    pub in_circuit: String,
    pub out_circuit_group: String,
    pub out_circuit: String,
    pub basic_service_type: String,
    pub basic_service_code: String,
    pub facility_usage: String,
    pub in_channel_allocated_time: String,
    pub charging_start_time: String,
    pub charging_end_time: String,
    pub cause_for_termination: String,
    pub call_type: String,
    pub roam_mcz_chrg_type: String,
    pub roam_mcz_duration: String,
    pub roam_mcz_tariff_class: String,
    pub roam_mcz_pulses: String,
    pub called_msrn_ton: String,
    pub calling_number_ton: String,
    pub called_number_ton: String,
    pub intermediate_chrg_cause: String,
    pub leg_call_reference: String,
    pub out_channel_allocated_time: String,
    pub call_reference_time: String,
    pub b_idle_time: String,
    pub roam_mcz_change_percent: String,
    pub roam_mcz_change_direction: String,
    pub calling_charging_area: String,
    pub called_charging_area: String,
    pub routing_category: String,
    pub cug_interlock: String,
    pub cug_outgoing_access: String,
    pub cug_information: String,
    pub hot_billing_record_number: String,
    pub number_of_in_records: String,
    pub tns_carrier_code: String,
    pub carrier_selection: String,
    pub pic: String,
    pub camel_call_reference: String,
    pub camel_exchange_id_ton: String,
    pub camel_exchange_id: String,
    pub scp_connection: String,
    pub number_of_all_in_records: String,
    pub add_routing_category: String,
    pub in_bnc_connection_type: String,
    pub inside_user_plane_index: String,
    pub inside_control_plane_index: String,
    pub out_bnc_connection_type: String,
    pub outside_user_plane_index: String,
    pub outside_control_plane_index: String,
    pub collect_call_indicator: String,
    pub redirecting_number: String,
    pub rate_adaption: String,
}

impl ROAM {
    pub fn new(bytes: &[u8]) -> Self {
        let intermediate_record_number = IntermediateRecordNumber::new(&bytes[25..26]).value; //   BCD(  1)        25
        let intermediate_charging_ind = IntermediateChargingInd::new(bytes[26]).value; //     C(  1)        26
        let number_of_ss_records = NumberOfSSRecords::new(bytes[27]).value; //   BCD(  1)        27
        let calling_number = CallingNumber::new(&bytes[28..38]).value; //     C( 10)        28
        let called_imsi = IMSI::new(&bytes[38..46]).value; //     C(  8)        38
        let called_number = NUMBER::new(&bytes[46..58]).value; //     C( 12)        46
        let called_msrn = MSRN::new(&bytes[58..70]).value; //     C( 12)        58
        let in_circuit_group = InCircuitGroup::new(&bytes[70..72]).value; //   BCD(  2)        70
        let in_circuit = InCircuit::new(&bytes[72..74]).value; //   BCD(  2)        72
        let out_circuit_group = OutCircuitGroup::new(&bytes[74..76]).value; //   BCD(  2)        74
        let out_circuit = OutCircuit::new(&bytes[76..78]).value; //   BCD(  2)        76
        let basic_service_type = BasicServiceType::new(bytes[78]).value; //     C(  1)        78
        let basic_service_code = BasicServiceCode::new(bytes[79], &basic_service_type).value; //     C(  1)        79
        let facility_usage = FacilityUsage::new(&bytes[80..84]).value; //     C(  4)        80
        let in_channel_allocated_time = InChannelAllocatedTime::new(&bytes[84..91]).value; //     C(  7)        84
        let charging_start_time = ChargingStartTime::new(&bytes[91..98]).value; //     C(  7)        91
        let charging_end_time = BcdTimestamp::new(&bytes[98..105]).value; //     C(  7)        98
        let cause_for_termination = CauseForTermination::new(&bytes[105..109]).value; //    DW(  1)       105
        let call_type = CallType::new(bytes[109]).value; //     C(  1)       109
        let roam_mcz_chrg_type = ChargeType::new(bytes[110]).value; //     C(  1)       110
        let roam_mcz_duration = Duration::new(&bytes[111..114]).value; //   BCD(  3)       111
        let roam_mcz_tariff_class = TariffClass::new(&bytes[114..117]).value; //   BCD(  3)       114
        let roam_mcz_pulses = Pulses::new(&bytes[117..119]).value; //   BCD(  2)       117
        let called_msrn_ton = TON::new(bytes[119]).value; //     C(  1)       119
        let calling_number_ton = TON::new(bytes[120]).value; //     C(  1)       120
        let called_number_ton = TON::new(bytes[121]).value; //     C(  1)       121
        let intermediate_chrg_cause = IntermediateChrgCause::new(&bytes[122..124]).value; //     C(  2)       122
                                                                                          // let leg_call_reference = LegCallReference::new(&bytes[124..129]).value; //     C(  5)       124
        let leg_call_reference = "<not implemented>".to_string();
        let out_channel_allocated_time = OutChannelAllocatedTime::new(&bytes[129..136]).value; //     C(  7)       129
        let call_reference_time = CallReferenceTime::new(&bytes[136..143]).value; //     C(  7)       136
        let b_idle_time = BIdleTime::new(&bytes[143..150]).value; //     C(  7)       143
        let roam_mcz_change_percent = ChangePercent::new(bytes[150]).value; //     C(  1)       150
        let roam_mcz_change_direction = ChangeDirection::new(bytes[151]).value; //     C(  1)       151
        let calling_charging_area = ChargingArea::new(&bytes[152..154]).value; //     W(  1)       152
        let called_charging_area = ChargingArea::new(&bytes[154..156]).value; //     W(  1)       154
        let routing_category = RoutingCategory::new(bytes[156]).value; //     C(  1)       156
        let cug_interlock = CugInterlock::new(&bytes[157..161]).value; //     C(  4)       157
        let cug_outgoing_access = CugOutgoingAccess::new(bytes[161]).value; //     C(  1)       161
        let cug_information = CugInformation::new(bytes[162]).value; //     C(  1)       162
        let hot_billing_record_number = HotBilingRecordNumber::new(&bytes[163..167]).value; //   BCD(  4)       163
        let number_of_in_records = NumberOfInRecords::new(bytes[167]).value; //   BCD(  1)       167
        let tns_carrier_code = TNSCarrierCode::new(&bytes[168..170]).value; //     W(  1)       168
        let carrier_selection = CarrierSelection::new(bytes[170]).value; //     C(  1)       170
        let pic = PIC::new(&bytes[171..173]).value; //     W(  1)       171
        let camel_call_reference = CamelCallReference::new(&bytes[173..181]).value; //     C(  8)       173
        let camel_exchange_id_ton = TON::new(bytes[181]).value; //     C(  1)       181
        let camel_exchange_id = CamelExchangeId::new(&bytes[182..191]).value; //     C(  9)       182
        let scp_connection = SCPConnection::new(bytes[191]).value; //     C(  1)       191
        let number_of_all_in_records = NumberOfAllInRecords::new(bytes[192]).value; //   BCD(  1)       192
        let add_routing_category = AddRoutingCategory::new(&bytes[193..195]).value; //     W(  1)       193
        let in_bnc_connection_type = BncConnectionType::new(bytes[195]).value; //     C(  1)       195
        let inside_user_plane_index = UserPlaneIndex::new(&bytes[196..198]).value; //   BCD(  2)       196
        let inside_control_plane_index = ControlPlaneIndex::new(&bytes[198..200]).value; //   BCD(  2)       198
        let out_bnc_connection_type = BncConnectionType::new(bytes[200]).value; //     C(  1)       200
        let outside_user_plane_index = UserPlaneIndex::new(&bytes[201..203]).value; //   BCD(  2)       201
        let outside_control_plane_index = ControlPlaneIndex::new(&bytes[203..205]).value; //   BCD(  2)       203
                                                                                          // let collect_call_indicator = CollectCallIndicator::new(bytes[205]).value; //     C(  1)       205
        let collect_call_indicator = "<not implemented>".to_string();
        let redirecting_number = NUMBER::new(&bytes[206..218]).value; //     C( 12)       206
        let rate_adaption = RateAdaption::new(bytes[218]).value; //     C(  1)       218
        Self {
            intermediate_record_number,
            intermediate_charging_ind,
            number_of_ss_records,
            calling_number,
            called_imsi,
            called_number,
            called_msrn,
            in_circuit_group,
            in_circuit,
            out_circuit_group,
            out_circuit,
            basic_service_type,
            basic_service_code,
            facility_usage,
            in_channel_allocated_time,
            charging_start_time,
            charging_end_time,
            cause_for_termination,
            call_type,
            roam_mcz_chrg_type,
            roam_mcz_duration,
            roam_mcz_tariff_class,
            roam_mcz_pulses,
            called_msrn_ton,
            calling_number_ton,
            called_number_ton,
            intermediate_chrg_cause,
            leg_call_reference,
            out_channel_allocated_time,
            call_reference_time,
            b_idle_time,
            roam_mcz_change_percent,
            roam_mcz_change_direction,
            calling_charging_area,
            called_charging_area,
            routing_category,
            cug_interlock,
            cug_outgoing_access,
            cug_information,
            hot_billing_record_number,
            number_of_in_records,
            tns_carrier_code,
            carrier_selection,
            pic,
            camel_call_reference,
            camel_exchange_id_ton,
            camel_exchange_id,
            scp_connection,
            number_of_all_in_records,
            add_routing_category,
            in_bnc_connection_type,
            inside_user_plane_index,
            inside_control_plane_index,
            out_bnc_connection_type,
            outside_user_plane_index,
            outside_control_plane_index,
            collect_call_indicator,
            redirecting_number,
            rate_adaption,
        }
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

// DATA:
// FIELD NAME                                   DATA TYPE  POSITION

// intermediate_record_number                    BCD(  1)        25
// intermediate_charging_ind                       C(  1)        26
// number_of_ss_records                          BCD(  1)        27
// calling_number                                  C( 10)        28
// called_imsi                                     C(  8)        38
// called_number                                   C( 12)        46
// called_msrn                                     C( 12)        58
// in_circuit_group                              BCD(  2)        70
// in_circuit                                    BCD(  2)        72
// out_circuit_group                             BCD(  2)        74
// out_circuit                                   BCD(  2)        76
// basic_service_type                              C(  1)        78
// basic_service_code                              C(  1)        79
// facility_usage                                  C(  4)        80
// in_channel_allocated_time                       C(  7)        84
// charging_start_time                             C(  7)        91
// charging_end_time                               C(  7)        98
// cause_for_termination                          DW(  1)       105
// call_type                                       C(  1)       109
// roam_mcz_chrg_type                              C(  1)       110
// roam_mcz_duration                             BCD(  3)       111
// roam_mcz_tariff_class                         BCD(  3)       114
// roam_mcz_pulses                               BCD(  2)       117
// called_msrn_ton                                 C(  1)       119
// calling_number_ton                              C(  1)       120
// called_number_ton                               C(  1)       121
// intermediate_chrg_cause                         C(  2)       122
// leg_call_reference                              C(  5)       124
// out_channel_allocated_time                      C(  7)       129
// call_reference_time                             C(  7)       136
// b_idle_time                                     C(  7)       143
// roam_mcz_change_percent                         C(  1)       150
// roam_mcz_change_direction                       C(  1)       151
// calling_charging_area                           W(  1)       152
// called_charging_area                            W(  1)       154
// routing_category                                C(  1)       156
// cug_interlock                                   C(  4)       157
// cug_outgoing_access                             C(  1)       161
// cug_information                                 C(  1)       162
// hot_billing_record_number                     BCD(  4)       163
// number_of_in_records                          BCD(  1)       167
// tns_carrier_code                                W(  1)       168
// carrier_selection                               C(  1)       170
// pic                                             W(  1)       171
// camel_call_reference                            C(  8)       173
// camel_exchange_id_ton                           C(  1)       181
// camel_exchange_id                               C(  9)       182
// scp_connection                                  C(  1)       191
// number_of_all_in_records                      BCD(  1)       192
// add_routing_category                            W(  1)       193
// in_bnc_connection_type                          C(  1)       195
// inside_user_plane_index                       BCD(  2)       196
// inside_control_plane_index                    BCD(  2)       198
// out_bnc_connection_type                         C(  1)       200
// outside_user_plane_index                      BCD(  2)       201
// outside_control_plane_index                   BCD(  2)       203
// collect_call_indicator                          C(  1)       205
// redirecting_number                              C( 12)       206
// rate_adaption                                   C(  1)       218
