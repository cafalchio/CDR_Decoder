use cdr_decoder::datatypes::primitives::*;
use cdr_decoder::datatypes::blocks::*;
use cdr_decoder::datatypes::mixed::*;

#[cfg(test)]
mod tests {
    use cdr_decoder::datatypes::primitives::BcdTimestamp;

    use super::*;

    #[test]
    fn test_bcd_value() {
        let bcd: BCD = BCD::new(&0x05);
        assert_eq!(bcd.value, 5);
    }

    #[test]
    fn test_bcd_word() {
        let bytes: [u8; 2] = [0x00, 0x05];
        let bcd_word: BCDWord = BCDWord::new(&bytes);
        assert_eq!(bcd_word.value, 500);
    }

    #[test]
    fn test_bcd_dword() {
        let bytes: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
        let bcd_dword = BCD2uword::new(&bytes);
        assert_eq!(bcd_dword.value, 1);
    }

    #[test]
    fn test_timestamp() {
        let bytes: [u8; 7] = [0x40, 0x15, 0x12, 0x10, 0x11, 0x98, 0x19];
        let timestamp: BcdTimestamp = BcdTimestamp::new(&bytes);
        assert_eq!("10/11/1998 12:15:40", timestamp.value);
    }

    #[test]
    fn test_hbyte() {
        let byte: [u8; 1] = [0x12];
        let hexbyte = HByte::new(&byte);
        assert_eq!(18, hexbyte.value);
    }

    #[test]
    fn test_hword() {
        let bytes = [0x6F, 0x00];
        let hword = HWord::new(&bytes);
        assert_eq!(111, hword.value);
    }

    #[test]
    fn test_hdword() {
        let bytes = [0x00, 0x02, 0x00, 0xA0];
        let hdword = HDWord::new(&bytes);
        assert_eq!(131232, hdword.value);
    }


    #[test]
    fn test_header() {
        let bytes: [u8; 25] = [
            0x26, 0x02, // record_lenght
            0x01,  // record type
            0x03, 0x00, 0x00, 0x00, // record number
            0x00, // record status
            0xD7, 0x8D, // checksum
            0x31, 0x41, 0x24, 0x00, 0x00,
            0x94, 0x71, 0x37, 0x78, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            // 0x00, 0x00, 0x00
        ];
        let header = Header::new(&bytes);
        assert_eq!(header.record_length, 550);
        assert_eq!(header.record_type, "Mobile-Originated Call");
        assert_eq!(header.record_number, 3);
        assert_eq!(header.record_status, "Normal");
        assert_eq!(header.check_sum, 36311);
        assert_eq!(header.call_reference, "comp:4131 process:0024 focus:00");
        assert_eq!(header.exchange_id, "49177387"); 
        
    }

    #[test]
    fn test_acceptable_channel_codings() {
        let byte = 0x00;
        let acceptable_codings = AcceptableChannelCodings::new(byte);
        assert_eq!("", acceptable_codings.value);

        let byte = 0x01;
        let acceptable_codings = AcceptableChannelCodings::new(byte);
        assert_eq!("4,8 kbit/s", acceptable_codings.value);

        let byte = 0x03;
        let acceptable_codings = AcceptableChannelCodings::new(byte);
        assert_eq!("4,8 9,6 kbit/s", acceptable_codings.value);

        let byte = 0x33;
        let acceptable_codings = AcceptableChannelCodings::new(byte);
        assert_eq!("4,8 9,6 28,8 32,0 kbit/s", acceptable_codings.value);
    }

}
