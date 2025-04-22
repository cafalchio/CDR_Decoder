// FORMAT TYPE:      9
// MESSAGE NUMBER:   efc8
// FORMAT TYPE NAME: SMMT
// RECORD LENGTH:    140

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

// called_imsi                                     C(  8)        25
// called_imei                                     C(  8)        33
// called_number                                   C( 12)        41
// called_category                                 C(  1)        53
// called_ms_classmark                             C(  1)        54
// called_subs_lac                                 W(  1)        55
// called_subs_ci                                  W(  1)        57
// sms_centre                                      C( 10)        59
// incoming_time                                   C(  7)        69
// cause_for_termination                          DW(  1)        76
// msc_type                                        C(  1)        80
// calling_number                                  C( 11)        81
// called_vmsc_number                              C( 10)        92
// calling_number_ton                              C(  1)       102
// called_number_ton                               C(  1)       103
// sms_type                                        C(  1)       104
// call_reference_time                             C(  7)       105
// hot_billing_record_number                     BCD(  4)       112
// tariff_class                                  BCD(  3)       116
// calling_vmsc_number                             C( 10)       119
// sms_length                                      C(  1)       129
// number_of_in_records                          BCD(  1)       130
// num_of_concatenated_sms                         C(  1)       131
// concatenated_record_number                      C(  1)       132
// concatenated_sms_reference                      W(  1)       133
// application_info                                C(  1)       135
// routing_category                                C(  1)       136
// add_routing_category                            W(  1)       137
// radio_network_type                              C(  1)       139
                                                                                                                            
