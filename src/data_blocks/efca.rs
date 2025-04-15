// FORMAT TYPE:      23
// MESSAGE NUMBER:   efca
// FORMAT TYPE NAME: SMMF
// RECORD LENGTH:    151

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
// called_number_ton                               C(  1)        41
// called_number                                   C( 12)        42
// called_category                                 C(  1)        54
// sms_centre                                      C( 10)        55
// incoming_time                                   C(  7)        65
// delivery_time                                   C(  7)        72
// cause_for_termination                          DW(  1)        79
// basic_service_type                              C(  1)        83
// basic_service_code                              C(  1)        84
// msc_type                                        C(  1)        85
// calling_number_ton                              C(  1)        86
// calling_number                                  C( 11)        87
// called_vmsc_number                              C( 10)        98
// sms_type                                        C(  1)       108
// sms_length                                      C(  1)       109
// number_of_in_records                          BCD(  1)       110
// forwarded_to_number_ton                         C(  1)       111
// forwarded_to_number                             C( 12)       112
// forwarded_to_smsc                               C( 12)       124
// num_of_concatenated_sms                         C(  1)       136
// concatenated_record_number                      C(  1)       137
// concatenated_sms_reference                      W(  1)       138
// routing_category                                C(  1)       140
// application_info                                C(  1)       141
// add_routing_category                            W(  1)       142
// call_reference_time                             C(  7)       144
                                                                                                                            
