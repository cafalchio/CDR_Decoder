// FORMAT TYPE:      12
// MESSAGE NUMBER:   dd93
// FORMAT TYPE NAME: PTC
// RECORD LENGTH:    189

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
// out_circuit_group                             BCD(  2)        54
// out_circuit                                   BCD(  2)        56
// in_channel_allocated_time                       C(  7)        58
// charging_start_time                             C(  7)        65
// charging_end_time                               C(  7)        72
// cause_for_termination                          DW(  1)        79
// call_type                                       C(  1)        83
// ticket_type                                     C(  1)        84
// oaz_chrg_type                                   C(  1)        85
// oaz_duration                                  BCD(  3)        86
// oaz_tariff_class                              BCD(  3)        89
// oaz_pulses                                    BCD(  2)        92
// called_msrn_ton                                 C(  1)        94
// called_msrn                                     C( 12)        95
// intermediate_chrg_cause                         C(  2)       107
// leg_call_reference                              C(  5)       109
// out_channel_allocated_time                      C(  7)       114
// basic_service_type                              C(  1)       121
// basic_service_code                              C(  1)       122
// call_reference_time                             C(  7)       123
// b_idle_time                                     C(  7)       130
// redirected_indicator                            C(  1)       137
// number_of_in_records                          BCD(  1)       138
// tns_carrier_code                                W(  1)       139
// carrier_selection                               C(  1)       141
// npdb_query_status                               C(  1)       142
// loc_routing_number                              C( 12)       143
// scp_connection                                  C(  1)       155
// number_of_all_in_records                      BCD(  1)       156
// loc_routing_number_ton                          C(  1)       157
// out_bnc_connection_type                         C(  1)       158
// outside_user_plane_index                      BCD(  2)       159
// outside_control_plane_index                   BCD(  2)       161
// collect_call_indicator                          C(  1)       163
// outpulsed_number                                C( 12)       164
// redirecting_number                              C( 12)       176
// rate_adaption                                   C(  1)       188
                                                                                                                            
