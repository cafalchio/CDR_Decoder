#![allow(dead_code)]
#![allow(unused_variables)]

mod datatypes;
use cdr_decoder::core::process_file::*;
use cdr_decoder::data_blocks::blocks;
use std::cmp;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let mut all_types: Vec<String> = Vec::new();

    println!("Running extraction...");
    let start_time = Instant::now();
    let bytes = read_file("data/VL_GNK_MSSDF5_T20250115111404_22245_N_00000.BACKUP.gz");
    // let bytes = read_file("data/VL_GNK_MSSDF5_T20250115111404_22245_N_00000.BACKUP.gz");

    let mut next_header = 0;
    let mut cnt = 0;
    let mut last_intelligent = 0;
    let mut max_blocks = 0;

    while next_header < bytes.len() {
        cnt += 1;

        let header = extract_header(&bytes[next_header..]);
        all_types.push(header.record_type.clone());

        match blocks::Blocks::new(
            &header.record_type,
            &bytes[next_header..next_header + header.record_length as usize],
        ) {
            Some(block) => {
                let json = block.to_json().unwrap();
                // println!("{}", json);
            }
            None => {
                // handle unknown record type if needed
            }
        }

        match header.record_type.as_str() {
            "Intelligent network data 1" => {
                last_intelligent = next_header;
                let mut ff_ref = header.record_length as usize + 1;
                while next_header + ff_ref + 1 <= bytes.len() && bytes[next_header + ff_ref] == 0xFF
                {
                    ff_ref += 1;
                }
                // Check for OOB
                if next_header + ff_ref > bytes.len() {
                    // println!("Preventing OOB at offset {}", next_header + ff_ref);
                    break;
                }
                next_header += ff_ref;
            }

            "Trailer" => {
                println!("END OF FILE at offset {}", next_header);
                break;
            }
            _ => {
                let advance = header.record_length as usize;
                if next_header + advance > bytes.len() {
                    println!(
                        "Advance of {} from {} would overflow buffer of size {}",
                        advance,
                        next_header,
                        bytes.len()
                    );
                    break;
                }
                next_header += advance;
            }
        }

        // Handle unknown or bad headers
        if header.record_type == "not found" || header.record_length == 0 {
            // println!("Trying to recover from corrupted or unknown block...");
            next_header = last_intelligent;

            let mut skip = 0;
            for offset in 20..cmp::min(200, bytes.len() - next_header) {
                let nblocks = read_last_blocks(&bytes, next_header, offset, max_blocks);
                if nblocks > max_blocks {
                    max_blocks = nblocks;
                    skip = offset;
                }
            }

            if skip == 0 && (bytes.len() - next_header > 65) {
                next_header += 65;
                continue;
            }

            next_header += skip;
        }
    }
    let mut m: HashMap<String, usize> = HashMap::new();
    for x in all_types {
        *m.entry(x).or_default() += 1;
    }
    println!("Ran {} blocks in {:.2?}", cnt, Instant::now() - start_time);
    println!(
        "Bytes left: {} bytes",
        bytes.len().saturating_sub(next_header)
    );
    println!("---------What to implement first----------");
    for (key, value) in m.into_iter() {
        println!("{} -> {}", key, value);
    }
}
