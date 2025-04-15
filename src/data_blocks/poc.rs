// FORMAT TYPE:      11
// MESSAGE NUMBER:   dd94
// FORMAT TYPE NAME: POC
// RECORD LENGTH:    200

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
// calling_number_ton                              C(  1)        28
// calling_number                                  C( 12)        29
// called_number_ton                               C(  1)        41
// called_number                                   C( 12)        42
// in_circuit_group                              BCD(  2)        54
// in_circuit                                    BCD(  2)        56
// in_channel_allocated_time                       C(  7)        58
// charging_start_time                             C(  7)        65
// charging_end_time                               C(  7)        72
// cause_for_termination                          DW(  1)        79
// call_type                                       C(  1)        83
// ticket_type                                     C(  1)        84
// iaz_chrg_type                                   C(  1)        85
// iaz_duration                                  BCD(  3)        86
// iaz_tariff_class                              BCD(  3)        89
// iaz_pulses                                    BCD(  2)        92
// called_msrn_ton                                 C(  1)        94
// called_msrn                                     C( 12)        95
// intermediate_chrg_cause                         C(  2)       107
// orig_dialling_class                             W(  1)       109
// leg_call_reference                              C(  5)       111
// basic_service_type                              C(  1)       116
// basic_service_code                              C(  1)       117
// call_reference_time                             C(  7)       118
// number_of_in_records                          BCD(  1)       125
// b_idle_time                                     C(  7)       126
// redirected_indicator                            C(  1)       133
// loc_routing_number                              C( 12)       134
// npdb_query_status                               C(  1)       146
// scp_connection                                  C(  1)       147
// number_of_all_in_records                      BCD(  1)       148
// loc_routing_number_ton                          C(  1)       149
// camel_call_reference                            C(  8)       150
// camel_exchange_id_ton                           C(  1)       158
// camel_exchange_id                               C(  9)       159
// in_bnc_connection_type                          C(  1)       168
// inside_user_plane_index                       BCD(  2)       169
// inside_control_plane_index                    BCD(  2)       171
// collect_call_indicator                          C(  1)       173
// redirecting_number                              C( 12)       174
// dialled_digits                                  C( 12)       186
// rate_adaption                                   C(  1)       198
// calling_pstn_category                           C(  1)       199
                                                                                                                            
