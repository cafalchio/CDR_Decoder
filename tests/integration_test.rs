use cdr_decoder::datatypes::primitives::{BCD, BCDWord};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bcd_word_as_dec() {
        let high = BCD { value: 0x12 };  
        let low = BCD { value: 0x34 };
        let bcd_word = BCDWord { high, low };
        assert_eq!(bcd_word.as_dec(), 1234);
    }

    #[test]
    fn test_bcd_word_display() {
        // Create BCD for high (12) and low (34)
        let high = BCD { value: 0x12 };  
        let low = BCD { value: 0x34 }; 
        let bcd_word = BCDWord { high, low };
        let expected_display = "1234";
        assert_eq!(format!("{}", bcd_word.as_dec()), expected_display);
    }
}