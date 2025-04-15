// FORMAT TYPE:      26
// MESSAGE NUMBER:   dd8a
// FORMAT TYPE NAME: IN4
// RECORD LENGTH:    108

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

// in_record_number                              BCD(  1)        25
// in_data                                         C( 40)        26
// leg_call_reference                              C(  5)        66
// in_channel_allocated_time                       C(  7)        71
// in_data_length                                  W(  1)        78
// basic_call_state_model                          C(  1)        80
// party_to_charge                                 C(  1)        81
// protocol_identification                         C(  1)        82
// call_reference_time                             C(  7)        83
// camel_call_reference                            C(  8)        90
// camel_exchange_id_ton                           C(  1)        98
// camel_exchange_id                               C(  9)        99
                                                                                                                            

// FORMAT TYPE:      28
// MESSAGE NUMBER:   dd8a
// FORMAT TYPE NAME: IN5
// RECORD LENGTH:    228

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

// in_record_number                              BCD(  1)        25
// in_data                                         C(160)        26
// leg_call_reference                              C(  5)       186
// in_channel_allocated_time                       C(  7)       191
// in_data_length                                  W(  1)       198
// camel_call_reference                            C(  8)       200
// camel_exchange_id_ton                           C(  1)       208
// camel_exchange_id                               C(  9)       209
// basic_call_state_model                          C(  1)       218
// party_to_charge                                 C(  1)       219
// protocol_identification                         C(  1)       220
// call_reference_time                             C(  7)       221
                                                                                                                            
