pub struct Moc {
    intermediate_record_number: u8 ,                   //BCD(  1)        25
    intermediate_charging_ind: u8 ,                    //  C(  1)        26 IntermediateChargingInd
    number_of_ss_records: u8 ,                         //BCD(  1)        27
    calling_imsi:u8 ,                                  //  C(  8)        28
    calling_imei:u8 ,                                  //  C(  8)        36
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
}//
    /*:u8 ,FORMAT TYPE:      1//
    FORMAT TYPE:      1//
    MESSAGE NUMBER:   dd98//
    FORMAT TYPE NAME: MOC//
    RECORD LENGTH:    322//
//
    HEADER:
    FIELD NAME                                   DATA TYPE  POSITION

    record_length                                   W(  1)         0
    record_type                                   BCD(  1)         2
    record_number                                 BCD(  4)         3
    record_status                                   C(  1)         7
    check_sum                                       W(  1)         8
    call_reference                                  C(  5)        10
    exchange_id                                     C( 10)        15
                                                                                                                                
                                                                                                                
    DATA:
    FIELD NAME                                   DATA TYPE  POSITION

    intermediate_record_number                    BCD(  1)        25
    intermediate_charging_ind                       C(  1)        26
    number_of_ss_records                          BCD(  1)        27
    calling_imsi                                    C(  8)        28
    calling_imei                                    C(  8)        36
    calling_number                                  C( 10)        44
    calling_category                                C(  1)        54
    calling_ms_classmark                            C(  1)        55
    called_imsi                                     C(  8)        56
    called_imei                                     C(  8)        64
    called_number_ton                               C(  1)        72
    called_number                                   C( 12)        73
    called_ms_classmark                             C(  1)        85
    dialled_digits_ton                              C(  1)        86
    dialled_digits                                  C( 12)        87
    calling_subs_first_lac                          W(  1)        99
    calling_subs_first_ci                           W(  1)       101
    calling_subs_last_ex_id                         C( 10)       103
    calling_subs_last_lac                           W(  1)       113
    calling_subs_last_ci                            W(  1)       115
    out_circuit_group                             BCD(  2)       117
    out_circuit                                   BCD(  2)       119
    basic_service_type                              C(  1)       121
    basic_service_code                              C(  1)       122
    facility_usage                                  C(  4)       123
    non_transparency_indicator                      C(  1)       127
    channel_rate_indicator                          C(  1)       128
    in_channel_allocated_time                       C(  7)       129
    charging_start_time                             C(  7)       136
    charging_end_time                               C(  7)       143
    cause_for_termination                          DW(  1)       150
    call_type                                       C(  1)       154
    orig_mcz_chrg_type                              C(  1)       155
    orig_mcz_duration                             BCD(  3)       156
    orig_mcz_tariff_class                         BCD(  3)       159
    orig_mcz_pulses                               BCD(  2)       162
    called_msrn_ton                                 C(  1)       164
    called_msrn                                     C( 12)       165
    calling_number_ton                              C(  1)       177
    intermediate_chrg_cause                         C(  2)       178
    calling_modify_parameters                       C( 14)       180
    orig_mcz_modify_percent                         W(  1)       194
    orig_mcz_modify_direction                       C(  1)       196
    orig_dialling_class                             W(  1)       197
    leg_call_reference                              C(  5)       199
    call_reference_time                             C(  7)       204
    speech_version                                  C(  1)       211
    b_idle_time                                     C(  7)       212
    pni                                             C(  3)       219
    redirected_indicator                            C(  1)       222
    orig_mcz_change_percent                         C(  1)       223
    orig_mcz_change_direction                       C(  1)       224
    calling_charging_area                           W(  1)       225
    called_charging_area                            W(  1)       227
    connected_to_number_ton                         C(  1)       229
    connected_to_number                             C( 12)       230
    cug_interlock                                   C(  4)       242
    cug_outgoing_access                             C(  1)       246
    cug_information                                 C(  1)       247
    hot_billing_record_number                     BCD(  4)       248
    number_of_in_records                          BCD(  1)       252
    regional_subs_indicator                         C(  1)       253
    regional_subs_location_type                     C(  1)       254
    tns_carrier_code                                W(  1)       255
    carrier_selection                               C(  1)       257
    pic                                             W(  1)       258
    routing_category                                C(  1)       260
    called_category                                 C(  1)       261
    calling_cell_band                               C(  1)       262
    req_fixed_nw_user_rate                          C(  1)       263
    req_other_modem_type                            C(  1)       264
    acceptable_channel_codings                      C(  1)       265
    req_number_of_channels                          C(  1)       266
    req_air_interface_user_rate                     C(  1)       267
    req_user_initiated_mod_ind                      C(  1)       268
    used_number_of_channels                         C(  1)       269
    used_other_modem_type                           C(  1)       270
    used_fixed_nw_user_rate                         C(  1)       271
    used_channel_coding                             C(  1)       272
    camel_call_reference                            C(  8)       273
    camel_exchange_id_ton                           C(  1)       281
    camel_exchange_id                               C(  9)       282
    npdb_query_status                               C(  1)       291
    scp_connection                                  C(  1)       292
    number_of_all_in_records                      BCD(  1)       293
    loc_routing_number                              C( 12)       294
    loc_routing_number_ton                          C(  1)       306
    add_routing_category                            W(  1)       307
    out_bnc_connection_type                         C(  1)       309
    outside_user_plane_index                      BCD(  2)       310
    outside_control_plane_index                   BCD(  2)       312
    emergency_call_category                         C(  1)       314
    rate_adaption                                   C(  1)       315
    ms_classmark3                                   C(  1)       316
    collect_call_indicator                          C(  1)       317
    calling_subs_last_ex_id_ton                     C(  1)       318
    called_subs_last_ex_id_ton                      C(  1)       319
    radio_network_type                              C(  1)       320
    used_air_interface_user_rate                    C(  1)       321
    */
    


