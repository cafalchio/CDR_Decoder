mod datatypes;
use cdr_decoder::{core::process_file::*, data_blocks::header};
use std::{str::FromStr, time::Instant};
use strum_macros::{Display, EnumString, FromRepr};

// Updated trait that directly converts u8 to string



fn read_headers(bytes: &[u8]) {
    let before = Instant::now();
    println!("Running extraction");
    let bytes = read_file("data/VL_GNK_MSSDF5_T20250115111404_22245_N_00000.BACKUP.gz");
    let mut next_header: usize = 0;
    let mut counter = 0;
    loop {
        let header = extract_header(&bytes[next_header..]);
        println!("rec lenght: {}", header.record_length);
        println!(
            "record type: {} lenght: {}",
            header.record_type, header.record_length
        );
        println!("checksum: {}", header.check_sum);
        println!("call ref: {}", header.call_reference);
        println!("exchange id: {}", header.exchange_id);
        println!("status: {}", header.record_status);
        next_header += header.record_length as usize;
        counter += 1;
        if header.record_type == "Trailer".to_string() {
            break;
        }
    }
    println!("Number of Headers: {}", counter);
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn main() {
    // read_multiple_files("/home/cafalchio/projects/cdr_decoder/data");
    let before = Instant::now();
    println!("Running extraction");
    let bytes = read_file("data/VL_GNK_MSSDF5_T20250115111423_22247_N_00000.BACKUP.gz");
    let mut next_header: usize = 0;
    let mut counter = 0;
    loop {
        let header = extract_header(&bytes[next_header..]);
        println!("record type: {} lenght: {}", header.record_type, header.record_length);

        // println!("checksum: {}", header.check_sum);
        // println!("call ref: {}", header.call_reference);
        // println!("exchange id: {}", header.exchange_id);
        // println!("status: {}", header.record_status);
        if header.record_length == 0 {
            break;
        }
        next_header += header.record_length as usize;
        counter += 1;
        if header.record_type == "Trailer".to_string() {
            println!("record type: {} lenght: {}", header.record_type, header.record_length);
            break;
        }


    }
    println!("Number of Headers: {}", counter);
    println!("Elapsed time: {:.2?}", before.elapsed());
}
