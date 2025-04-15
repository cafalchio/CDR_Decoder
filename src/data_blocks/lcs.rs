// FORMAT TYPE:      27
// MESSAGE NUMBER:   e20d
// FORMAT TYPE NAME: LCS
// RECORD LENGTH:    164

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

// served_imsi                                     C(  8)        25
// served_imei                                     C(  8)        33
// service_time                                    C(  7)        41
// served_number_ton                               C(  1)        48
// served_number                                   C( 10)        49
// served_subs_lac                                 W(  1)        59
// served_subs_ci                                  W(  1)        61
// location_estimate                               C( 20)        63
// location_request_type                           C(  1)        83
// cause_for_termination                          DW(  1)        84
// used_position_method                            C(  8)        88
// response_time                                   C(  1)        96
// vertical_accuracy                               C(  1)        97
// horizontal_accuracy                             C(  1)        98
// gmlc_address                                    C(  9)        99
// subs_roaming_status                             C(  1)       108
// gps_data_length                                 C(  1)       109
// gps_data                                        C( 38)       110
// client_external_id                              C( 12)       148
// client_external_id_ton                          C(  1)       160
// age_of_estimate                                 W(  1)       161
// radio_network_type                              C(  1)       163
