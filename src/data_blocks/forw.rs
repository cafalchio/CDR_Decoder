
// FORMAT TYPE:      3
// MESSAGE NUMBER:   dd96
// FORMAT TYPE NAME: FORW
// RECORD LENGTH:    301

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
               