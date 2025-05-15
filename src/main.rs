mod datatypes;
use cdr_decoder::core::process_file::*;
use cdr_decoder::data_blocks::blocks;
use cdr_decoder::database::db::*;
use cdr_decoder::database::util::{insert_cdr_blocks, insert_cdr_file};
use colored::Colorize;
use std::cmp;
use std::time::Instant;
use cdr_decoder::database::db::NewCdrBlock;


fn main() {
    let filename = "VL_GNK_MSSDF5_T20250115111403_10647_N_00000";

    // Add file to db
    let mut connection: diesel::PgConnection = establish_connection();
    let inserted_file = insert_cdr_file(&mut connection, filename).unwrap();

    println!("Running extraction...");
    let start_time = Instant::now();
    let bytes = read_file(
        "/home/cafalchio/Downloads/VL_GNK_MSSDF5_T20250115111403_10647_N_00000.BACKUP.gz",
    );

    let mut new_blocks: Vec<NewCdrBlock> = vec![]; 
    let mut next_header = 0;
    let mut last_trailer = 0;
    let mut max_blocks = 0;
    let mut count_blocks = 0;

    while next_header < bytes.len() {

        let header = extract_header(&bytes[next_header..]);

        if header.record_type.starts_with("not found") || header.record_length == 0 {
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

        if header.record_type == "Trailer" {
            count_blocks += 1;
        }

        match blocks::Blocks::new(
            &header.record_type,
            &bytes[next_header..next_header + header.record_length as usize],
        ) {
            Some(block) => {
                // let json_str: String = block.to_json_str().unwrap();
                let json = block.to_json().unwrap();
                // Print all blocks to console
                // println!("{}", json);

                let new_block = NewCdrBlock {
                    file_id: inserted_file.id,
                    block_type: header.record_type.clone(),
                    block_index: count_blocks,
                    parsed_data: Some(json),
                };
                new_blocks.push(new_block.clone());

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
    println!("Saving in db");
    insert_cdr_blocks(&mut connection, new_blocks);
    println!("{}", "\n----- Summary -----".green());
    println!("Ran {} blocks in {:.2?}", count_blocks, Instant::now() - start_time);
    println!(
        "Bytes left: {} bytes",
        bytes.len().saturating_sub(next_header)
    );
}
