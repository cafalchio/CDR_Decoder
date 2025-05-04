use pyo3::prelude::*;
pub mod core;
pub mod data_blocks;
pub mod datatypes;
use core::process_file::*;
use data_blocks::blocks;
use std::cmp;
use std::collections::HashMap;
use std::time::Instant;

fn format_summary(
    cnt: usize,
    start_time: Instant,
    bytes: &[u8],
    next_header: usize,
    m: HashMap<String, usize>,
) -> String {
    let mut output = String::new();

    output += &format!("\n----- Summary -----\n");
    output += &format!(
        "Ran {} blocks in {:.2?}\n",
        cnt,
        Instant::now() - start_time
    );
    output += &format!(
        "Bytes left: {} bytes\n",
        bytes.len().saturating_sub(next_header)
    );
    output += &format!("\n----- Counts -----\n");

    for (key, value) in m {
        output += &format!("{} -> {}\n", key, value);
    }

    output
}

pub fn test_cdr_extraction(file: String) -> String {
    let mut all_types: Vec<String> = Vec::new();
    println!("Running extraction...");
    let start_time = Instant::now();
    let bytes = read_file(&file);

    let mut next_header = 0;
    let mut cnt = 0;
    let mut last_trailer = 0;
    let mut max_blocks = 0;

    while next_header < bytes.len() {
        cnt += 1;

        let header = extract_header(&bytes[next_header..]);
        all_types.push(header.record_type.clone());

        if header.record_type.starts_with("not found") || header.record_length == 0 {
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
                let _json = block.to_json().unwrap();
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
    // println!("\n----- Summary -----");
    // println!("Ran {} blocks in {:.2?}", cnt, Instant::now() - start_time);
    // println!(
    //     "Bytes left: {} bytes",
    //     bytes.len().saturating_sub(next_header)
    // );
    // println!("\n----- Counts -----");
    // for (key, value) in m.into_iter() {
    //     println!("{} -> {}", key, value);
    // }

    let summary = format_summary(cnt, start_time, &bytes, next_header, m);
    println!("{}", &summary);
    summary
}

#[pyfunction]
fn extract_cdr_rs(file: String) {
    test_cdr_extraction(file);
}

/// A Python module implemented in Rust.
#[pymodule]
fn cdr_decoder(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(extract_cdr_rs, m)?)?;
    Ok(())
}
