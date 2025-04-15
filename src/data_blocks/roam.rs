
// FORMAT TYPE:      4
// MESSAGE NUMBER:   dd95
// FORMAT TYPE NAME: ROAM
// RECORD LENGTH:    219

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
                      