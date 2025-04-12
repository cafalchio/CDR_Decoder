use crate::datatypes::mixed::*;
use crate::datatypes::primitives::*;



pub struct Moc {
    intermediate_record_number: u8 ,                   //BCD(  1)        25
    intermediate_charging_ind: u8 ,                    //  C(  1)        26 IntermediateChargingInd
    number_of_ss_records: u8 ,                         //BCD(  1)        27
    calling_imsi:u32 ,                                 //  C(  8)        28
    calling_imei:u32 ,                                 //  C(  8)        36
    calling_number:u8 ,                                //  C( 10)        44
    calling_category:u8 ,                              //  C(  1)        54
    calling_ms_classmark:u8 ,                          //  C(  1)        55
    called_imsi:u8 ,                                   //  C(  8)        56
    called_imei:u8 ,                                   //  C(  8)        64
    called_number_ton:u8 ,                             //  C(  1)        72
    called_number:u8 ,                                 //  C( 12)        73
    called_ms_classmark:u8 ,                           //  C(  1)        85
    dialled_digits_ton:u8 ,                            //  C(  1)        86
    dialled_digits:u8 ,                                //  C( 12)        87
    calling_subs_first_lac:u8 ,                        //  W(  1)        99
    calling_subs_first_ci:u8 ,                         //  W(  1)       101
    calling_subs_last_ex_id:u8 ,                       //  C( 10)       103
    calling_subs_last_lac:u8 ,                         //  W(  1)       113
    calling_subs_last_ci:u8 ,                          //  W(  1)       115
    out_circuit_group:u8 ,                             //BCD(  2)       117
    out_circuit:u8 ,                                   //BCD(  2)       119
    basic_service_type:u8 ,                            //  C(  1)       121
    basic_service_code:u8 ,                            //  C(  1)       122
    facility_usage:u8 ,                                //  C(  4)       123
    non_transparency_indicator:u8 ,                    //  C(  1)       127
    channel_rate_indicator:u8 ,                        //  C(  1)       128
    in_channel_allocated_time:u8 ,                     //  C(  7)       129
    charging_start_time:u8 ,                           //  C(  7)       136
    charging_end_time:u8 ,                             //  C(  7)       143
    cause_for_termination:u8 ,                         // DW(  1)       150
    call_type:u8 ,                                     //  C(  1)       154
    orig_mcz_chrg_type:u8 ,                            //  C(  1)       155
    orig_mcz_duration:u8 ,                             //BCD(  3)       156
    orig_mcz_tariff_class:u8 ,                         //BCD(  3)       159
    orig_mcz_pulses:u8 ,                               //BCD(  2)       162
    called_msrn_ton:u8 ,                               //  C(  1)       164
    called_msrn:u8 ,                                   //  C( 12)       165
    calling_number_ton:u8 ,                            //  C(  1)       177
    intermediate_chrg_cause:u8 ,                       //  C(  2)       178
    calling_modify_parameters:u8 ,                     //  C( 14)       180
    orig_mcz_modify_percent:u8 ,                       //  W(  1)       194
    orig_mcz_modify_direction:u8 ,                     //  C(  1)       196
    orig_dialling_class:u8 ,                           //  W(  1)       197
    leg_call_reference:u8 ,                            //  C(  5)       199
    call_reference_time:u8 ,                           //  C(  7)       204
    speech_version:u8 ,                                //  C(  1)       211
    b_idle_time:u8 ,                                   //  C(  7)       212
    pni:u8 ,                                           //  C(  3)       219
    redirected_indicator:u8 ,                          //  C(  1)       222
    orig_mcz_change_percent:u8 ,                       //  C(  1)       223
    orig_mcz_change_direction:u8 ,                     //  C(  1)       224
    calling_charging_area:u8 ,                         //  W(  1)       225
    called_charging_area:u8 ,                          //  W(  1)       227
    connected_to_number_ton:u8 ,                       //  C(  1)       229
    connected_to_number:u8 ,                           //  C( 12)       230
    cug_interlock:u8 ,                                 //  C(  4)       242
    cug_outgoing_access:u8 ,                           //  C(  1)       246
    cug_information:u8 ,                               //  C(  1)       247
    hot_billing_record_number:u8 ,                     //BCD(  4)       248
    number_of_in_records:u8 ,                          //BCD(  1)       252
    regional_subs_indicator:u8 ,                       //  C(  1)       253
    regional_subs_location_type:u8 ,                   //  C(  1)       254
    tns_carrier_code:u8 ,                              //  W(  1)       255
    carrier_selection:u8 ,                             //  C(  1)       257
    pic:u8 ,                                           //  W(  1)       258
    routing_category:u8 ,                              //  C(  1)       260
    called_category:u8 ,                               //  C(  1)       261
    calling_cell_band:u8 ,                             //  C(  1)       262
    req_fixed_nw_user_rate:u8 ,                        //  C(  1)       263
    req_other_modem_type:u8 ,                          //  C(  1)       264
    acceptable_channel_codings:u8 ,                    //  C(  1)       265
    req_number_of_channels:u8 ,                        //  C(  1)       266
    req_air_interface_user_rate:u8 ,                   //  C(  1)       267
    req_user_initiated_mod_ind:u8 ,                    //  C(  1)       268
    used_number_of_channels:u8 ,                       //  C(  1)       269
    used_other_modem_type:u8 ,                         //  C(  1)       270
    used_fixed_nw_user_rate:u8 ,                       //  C(  1)       271
    used_channel_coding:u8 ,                           //  C(  1)       272
    camel_call_reference:u8 ,                          //  C(  8)       273
    camel_exchange_id_ton:u8 ,                         //  C(  1)       281
    camel_exchange_id:u8 ,                             //  C(  9)       282
    npdb_query_status:u8 ,                             //  C(  1)       291
    scp_connection:u8 ,                                //  C(  1)       292
    number_of_all_in_records:u8 ,                      //BCD(  1)       293
    loc_routing_number:u8 ,                            //  C( 12)       294
    loc_routing_number_ton:u8 ,                        //  C(  1)       306
    add_routing_category:u8 ,                          //  W(  1)       307
    out_bnc_connection_type:u8 ,                       //  C(  1)       309
    outside_user_plane_index:u8 ,                      //BCD(  2)       310
    outside_control_plane_index:u8 ,                   //BCD(  2)       312
    emergency_call_category:u8 ,                       //  C(  1)       314
    rate_adaption:u8 ,                                 //  C(  1)       315
    ms_classmark3:u8 ,                                 //  C(  1)       316
    collect_call_indicator:u8 ,                        //  C(  1)       317
    calling_subs_last_ex_id_ton:u8 ,                   //  C(  1)       318
    called_subs_last_ex_id_ton:u8 ,                    //  C(  1)       319
    radio_network_type:u8 ,                            //  C(  1)       320
    used_air_interface_user_rate:u8 ,                  //  C(  1)       321
}

