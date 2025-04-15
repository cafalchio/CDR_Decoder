// FORMAT TYPE:      6
// MESSAGE NUMBER:   efcb
// FORMAT TYPE NAME: HLRI
// RECORD LENGTH:    67

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
// called_number                                   C( 10)        33
// routing_number                                  C( 12)        43
// charging_time                                   C(  7)        55
// number_of_forwardings                           C(  1)        62
// cause_for_termination                          DW(  1)        63
                                                                                                                            
