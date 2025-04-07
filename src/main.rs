mod datatypes;
use cdr_decoder::{core::process_file::*, data_blocks::header};
// use datatypes::primitives::BcdTimestamp;
fn main() {
    println!("Running extraction");
    let bytes = read_file("data/VL_GNK_MSSDF5_T20250115111404_22245_N_00000.BACKUP.gz");
    let header = extract_header(&bytes);
    println!("rec lenght: {}", header.record_length);
    println!("record type: {}", header.record_type);
    println!("checksum: {}", header.check_sum);
    println!("call ref: {}", header.call_reference);
    println!("exchange id: {}", header.exchange_id);
    println!("status: {}", header.record_status);
}
