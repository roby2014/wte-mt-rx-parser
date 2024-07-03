//! MT(6) Raw Data Serial Out Packet Format
//! 
//! **From firmware revision v1.88 the raw data output packet is supported.**
//! 
//! Data provided from the MT-RX is in the following format:
//! - `MT6UUUNNNRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRYYYY`
//! 
//! Where:
//! - `MT6` is fixed and actually “MT6”
//! - `UUU`- is a 3 character MT-RX configurable ID – by default this is “001”
//! - `NNN` -is a 3 decimal digit cycling packet sequence number from 000 to 511. This sequence number
//! increments after each new test or distress message is received. After 511 the sequence cycles to 000
//! and begins again.
//! - `RRR`.. is 36 characters of raw data in a hex format.
//! - `YYYY` – is a 4 character checksum (calculated from R – the first raw data character)

use crate::ParseError;

/// MT Raw Data Serial Out Packet Format.
#[derive(Clone, Debug, PartialEq)]
pub struct MtRaw {
    /// `MT6`.
    pub header: String,

    /// MT-RX configurable ID.
    pub id: String,

    /// Cycling packet sequence number.
    pub sequence_number: usize,

    /// Raw data.
    pub data: [u8; 36],

    /// Checksum.
    pub checksum: u16,
}

    /// Returns whether `message` is a valid MT(6) message.
    ///
    /// ## Examples
    /// ```
    /// use wte_mt_rx_parser::mt_raw;
    /// println!("is it MT6? {}", mt_raw::is_mt("MT6001001FFFE2FA00E0000CBAB959DB0903788C71B79F84B"));
    /// ```
    pub fn is_mt(message: &str) -> bool {
        return message.starts_with("MT6");
    }

    /// Tries to parse a "Raw Data Serial Out Packet Format" `message`.
    ///
    /// ## Notes
    /// - Checksum is not calculated here. Use [`compute_checksum`] if you require
    /// it to be correct.
    ///
    /// ## Examples
    /// ```
    /// use wte_mt_rx_parser::mt_raw;
    /// let parsed = mt_raw::parse("MT6001001FFFE2FA00E0000CBAB959DB0903788C71B79F84B").unwrap();
    /// println!("parsed: {:?}", parsed);
    /// ```
    ///
    /// ## Message format
    /// Data provided should be in the following format:
    /// - `MT6001001FFFE2FA00E0000CBAB959DB0903788C71B79F84B`
    pub fn parse(message: &str) -> Result<MtRaw, ParseError> {
        // 012 345 678 901234567890123456789012345678901234 5678
        // MT6 UUU NNN RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR YYYY

        const MT6_LEN: usize = 49;
        if message.len() != MT6_LEN {
            return Err(ParseError::SizeNotMatch {
                expected: MT6_LEN,
                found: message.len(),
            });
        }

        let header = message[0..3].to_string();
        let id = message[3..6].to_string();
        let sequence_number = message[6..9].parse::<usize>()?;
        let data: [u8; 36] = message[9..45].as_bytes().try_into().unwrap();
        let checksum = u16::from_str_radix(&message[45..49], 16)?;

        // TODO: calculate checksum here?

        let result = MtRaw {
            header,
            id,
            sequence_number,
            data,
            checksum,
        };

        Ok(result)
    }

    /// Calculate checksum of `data_source`.
    ///
    /// ## Examples
    /// ```
    /// use wte_mt_rx_parser::mt_raw;
    /// if mt_raw::compute_checksum("FFFE2FA00E0000CBAB959DB0903788C71B79".as_bytes()) == 0xf84b {
    ///     println!("valid checksum!");
    /// } else {
    ///     println!("not a valid checksum!");
    /// }
    /// ```
    pub fn compute_checksum(data_source: &[u8]) -> u16 {
        let mut checksum = 0;
        for byte in data_source.iter() {
            checksum ^= *byte as u16;
            if (checksum & 0x8000) != 0 {
                checksum = (checksum << 1) | 0x01;
            } else {
                checksum <<= 1;
            }
        }
        checksum
    }
