// FORMAT TYPE:      29
// MESSAGE NUMBER:   efc5
// FORMAT TYPE NAME: USSD
// RECORD LENGTH:    88

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
// served_number_ton                               C(  1)        41
// served_number                                   C( 10)        42
// initiator                                       C(  1)        52
// action                                          C(  1)        53
// start_time                                      C(  7)        54
// end_time                                        C(  7)        61
// cause_for_termination                          DW(  1)        68
// served_subs_lac                                 W(  1)        72
// served_subs_ci                                  W(  1)        74
// number_of_transactions                        BCD(  1)        76
// service_code                                    C( 10)        77
// radio_network_type                              C(  1)        87
    