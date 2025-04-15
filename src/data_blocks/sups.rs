// FORMAT TYPE:      5
// MESSAGE NUMBER:   efcd
// FORMAT TYPE NAME: SUPS
// RECORD LENGTH:    157

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

// ss_record_number                              BCD(  1)        25
// served_imsi                                     C(  8)        26
// served_imei                                     C(  8)        34
// served_number                                   C( 10)        42
// served_subs_lac                                 W(  1)        52
// served_subs_ci                                  W(  1)        54
// supplementary_service_code                      C(  1)        56
// action                                          C(  1)        57
// in_channel_allocated_time                       C(  7)        58
// charging_time                                   C(  7)        65
// cause_for_termination                          DW(  1)        72
// result_indicator                                W(  1)        76
// parameters_length                               C(  1)        78
// parameters                                      C( 30)        79
// served_number_ton                               C(  1)       109
// served_ms_classmark                             C(  1)       110
// basic_service_type                              C(  1)       111
// basic_service_code                              C(  1)       112
// leg_call_reference                              C(  5)       113
// call_reference_time                             C(  7)       118
// pni                                             C(  3)       125
// hot_billing_record_number                     BCD(  4)       128
// routing_category                                C(  1)       132
// ms_classmark3                                   C(  1)       133
// served_cell_band                                C(  1)       134
// basic_call_state_model                          C(  1)       135
// camel_call_reference                            C(  8)       136
// camel_exchange_id_ton                           C(  1)       144
// camel_exchange_id                               C(  9)       145
// add_routing_category                            W(  1)       154
// radio_network_type                              C(  1)       156
                                                                                                                            