impl Moc {
    pub fn new(bytes: &[u8]) -> Moc {
        let intermediate_record_number = BCD::try_from(&byte[25]).value;   //BCD(  1)        25
        let intermediate_charging_ind =  HByte::new(&byte[26]).value;      //  C(  1)        26 IntermediateChargingInd
        let number_of_ss_records =  BCD::try_from(&byte[27]).value;       //BCD(  1)        27
        let calling_imsi:u32 ;                                       //  C(  8)        28
        let calling_imei:u32 ;                                       //  C(  8)        36
        let calling_number:u8 ;                                      //  C( 10)        44
        let calling_category:u8 ;                                    //  C(  1)        54
        let calling_ms_classmark:u8 ;                                //  C(  1)        55
        let called_imsi:u8 ;                                         //  C(  8)        56
        let called_imei:u8 ;                                         //  C(  8)        64
        let called_number_ton:u8 ;                                   //  C(  1)        72
        let called_number:u8 ;                                       //  C( 12)        73
        let called_ms_classmark:u8 ;                                 //  C(  1)        85
        let dialled_digits_ton:u8 ;                                  //  C(  1)        86
        let dialled_digits:u8 ;                                      //  C( 12)        87
        let calling_subs_first_lac:u8 ;                              //  W(  1)        99
        let calling_subs_first_ci:u8 ;                               //  W(  1)       101
        let calling_subs_last_ex_id:u8 ;                             //  C( 10)       103
        let calling_subs_last_lac:u8 ;                               //  W(  1)       113
        let calling_subs_last_ci:u8 ;                                //  W(  1)       115
        let out_circuit_group = decode_bcds(&bytes[117..119]);       //BCD(  2)       117
        let out_circuit:u8 ;                                         //BCD(  2)       119
        let basic_service_type:u8 ;                                  //  C(  1)       121
        let basic_service_code:u8 ;                                  //  C(  1)       122
        let facility_usage:u8 ;                                      //  C(  4)       123
        let non_transparency_indicator:u8 ;                          //  C(  1)       127
        let channel_rate_indicator:u8 ;                              //  C(  1)       128
        let in_channel_allocated_time:u8 ;                           //  C(  7)       129
        let charging_start_time:u8 ;                                 //  C(  7)       136
        let charging_end_time:u8 ;                                   //  C(  7)       143
        let cause_for_termination:u8 ;                               // DW(  1)       150
        let call_type:u8 ;                                           //  C(  1)       154
        let orig_mcz_chrg_type:u8 ;                                  //  C(  1)       155
        let orig_mcz_duration:u8 ;                                   //BCD(  3)       156
        let orig_mcz_tariff_class:u8 ;                               //BCD(  3)       159
        let orig_mcz_pulses:u8 ;                                     //BCD(  2)       162
        let called_msrn_ton:u8 ;                                     //  C(  1)       164
        let called_msrn:u8 ;                                         //  C( 12)       165
        let calling_number_ton:u8 ;                                  //  C(  1)       177
        let intermediate_chrg_cause:u8 ;                             //  C(  2)       178
        let calling_modify_parameters:u8 ;                           //  C( 14)       180
        let orig_mcz_modify_percent:u8 ;                             //  W(  1)       194
        let orig_mcz_modify_direction:u8 ;                           //  C(  1)       196
        let orig_dialling_class:u8 ;                                 //  W(  1)       197
        let leg_call_reference:u8 ;                                  //  C(  5)       199
        let call_reference_time:u8 ;                                 //  C(  7)       204
        let speech_version:u8 ;                                      //  C(  1)       211
        let b_idle_time:u8 ;                                         //  C(  7)       212
        let pni:u8 ;                                                 //  C(  3)       219
        let redirected_indicator:u8 ;                                //  C(  1)       222
        let orig_mcz_change_percent:u8 ;                             //  C(  1)       223
        let orig_mcz_change_direction:u8 ;                           //  C(  1)       224
        let calling_charging_area:u8 ;                               //  W(  1)       225
        let called_charging_area:u8 ;                                //  W(  1)       227
        let connected_to_number_ton:u8 ;                             //  C(  1)       229
        let connected_to_number:u8 ;                                 //  C( 12)       230
        let cug_interlock:u8 ;                                       //  C(  4)       242
        let cug_outgoing_access:u8 ;                                 //  C(  1)       246
        let cug_information:u8 ;                                     //  C(  1)       247
        let hot_billing_record_number:u8 ;                           //BCD(  4)       248
        let number_of_in_records:u8 ;                                //BCD(  1)       252
        let regional_subs_indicator:u8 ;                             //  C(  1)       253
        let regional_subs_location_type:u8 ;                         //  C(  1)       254
        let tns_carrier_code:u8 ;                                    //  W(  1)       255
        let carrier_selection:u8 ;                                   //  C(  1)       257
        let pic:u8 ;                                                 //  W(  1)       258
        let routing_category:u8 ;                                    //  C(  1)       260
        let called_category:u8 ;                                     //  C(  1)       261
        let calling_cell_band:u8 ;                                   //  C(  1)       262
        let req_fixed_nw_user_rate:u8 ;                              //  C(  1)       263
        let req_other_modem_type:u8 ;                                //  C(  1)       264
        let acceptable_channel_codings:u8 ;                          //  C(  1)       265
        let req_number_of_channels:u8 ;                              //  C(  1)       266
        let req_air_interface_user_rate:u8 ;                         //  C(  1)       267
        let req_user_initiated_mod_ind:u8 ;                          //  C(  1)       268
        let used_number_of_channels:u8 ;                             //  C(  1)       269
        let used_other_modem_type:u8 ;                               //  C(  1)       270
        let used_fixed_nw_user_rate:u8 ;                             //  C(  1)       271
        let used_channel_coding:u8 ;                                 //  C(  1)       272
        let camel_call_reference:u8 ;                                //  C(  8)       273
        let camel_exchange_id_ton:u8 ;                               //  C(  1)       281
        let camel_exchange_id:u8 ;                                   //  C(  9)       282
        let npdb_query_status:u8 ;                                   //  C(  1)       291
        let scp_connection:u8 ;                                      //  C(  1)       292
        let number_of_all_in_records:u8 ;                            //BCD(  1)       293
        let loc_routing_number:u8 ;                                  //  C( 12)       294
        let loc_routing_number_ton:u8 ;                              //  C(  1)       306
        let add_routing_category:u8 ;                                //  W(  1)       307
        let out_bnc_connection_type:u8 ;                             //  C(  1)       309
        let outside_user_plane_index:u8 ;                            //BCD(  2)       310
        let outside_control_plane_index:u8 ;                         //BCD(  2)       312
        let emergency_call_category:u8 ;                             //  C(  1)       314
        let rate_adaption:u8 ;                                       //  C(  1)       315
        let ms_classmark3:u8 ;                                       //  C(  1)       316
        let collect_call_indicator:u8 ;                              //  C(  1)       317
        let calling_subs_last_ex_id_ton:u8 ;                         //  C(  1)       318
        let called_subs_last_ex_id_ton:u8 ;                          //  C(  1)       319
        let radio_network_type:u8 ;                                  //  C(  1)       320
        let used_air_interface_user_rate:u8 ;                        //  ;  1)       321
     }      
     Moc {
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
        req_user_initiated_mod_ind,s
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