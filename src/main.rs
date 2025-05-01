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
    let bytes = read_file("data/test_file1.gz");

    let mut next_header = 0;
    let mut cnt = 0;
    let mut last_block = 0;
    let mut max_blocks = 0;

    while next_header < bytes.len() {
        cnt += 1;

        let header = extract_header(&bytes[next_header..]);
        all_types.push(header.record_type.clone());
        println!("{} ---> {}", header.record_type, header.record_length);
        if header.record_length == 0 {
            break;
        }

        match blocks::Blocks::new(
            &header.record_type,
            &bytes[next_header..next_header + header.record_length as usize],
        ) {
            Some(block) => {
                let json = block.to_json().unwrap();
                // println!("{}", json);
            }
            None => {
                println!("Error trying to read {}", header.record_type);
            }
        }


        if header.record_type == "not found" || header.record_length == 0 {
            println!("Trying to recover from corrupted or unknown block...");
            let mut skip = 0;
            for offset in 20..cmp::min(400, bytes.len() - next_header) {
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
        next_header += header.record_length as usize;
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



// fn main() {
//     let input_path = "data/test_file1.gz";

//     let bytes = read_file(input_path);
//     let mut position = 0usize;
//     let mut block_index = 0;

//     println!("Block_Index;Block_Type;Block_Length;Position");

//     while position + 3 <= bytes.len() {
//         // Read 2 bytes for block length (little-endian)
//         let b1 = bytes[position] as u16;
//         let b2 = bytes[position + 1] as u16;
//         let block_length = ((b2 << 8) | b1) as usize;

//         // Read 1 byte for block type
//         let type_byte = bytes[position + 2];
//         let record_type = RecordType::new(type_byte).value().to_string();

//         // Print block info
//         block_index += 1;
//         println!(
//             "{};{};{};{}",
//             block_index, record_type, block_length, position
//         );

//         // Move to the next block
//         if block_length == 0 {
//             break;
//         }

//         position += block_length;
//     }

//     println!("Processed {} blocks", block_index);
// }