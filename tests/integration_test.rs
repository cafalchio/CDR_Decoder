use cdr_decoder::data_blocks::header::Header;
use cdr_decoder::datatypes::charging_fields::*;
use cdr_decoder::datatypes::primitives::*;

#[cfg(test)]
mod tests {
    use super::*;
    use cdr_decoder::datatypes::primitives::BcdTimestamp;

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
        let byte: u8 = 0x12;
        let hexbyte = HByte::new(byte);
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
        let bytes = [0x01, 0x01, 0x00, 0x00];
        let hdword: HDWord = HDWord::new(&bytes);
        assert_eq!(257, hdword.value);
    }

    #[test]
    fn test_header() {
        let bytes: [u8; 25] = [
            0x26, 0x02, // record_lenght
            0x01, // record type
            0x03, 0x00, 0x00, 0x00, // record number
            0x00, // record status
            0xD7, 0x8D, // checksum
            0x31, 0x41, 0x24, 0x00, 0x00, // call reference
            0x94, 0x71, 0x37, 0x78, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // exchange id
            0xFF,
            // 0x00, 0x00, 0x00
        ];
        let header = Header::new(&bytes).unwrap();
        assert_eq!(header.record_length, 550);
        assert_eq!(header.record_type, "Mobile-originated call");
        assert_eq!(header.record_number, 3);
        assert_eq!(header.record_status, "normal ok");
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

    #[test]
    fn test_number() {
        let bytes: [u8; 10] = [0x94, 0x71, 0x37, 0x11, 0x60, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let decoded_number = NUMBER::new(&bytes).value;
        assert_eq!("4917731106", decoded_number);
    }

    #[test]
    fn test_exchange_id() {
        let bytes = [0x94, 0x71, 0x37, 0x78, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let exchange_id = ExchangeId::new(&bytes).value;
        assert_eq!("49177387", exchange_id);
    }

    #[test]
    fn test_called_subs_last_ex_id() {
        let bytes = [0x94, 0x71, 0x37, 0x78, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let called_subs_last_ex_id = LastExId::new(&bytes).value;
        assert_eq!("49177387", called_subs_last_ex_id);
    }
    #[test]
    fn test_calling_modify_parameters() {
        let bytes = [
            0x14, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let calling_modify_parameters = ModifyParameters::new(&bytes).value;
        assert_eq!("e1=14, e2=A, e4=64", calling_modify_parameters);
    }

    #[test]
    fn test_modify_percent() {
        let bytes: [u8; 2] = [0x06, 0x00];
        let modify_percent = ModifyPercent::new(&bytes).value;
        assert_eq!("6%", modify_percent);
    }

    #[test]
    fn test_facility_usage() {
        let bytes = [0xA0, 0x00, 0x02, 0x00];
        let facility_usage = FacilityUsage::new(&bytes).value;
        assert_eq!("call hold, multiparty, camel", facility_usage);
    }

    #[test]
    fn test_channel_rate_indicator() {
        let byte: u8 = 0x18;
        let channel_rate_indicator = ChannelRateIndicator::new(byte).value;
        assert_eq!(
            "requested: full rate, used: full rate",
            channel_rate_indicator
        );
    }

    #[test]
    fn test_camel_call_reference() {
        let bytes: [u8; 8] = [0x40, 0x04, 0x41, 0x31, 0x00, 0x40, 0x00, 0x00];
        let camel_call_reference = CamelCallReference::new(&bytes[..]).value;
        assert_eq!("4004413100400000", camel_call_reference);
    }

    #[test]
    fn test_calling_cell_band() {
        let byte = 0x01;
        let calling_cell_band = CellBand::new(byte).value;
        assert_eq!("GSM", calling_cell_band);
    }

    #[test]
    fn test_charge_number() {
        let bytes: [u8; 12] = [
            0x94, 0x71, 0x37, 0x11, 0x60, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ];
        let charge_number = NUMBER::new(&bytes[..]).value;
        assert_eq!("4917731106", charge_number);
    }

    #[test]
    fn test_charge_number_ton() {
        let byte = 0x05;
        let charge_number_ton = TON::new(byte).value;
        assert_eq!("International", charge_number_ton);
    }

    #[test]
    fn test_cug_interlock() {
        let bytes = [0x03, 0x44, 0x6F, 0x00];
        let cug_interlock = CugInterlock::new(&bytes).value;
        assert_eq!("network indicator: 0344, CUG code: 111", cug_interlock);
    }

    #[test]
    fn test_orig_mcz_pulses() {
        let bytes: [u8; 2] = [0x34, 0x12];
        let orig_mcz_pulses = Pulses::new(&bytes).value;
        assert_eq!("1234", orig_mcz_pulses);
    }

    #[test]
    fn test_orig_mcz_tariff_class() {
        let bytes: [u8; 3] = [0x01, 0x10, 0x00];
        let orig_mcz_tariff_class = TariffClass::new(&bytes).value;
        assert_eq!("001001", orig_mcz_tariff_class);
    }

    #[test]
    fn test_intermediate_chrg_cause() {
        let bytes: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
        let intermediate_chrg_cause = IntermediateChrgCause::new(&bytes).value;
        assert_eq!("Value at the end of the call", intermediate_chrg_cause);
    }

    #[test]
    fn test_intermediate_chrg_cause_2_bytes() {
        let bytes: [u8; 2] = [0x01, 0x00];
        let intermediate_chrg_cause = IntermediateChrgCause::new(&bytes).value;
        assert_eq!("Value at the end of the call", intermediate_chrg_cause);
    }

    // #[test]
    // fn test_full_extraction() {
    //     let test_file = "tests/test_file.bin";

    // }
}
