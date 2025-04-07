use flate2::read::GzDecoder;
use crate::data_blocks::header::Header;
use std::io::Read;

pub fn read_file(path:&str) -> Vec<u8> {
    let bytes = std::fs::read(path).unwrap();
    let mut gz = GzDecoder::new(&bytes[..]);
    let mut file_bytes = Vec::new();
    gz.read_to_end(&mut file_bytes).unwrap();
    file_bytes
}

pub fn extract_header(bytes: &[u8]) -> Header {
    let header = Header::new(&bytes[0..25]);
    header
}



