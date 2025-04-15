// FORMAT TYPE:      8
// MESSAGE NUMBER:   efc9
// FORMAT TYPE NAME: SMMO
// RECORD LENGTH:    150

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

// calling_imsi                                    C(  8)        25
// calling_imei                                    C(  8)        33
// calling_number                                  C( 10)        41
// calling_category                                C(  1)        51
// calling_ms_classmark                            C(  1)        52
// sms_centre                                      C( 10)        53
// calling_subs_lac                                W(  1)        63
// calling_subs_ci                                 W(  1)        65
// incoming_time                                   C(  7)        67
// cause_for_termination                          DW(  1)        74
// msc_type                                        C(  1)        78
// called_number                                   C( 12)        79
// calling_number_ton                              C(  1)        91
// called_number_ton                               C(  1)        92
// calling_vmsc_number                             C( 10)        93
// sms_type                                        C(  1)       103
// dialled_digits                                  C( 12)       104
// call_reference_time                             C(  7)       116
// tariff_class                                  BCD(  3)       123
// pni                                             C(  3)       126
// hot_billing_record_number                     BCD(  4)       129
// sms_length                                      C(  1)       133
// command_type                                    C(  1)       134
// message_reference                               C(  1)       135
// number_of_in_records                          BCD(  1)       136
// num_of_concatenated_sms                         C(  1)       137
// concatenated_record_number                      C(  1)       138
// concatenated_sms_reference                      W(  1)       139
// routing_category                                C(  1)       141
// application_info                                C(  1)       142
// dialled_digits_ton                              C(  1)       143
// default_sms_handling                            C(  1)       144
// camel_sms_modification                          C(  2)       145
// add_routing_category                            W(  1)       147
// radio_network_type                              C(  1)       149
                                                                                                                            
