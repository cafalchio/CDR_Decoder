import os
import gzip
import binascii
import sys

def bcd_to_decimal(b):
    """Convert a BCD byte to a decimal integer."""
    return ((b >> 4) & 0x0F) * 10 + (b & 0x0F)

def get_block_type(bcd_byte):
    """Return block type name based on BCD-encoded byte."""
    bcd_value = bcd_to_decimal(bcd_byte)
    return {
        0: "Header",
        1: "Mobile-originated call",
        2: "Mobile-terminated call",
        3: "Forwarded call",
        4: "Call to a roaming subscriber",
        5: "Supplementary service",
        6: "HLR interrogation",
        7: "Location update",
        8: "Short message service (point-to-point), mobile-originated",
        9: "Short message service (point-to-point), mobile-terminated",
        10: "Trailer",
        11: "PSTN-originated call",
        12: "PSTN-terminated call",
        13: "PBX-originated call",
        14: "PBX-terminated call",
        15: "Use of hardware",
        16: "Intelligent network data 1",
        17: "Unsuccessful call attempt",
        18: "Intelligent network data 2",
        19: "Intelligent network data 3",
        20: "Device-originated call",
        22: "Remote charging control",
        23: "IN-forwarde Sms",
        24: "Camel-originated call",
        25: "Camel-terminated call",
        26: "Intelligent network data 4",
        27: "Location service",
        28: "Intelligent network data 5",
        29: "Unstructured supplementary service data",
        30: "SIP-originated call",
        31: "SIP-terminated call",
        32: "SIP-originating message",
        33: "SIP-terminating message",
        35: "SIP CDR for registration"
    }.get(bcd_value, "Unknown")

def process_cdr_blocks(file_path, output_path=None):
    """
    Process CDR blocks by reading just their length and type.
    
    Args:
        file_path: Path to the gzipped CDR file
        output_path: Path to save the output summary
    """
    cnt = 0
    print(f"Processing file: {file_path}")
    
    if output_path is None:
        output_path = file_path + ".summary.csv"

    with gzip.open(file_path, 'rb') as f, open(output_path, 'w') as out:
        # Write header
        out.write("Block_Index;Block_Type;Block_Length;Position\n")
        
        block_count = 0
        position = 0
        
        while True:
            length_bytes = f.read(2)
            if not length_bytes or len(length_bytes) < 2:
                break
            
            b1 = binascii.b2a_hex(length_bytes[0:1]).decode()
            b2 = binascii.b2a_hex(length_bytes[1:2]).decode()
            block_length = int(b2 + b1, 16)

            type_byte = f.read(1)
            if not type_byte:
                print("Incomplete block at end of file")
                break

            block_type = get_block_type(type_byte[0])
            
            block_count += 1
            out.write(f"{block_count};{block_type};{block_length};{position}\n")
            
            remaining_bytes = block_length - 3
            if remaining_bytes > 0:
                f.read(remaining_bytes)
            
            position += block_length

    print(f"Processed {block_count} blocks")

def main():
    input_file = "/home/cafalchio/projects/CDR_Decoder/data/test_file1.gz"
    process_cdr_blocks(input_file)

if __name__ == "__main__":
    main()
