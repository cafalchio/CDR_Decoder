mod datatypes;
use cdr_decoder::{core::process_file::*, data_blocks::header};
use std::{str::FromStr, time::Instant};
use strum_macros::{Display, EnumString, FromRepr};

// Updated trait that directly converts u8 to string

pub struct IntermediateChargingInd(&'static str);

impl IntermediateChargingInd {
    pub fn new(value: u8) -> Self {
        Self(match value {
            0 => "Normal",
            1 => "Intermediate",
            2 => "Last Partial",
            3 => "NotUsed",
            _ => "Unknown",
        })
    }

    pub fn value(&self) -> &str {
        self.0
    }
}

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
    println!("{}", IntermediateChargingInd::new(0x02).value());
    // read_multiple_files("/home/cafalchio/projects/cdr_decoder/data");
    let before = Instant::now();
    // println!("Running extraction");
    // let bytes = read_file("data/VL_GNK_MSSDF5_T20250115111404_22245_N_00000.BACKUP.gz");
    // let mut next_header: usize = 0;
    // let mut counter = 0;
    // loop {
    //     let header = extract_header(&bytes[next_header..]);
    //     // println!("rec lenght: {}", header.record_length);
    //     // println!("record type: {} lenght: {}", header.record_type, header.record_length);
    //     // println!("checksum: {}", header.check_sum);
    //     // println!("call ref: {}", header.call_reference);
    //     // println!("exchange id: {}", header.exchange_id);
    //     // println!("status: {}", header.record_status);
    //     next_header += header.record_length as usize;
    //     counter += 1;
    //     if header.record_type == "Trailer".to_string() {
    //         break;
    //     }

    // }
    // println!("Number of Headers: {}", counter);
    println!("Elapsed time: {:.2?}", before.elapsed());
}
