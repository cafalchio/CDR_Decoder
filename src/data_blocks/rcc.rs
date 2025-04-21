// FORMAT TYPE:      22
// MESSAGE NUMBER:   dd8d
// FORMAT TYPE NAME: RCC
// RECORD LENGTH:    216

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
// intermediate_chrg_cause                         C(  2)        27
// number_of_ss_records                          BCD(  1)        29
// dialled_digits                                  C( 12)        30
// calling_number_ton                              C(  1)        42
// calling_number                                  C( 10)        43
// called_number_ton                               C(  1)        53
// called_number                                   C( 12)        54
// in_circuit_group                              BCD(  2)        66
// in_circuit                                    BCD(  2)        68
// out_circuit_group                             BCD(  2)        70
// out_circuit                                   BCD(  2)        72
// basic_service_type                              C(  1)        74
// basic_service_code                              C(  1)        75
// facility_usage                                  C(  4)        76
// in_channel_allocated_time                       C(  7)        80
// charging_start_time                             C(  7)        87
// charging_end_time                               C(  7)        94
// cause_for_termination                          DW(  1)       101
// called_msrn_ton                                 C(  1)       105
// called_msrn                                     C( 12)       106
// cug_interlock                                   C(  4)       118
// cug_outgoing_access                             C(  1)       122
// cug_information                                 C(  1)       123
// number_of_in_records                          BCD(  1)       124
// leg_call_reference                              C(  5)       125
// orig_mcz_chrg_type                              C(  1)       130
// orig_mcz_duration                             BCD(  3)       131
// orig_mcz_tariff_class                         BCD(  3)       134
// orig_mcz_pulses                               BCD(  2)       137
// orig_mcz_change_percent                         C(  1)       139
// orig_mcz_change_direction                       C(  1)       140
// tns_carrier_code                                W(  1)       141
// carrier_selection                               C(  1)       143
// pic                                             W(  1)       144
// dialled_digits_ton                              C(  1)       146
// calling_category                                C(  1)       147
// called_ms_classmark                             C(  1)       148
// calling_charging_area                           W(  1)       149
// called_charging_area                            W(  1)       151
// pni                                             C(  3)       153
// called_imsi                                     C(  8)       156
// called_imei                                     C(  8)       164
// connected_to_number_ton                         C(  1)       172
// connected_to_number                             C( 12)       173
// scp_connection                                  C(  1)       185
// number_of_all_in_records                      BCD(  1)       186
// call_reference_time                             C(  7)       187
// in_bnc_connection_type                          C(  1)       194
// inside_user_plane_index                       BCD(  2)       195
// inside_control_plane_index                    BCD(  2)       197
// out_bnc_connection_type                         C(  1)       199
// outside_user_plane_index                      BCD(  2)       200
// outside_control_plane_index                   BCD(  2)       202
// redirecting_number                              C( 12)       204
