use cdr_decoder::datatypes::primitives::*;

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
    fn test_bcdword_parse_value() {
        let bytes: [u8; 2] = [0x00, 0x05];
        let bcd_word: BCDWord = BCDWord::new(&bytes);
        assert_eq!(bcd_word.value, 500);
    }

    #[test]
    fn test_bcd_dword() {
        let bytes:[u8; 4] = [0x01, 0x00, 0x00, 0x00];
        let bcd_dword = BCD2uword::new(&bytes);
        assert_eq!(bcd_dword.value, 1);
    }

    #[test]
    fn test_timestamp() {
        let bytes: [u8; 7] = [0x40, 0x15, 0x12, 0x10,  0x11, 0x98, 0x19];
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


}