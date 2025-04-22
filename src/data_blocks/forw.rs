use crate::datatypes::charging_fields::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FORW {
    pub intermediate_record_number: String,
    pub intermediate_charging_ind: String,
    pub number_of_ss_records: String,
    pub cause_for_forwarding: String,
    pub forwarding_imsi: String,
    pub forwarding_number: String,
    pub forwarded_to_imsi: String,
    pub forwarded_to_imei: String,
    pub forwarded_to_number: String,
    pub forwarded_to_ms_classmark: String,
    pub orig_calling_number: String,
    pub in_circuit_group: String,
    pub in_circuit: String,
    pub out_circuit_group: String,
    pub out_circuit: String,
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
    pub forwarded_to_number_ton: String,
    pub forw_mcz_chrg_type: String,
    pub forw_mcz_duration: String,
    pub forw_mcz_tariff_class: String,
    pub forw_mcz_pulses: String,
    pub forwarded_to_msrn_ton: String,
    pub forwarded_to_msrn: String,
    pub forwarding_number_ton: String,
    pub orig_calling_number_ton: String,
    pub intermediate_chrg_cause: String,
    pub orig_dialling_class: String,
    pub leg_call_reference: String,
    pub call_reference_time: String,
    pub speech_version: String,
    pub b_idle_time: String,
    pub pni: String,
    pub forw_mcz_change_percent: String,
    pub forw_mcz_change_direction: String,
    pub forwarding_charging_area: String,
    pub forwarded_to_charging_area: String,
    pub connected_to_number_ton: String,
    pub connected_to_number: String,
    pub cug_interlock: String,
    pub cug_outgoing_access: String,
    pub cug_information: String,
    pub hot_billing_record_number: String,
    pub spare2: String,
    pub number_of_all_in_records: String,
    pub number_of_in_records: String,
    pub orig_called_number_ton: String,
    pub orig_called_number: String,
    pub tns_carrier_code: String,
    pub carrier_selection: String,
    pub pic: String,
    pub routing_category: String,
    pub ms_classmark3: String,
    pub forwarding_cell_band: String,
    pub camel_call_reference: String,
    pub camel_exchange_id_ton: String,
    pub camel_exchange_id: String,
    pub npdb_query_status: String,
    pub scp_connection: String,
    pub loc_routing_number_ton: String,
    pub loc_routing_number: String,
    pub forwarding_msrn_ton: String,
    pub forwarding_msrn: String,
    pub optimal_routing_indicator: String,
    pub add_routing_category: String,
    pub in_bnc_connection_type: String,
    pub inside_user_plane_index: String,
    pub inside_control_plane_index: String,
    pub out_bnc_connection_type: String,
    pub outside_user_plane_index: String,
    pub outside_control_plane_index: String,
    pub collect_call_indicator: String,
    pub forwarding_first_ci: String,
    pub forwarding_last_ex_id_ton: String,
    pub forwarded_to_last_ex_id_ton: String,
    pub radio_network_type: String,
    pub rate_adaption: String,
}
impl FORW {
    pub fn new(bytes: &[u8]) -> Self {
        let intermediate_record_number = IntermediateRecordNumber::new(&bytes[25..26]).value;
        let intermediate_charging_ind = IntermediateChargingInd::new(bytes[27]).value;
        let number_of_ss_records = NumberOfSSRecords::new(&bytes[27..28]).value;
        let cause_for_forwarding = CauseForForwarding::new(bytes[28]).value;
        let forwarding_imsi = IMSI::new(&bytes[29..37]).value;
        let forwarding_number = Number::new(&bytes[37..49]).value;
        let forwarded_to_imsi = IMSI::new(&bytes[49..57]).value;
        let forwarded_to_imei = IMEI::new(&bytes[57..65]).value;
        let forwarded_to_number = Number::new(&bytes[65..77]).value;
        let forwarded_to_ms_classmark = MSClassMark::new(bytes[77]).value; //    C(  1)        77
        let orig_calling_number = "".to_string(); //    C( 10)        78
        let in_circuit_group = InCircuitGroup::new(&bytes[88..90]).value;
        let in_circuit = InCircuit::new(&bytes[90..92]).value;
        let out_circuit_group = "".to_string(); //  BCD(  2)        92
        let out_circuit = "".to_string(); //  BCD(  2)        94
        let basic_service_type = BasicServiceType::new(bytes[96]).value;
        let basic_service_code = BasicServiceCode::new(bytes[97], bytes[96]).value;
        let facility_usage = FacilityUsage::new(&bytes[98..102]).value; //    C(  4)        98
        let non_transparency_indicator = NonTrasnparencyIndicator::new(bytes[102]).value;
        let channel_rate_indicator = ChannelRateIndicator::new(bytes[103]).value;
        let in_channel_allocated_time = InChannelAllocatedTime::new(&bytes[104..111]).value;
        let charging_start_time = ChargingStartTime::new(&bytes[111..118]).value;
        let charging_end_time = ChargingEndtime::new(&bytes[118..125]).value;
        let cause_for_termination = CauseForTermination::new(&bytes[125..129]).value;
        let call_type = CallType::new(bytes[129]).value;
        let forwarded_to_number_ton = TON::new(bytes[130]).value;
        let forw_mcz_chrg_type = ChargeType::new(bytes[131]).value;
        let forw_mcz_duration = Duration::new(&bytes[132..135]).value;
        let forw_mcz_tariff_class = TariffClass::new(&bytes[135..138]).value;
        let forw_mcz_pulses = Pulses::new(&bytes[138..140]).value;
        let forwarded_to_msrn_ton = TON::new(bytes[140]).value;
        let forwarded_to_msrn = MSRN::new(&bytes[141..153]).value;
        let forwarding_number_ton = TON::new(bytes[153]).value;
        let orig_calling_number_ton = TON::new(bytes[154]).value;
        let intermediate_chrg_cause = "".to_string(); // IntermediateChrgCause::new(&bytes[155..157]).value;
        let orig_dialling_class = "".to_string(); //    W(  1)       157
        let leg_call_reference = "".to_string(); // TODO                  //    C(  5)       159
        let call_reference_time = CallReferenceTime::new(&bytes[164..171]).value;
        let speech_version = "".to_string(); //    C(  1)       171
        let b_idle_time = BIdleTime::new(&bytes[172..179]).value;
        let pni = PNI::new(&bytes[179..182]).value;
        let forw_mcz_change_percent = ChangePercent::new(bytes[182]).value;
        let forw_mcz_change_direction = ChangeDirection::new(bytes[183]).value;
        let forwarding_charging_area = ChargingArea::new(&bytes[184..186]).value;
        let forwarded_to_charging_area = ChargingArea::new(&bytes[186..188]).value;
        let connected_to_number_ton = TON::new(bytes[188]).value;
        let connected_to_number = Number::new(&bytes[189..201]).value;
        let cug_interlock = "".to_string(); //TODO           //    C(  4)       201
        let cug_outgoing_access = CugOutgoingAccess::new(bytes[205]).value;
        let cug_information = CugInformation::new(bytes[206]).value;
        let hot_billing_record_number = HotBilingRecordNumber::new(&bytes[207..211]).value;
        let spare2 = "".to_string(); //    C(  1)       211
        let number_of_all_in_records = NumberOfAllInRecords::new(&bytes[212..213]).value;
        let number_of_in_records = NumberOfInRecords::new(bytes[213]).value;
        let orig_called_number_ton = "".to_string(); //    C(  1)       214
        let orig_called_number = "".to_string(); //    C( 12)       215
        let tns_carrier_code = "".to_string(); //    W(  1)       227
        let carrier_selection = "".to_string(); //    C(  1)       229
        let pic = "".to_string(); //    W(  1)       230
        let routing_category = "".to_string(); //    C(  1)       232
        let ms_classmark3 = "".to_string(); //    C(  1)       233
        let forwarding_cell_band = "".to_string(); //    C(  1)       234
        let camel_call_reference = "".to_string(); //    C(  8)       235
        let camel_exchange_id_ton = "".to_string(); //    C(  1)       243
        let camel_exchange_id = "".to_string(); //    C(  9)       244
        let npdb_query_status = "".to_string(); //    C(  1)       253
        let scp_connection = "".to_string(); //    C(  1)       254
        let loc_routing_number_ton = "".to_string(); //    C(  1)       255
        let loc_routing_number = "".to_string(); //    C( 12)       256
        let forwarding_msrn_ton = "".to_string(); //    C(  1)       268
        let forwarding_msrn = "".to_string(); //    C( 12)       269
        let optimal_routing_indicator = "".to_string(); //    C(  1)       281
        let add_routing_category = "".to_string(); //    W(  1)       282
        let in_bnc_connection_type = "".to_string(); //    C(  1)       284
        let inside_user_plane_index = "".to_string(); //  BCD(  2)       285
        let inside_control_plane_index = "".to_string(); //  BCD(  2)       287
        let out_bnc_connection_type = "".to_string(); //    C(  1)       289
        let outside_user_plane_index = "".to_string(); //  BCD(  2)       290
        let outside_control_plane_index = "".to_string(); //  BCD(  2)       292
        let collect_call_indicator = "".to_string(); //    C(  1)       294
        let forwarding_first_ci = "".to_string(); //    W(  1)       295
        let forwarding_last_ex_id_ton = "".to_string(); //    C(  1)       297
        let forwarded_to_last_ex_id_ton = "".to_string(); //    C(  1)       298
        let radio_network_type = "".to_string(); //    C(  1)       299
        let rate_adaption = "".to_string(); //    C(  1)       300

        Self {
            intermediate_record_number,
            intermediate_charging_ind,
            number_of_ss_records,
            cause_for_forwarding,
            forwarding_imsi,
            forwarding_number,
            forwarded_to_imsi,
            forwarded_to_imei,
            forwarded_to_number,
            forwarded_to_ms_classmark,
            orig_calling_number,
            in_circuit_group,
            in_circuit,
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
            forwarded_to_number_ton,
            forw_mcz_chrg_type,
            forw_mcz_duration,
            forw_mcz_tariff_class,
            forw_mcz_pulses,
            forwarded_to_msrn_ton,
            forwarded_to_msrn,
            forwarding_number_ton,
            orig_calling_number_ton,
            intermediate_chrg_cause,
            orig_dialling_class,
            leg_call_reference,
            call_reference_time,
            speech_version,
            b_idle_time,
            pni,
            forw_mcz_change_percent,
            forw_mcz_change_direction,
            forwarding_charging_area,
            forwarded_to_charging_area,
            connected_to_number_ton,
            connected_to_number,
            cug_interlock,
            cug_outgoing_access,
            cug_information,
            hot_billing_record_number,
            spare2,
            number_of_all_in_records,
            number_of_in_records,
            orig_called_number_ton,
            orig_called_number,
            tns_carrier_code,
            carrier_selection,
            pic,
            routing_category,
            ms_classmark3,
            forwarding_cell_band,
            camel_call_reference,
            camel_exchange_id_ton,
            camel_exchange_id,
            npdb_query_status,
            scp_connection,
            loc_routing_number_ton,
            loc_routing_number,
            forwarding_msrn_ton,
            forwarding_msrn,
            optimal_routing_indicator,
            add_routing_category,
            in_bnc_connection_type,
            inside_user_plane_index,
            inside_control_plane_index,
            out_bnc_connection_type,
            outside_user_plane_index,
            outside_control_plane_index,
            collect_call_indicator,
            forwarding_first_ci,
            forwarding_last_ex_id_ton,
            forwarded_to_last_ex_id_ton,
            radio_network_type,
            rate_adaption,
        }
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

// FORMAT TYPE:      3
// MESSAGE NUMBER:   dd96
// FORMAT TYPE NAME: FORW Forwarded Call
// RECORD LENGTH:    301

// DATA:
// FIELD NAME                                   DATA TYPE  POSITION

// intermediate_record_number                    BCD(  1)        25
// intermediate_charging_ind                       C(  1)        26
// number_of_ss_records                          BCD(  1)        27
// cause_for_forwarding                            C(  1)        28
// forwarding_imsi                                 C(  8)        29
// forwarding_number                               C( 12)        37
// forwarded_to_imsi                               C(  8)        49
// forwarded_to_imei                               C(  8)        57
// forwarded_to_number                             C( 12)        65
// forwarded_to_ms_classmark                       C(  1)        77
// orig_calling_number                             C( 10)        78
// in_circuit_group                              BCD(  2)        88
// in_circuit                                    BCD(  2)        90
// out_circuit_group                             BCD(  2)        92
// out_circuit                                   BCD(  2)        94
// basic_service_type                              C(  1)        96
// basic_service_code                              C(  1)        97
// facility_usage                                  C(  4)        98
// non_transparency_indicator                      C(  1)       102
// channel_rate_indicator                          C(  1)       103
// in_channel_allocated_time                       C(  7)       104
// charging_start_time                             C(  7)       111
// charging_end_time                               C(  7)       118
// cause_for_termination                          DW(  1)       125
// call_type                                       C(  1)       129
// forwarded_to_number_ton                         C(  1)       130
// forw_mcz_chrg_type                              C(  1)       131
// forw_mcz_duration                             BCD(  3)       132
// forw_mcz_tariff_class                         BCD(  3)       135
// forw_mcz_pulses                               BCD(  2)       138
// forwarded_to_msrn_ton                           C(  1)       140
// forwarded_to_msrn                               C( 12)       141
// forwarding_number_ton                           C(  1)       153
// orig_calling_number_ton                         C(  1)       154
// intermediate_chrg_cause                         C(  2)       155
// orig_dialling_class                             W(  1)       157
// leg_call_reference                              C(  5)       159
// call_reference_time                             C(  7)       164
// speech_version                                  C(  1)       171
// b_idle_time                                     C(  7)       172
// pni                                             C(  3)       179
// forw_mcz_change_percent                         C(  1)       182
// forw_mcz_change_direction                       C(  1)       183
// forwarding_charging_area                        W(  1)       184
// forwarded_to_charging_area                      W(  1)       186
// connected_to_number_ton                         C(  1)       188
// connected_to_number                             C( 12)       189
// cug_interlock                                   C(  4)       201
// cug_outgoing_access                             C(  1)       205
// cug_information                                 C(  1)       206
// hot_billing_record_number                     BCD(  4)       207
// spare2                                          C(  1)       211
// number_of_all_in_records                      BCD(  1)       212
// number_of_in_records                          BCD(  1)       213
// orig_called_number_ton                          C(  1)       214
// orig_called_number                              C( 12)       215
// tns_carrier_code                                W(  1)       227
// carrier_selection                               C(  1)       229
// pic                                             W(  1)       230
// routing_category                                C(  1)       232
// ms_classmark3                                   C(  1)       233
// forwarding_cell_band                            C(  1)       234
// camel_call_reference                            C(  8)       235
// camel_exchange_id_ton                           C(  1)       243
// camel_exchange_id                               C(  9)       244
// npdb_query_status                               C(  1)       253
// scp_connection                                  C(  1)       254
// loc_routing_number_ton                          C(  1)       255
// loc_routing_number                              C( 12)       256
// forwarding_msrn_ton                             C(  1)       268
// forwarding_msrn                                 C( 12)       269
// optimal_routing_indicator                       C(  1)       281
// add_routing_category                            W(  1)       282
// in_bnc_connection_type                          C(  1)       284
// inside_user_plane_index                       BCD(  2)       285
// inside_control_plane_index                    BCD(  2)       287
// out_bnc_connection_type                         C(  1)       289
// outside_user_plane_index                      BCD(  2)       290
// outside_control_plane_index                   BCD(  2)       292
// collect_call_indicator                          C(  1)       294
// forwarding_first_ci                             W(  1)       295
// forwarding_last_ex_id_ton                       C(  1)       297
// forwarded_to_last_ex_id_ton                     C(  1)       298
// radio_network_type                              C(  1)       299
// rate_adaption                                   C(  1)       300
