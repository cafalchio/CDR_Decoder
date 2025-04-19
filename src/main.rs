mod datatypes;
use cdr_decoder::{
    core::process_file::*,
    data_blocks::header,
    datatypes::{
        charging_fields_impl::decode_bcds,
        primitives::{BCDWord, HByte, BCD},
    },
};
use datatypes::primitives::HWord;
use std::cmp;
use std::time::Instant;

fn read_last_blocks(bytes: &[u8], start_pointer: usize, offset: usize, max_blocks: u32) -> u32 {
    let mut next_header = start_pointer;
    let mut blocks_counter: u32 = 0;
    let mut inis = 0;

    while next_header < bytes.len() {
        let header = extract_header(&bytes[next_header..]);

        if header.record_type == "not found" || header.record_length == 0 {
            return blocks_counter;
        }

        if header.record_type == "Intelligent network data 1" {
            inis += 1;
            if inis > 1 {
                return blocks_counter;
            }
            next_header += offset;
        } else {
            next_header += header.record_length as usize;
        }
        blocks_counter += 1;

        if header.record_type == "Trailer" && blocks_counter >= max_blocks {
            return blocks_counter;
        }
    }

    blocks_counter
}

fn main() {
    println!("Running extraction...");
    let start_time = Instant::now();

    let bytes = read_file("data/VL_GNK_MSSDF5_T20250115111349_22243_N_00000.BACKUP.gz");
    // let bytes = read_file("data/VL_GNK_MSSDF5_T20250115111432_10650_N_00000.BACKUP.gz");
    let mut next_header = 0;
    let mut cnt = 0;
    let mut last_intelligent = 0;
    let mut max_blocks = 0;

    while next_header < bytes.len() {
        cnt += 1;

        // Ensure there's enough data left for header extraction
        let header = extract_header(&bytes[next_header..]);

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
            println!("Trying to recover from corrupted or unknown block...");
            next_header = last_intelligent;

            let mut skip = 0;
            for offset in 20..cmp::min(300, bytes.len() - next_header) {
                let nblocks = read_last_blocks(&bytes, next_header, offset, max_blocks);
                if nblocks > max_blocks {
                    max_blocks = nblocks;
                    skip = offset;
                    // println!("Offset {} -> new max_blocks: {} at block {}", offset, max_blocks, cnt);
                }
            }

            if skip == 0 && (bytes.len() - next_header > 65) {
                next_header += 65;
                continue;
            }

            next_header += skip;
        }
    }

    println!("Ran {} blocks in {:.2?}", cnt, Instant::now() - start_time);
    println!(
        "Bytes left: {} bytes",
        bytes.len().saturating_sub(next_header)
    );
}
