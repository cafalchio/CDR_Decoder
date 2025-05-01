use crate::data_blocks::header::Header;
use flate2::read::GzDecoder;
use std::io::Read;
use walkdir::{DirEntry, WalkDir};

fn is_gzip(entry: &DirEntry) -> bool {
    // just a filter for gz files
    entry.file_type().is_file()
        && entry
            .file_name()
            .to_str()
            .map(|s| s.ends_with(".gz"))
            .unwrap_or(false)
}

pub fn read_multiple_files(path: &str) {
    // perhaps this will be where we extract,
    // we could break un getting the folders and then send the list of folders to be extracted
    let entries: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| is_gzip(e))
        .collect();

    for entry in entries {
        if let Some(path_str) = entry.path().to_str() {
            let bytes = read_file(path_str);
            read_headers(&bytes);
            // println!("Read {}", path_str);
        }
    }
}

pub fn read_file(path: &str) -> Vec<u8> {
    // currently reading the entire file in memory as the compress file has 1 to 2Mb
    let bytes = std::fs::read(path).unwrap();
    let mut gz = GzDecoder::new(&bytes[..]);
    let mut file_bytes = Vec::new();
    gz.read_to_end(&mut file_bytes).unwrap();
    file_bytes
}

pub fn extract_header(bytes: &[u8]) -> Header {
    let mut next_header = 0;
    while &bytes[next_header] == &0xFF {
        println!("escape");
        next_header += 1;
    }
    Header::new(&bytes[next_header..next_header+25])
}

fn read_headers(bytes: &[u8]) {
    let mut next_header: usize = 0;
    let mut counter = 0;
    loop {
        let header = extract_header(&bytes[next_header..]);
        header.record_length;
        header.check_sum;
        header.call_reference;
        header.exchange_id;
        header.record_status;
        next_header += 25 + header.record_length as usize;
        counter += 1;
    }
    println!("Extracted {} Headers", counter);
}

pub fn read_last_blocks(bytes: &[u8], start_pointer: usize, offset: usize, max_blocks: u32) -> u32 {
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
