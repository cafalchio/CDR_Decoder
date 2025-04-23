// FORMAT TYPE:      24
// MESSAGE NUMBER:   dd8c
// FORMAT TYPE NAME: COC Camel-originated Call
// RECORD LENGTH:    135

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
// in_channel_allocated_time                       C(  7)        27
// leg_call_reference                              C(  5)        34
// intermediate_chrg_cause                         C(  2)        39
// camel_call_reference                            C(  8)        41
// camel_exchange_id_ton                           C(  1)        49
// camel_exchange_id                               C(  9)        50
// charging_start_time                             C(  7)        59
// charging_end_time                               C(  7)        66
// duration_before_answer                        BCD(  3)        73
// chargeable_duration                           BCD(  3)        76
// basic_call_state_model                          C(  1)        79
// scf_address_ton                                 C(  1)        80
// scf_address                                     C(  9)        81
// camel_service_key                              DW(  1)        90
// default_call_handling                           C(  1)        94
// destination_number_ton                          C(  1)        95
// destination_number                              C( 12)        96
// level_of_camel_service                          C(  1)       108
// camel_modification                              C(  4)       109
// camel_modify_parameters                         C( 14)       113
// number_of_in_records                          BCD(  1)       127
// call_reference_time                             C(  7)       128
