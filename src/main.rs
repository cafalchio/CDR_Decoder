#![allow(dead_code)]
#![allow(unused_variables)]

mod datatypes;
use cdr_decoder::core::process_file::*;
use cdr_decoder::data_blocks::blocks;
use colored::Colorize;
use std::cmp;
use std::collections::HashMap;
use std::time::Instant;

// This function is Work in progress, needs fix
fn skip_error_blocks(bytes: &[u8], mut curr_position: usize) -> usize {
    // When we reach an unknown or block with lenght 0, we assume the block is
    // corrupted (or not implemented as a block type)
    // This function loops from current_position + (24 to 254)
    // for each new position, we read the number of blocks we can get without an error
    // the biggest number of blocks skipped will be considered the correct one and the skip
    println!("Trying to recover from corrupted or unknown block...");
    let mut max_blocks_reached = 0;
    let mut skip = 0;
    for offset in 24..cmp::min(254, bytes.len() - curr_position) {
        let nblocks = read_last_blocks(&bytes, curr_position, offset, max_blocks_reached);
        if nblocks > max_blocks_reached {
            max_blocks_reached = nblocks;
            skip = offset;
        }
    }
    if skip == 0 && (bytes.len() - curr_position > 65) {
        curr_position += 65;
    }
    curr_position
}

fn main() {
    let mut all_types: Vec<String> = Vec::new();

    println!("Running extraction...");
    let start_time = Instant::now();
    let bytes = read_file("data/test_file1.gz");
    let mut next_header = 0;
    let mut cnt = 0;
    let mut last_trailer = 0;
    let mut max_blocks = 0;

    while next_header < bytes.len() {
        cnt += 1;

        let header = extract_header(&bytes[next_header..]);
        all_types.push(header.record_type.clone());

        if header.record_type.starts_with("not found") || header.record_length == 0 {
            // TODO: fix skip_error_blocks function to use here.
            println!(
                "Trying to recover from corrupted or unknown block @{}",
                next_header
            );
            next_header = last_trailer;

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
            continue;
        }

        match blocks::Blocks::new(
            &header.record_type,
            &bytes[next_header..next_header + header.record_length as usize],
        ) {
            Some(block) => {
                let json = block.to_json().unwrap();
                // Print all blocks to console
                // println!("{}", json);
            }
            None => {}
        }

        match header.record_type.as_str() {
            "Trailer" => {
                last_trailer = next_header;
                next_header =
                    skip_trailer_bytes(&bytes, next_header + header.record_length as usize);
                continue;
            }
            _ => {
                next_header += header.record_length as usize;
                continue;
            }
        }
    }
    let mut m: HashMap<String, usize> = HashMap::new();
    for x in all_types {
        *m.entry(x).or_default() += 1;
    }

    // Collect into a vector of (&String, &usize) and sort
    let mut count_vec: Vec<(&String, &usize)> = m.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("{}", "\n----- Summary -----".green());
    println!("Ran {} blocks in {:.2?}", cnt, Instant::now() - start_time);
    println!(
        "Bytes left: {} bytes",
        bytes.len().saturating_sub(next_header)
    );
    println!("{}", "\n----- Counts -----".green());
    for (key, value) in count_vec {
        println!("{} -> {}", key, value);
    }
}
