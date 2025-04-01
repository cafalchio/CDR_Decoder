mod datatypes;
use datatypes::primitives::BcdTimestamp;
fn main() {
    let hex_dump = [0x46, 0x58, 0x15, 0x09, 0x04, 0x96, 0x19];
    let timestamp = BcdTimestamp::from_hex_dump(hex_dump);
    println!("BCD Timestamp: {}", timestamp);
    println!("Hex Dump Representation: {}", timestamp.to_bcd_string());
}
