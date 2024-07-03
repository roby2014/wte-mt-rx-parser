//! MT(1) Raw Data Serial Out Packet Format
//! 
//! Data provided from the MT-RX via serial ports/TCP is in the following format:
//! - `MT1UUUNNNTFHHHHHHHHHHHHHHHSS112233N4445566WYYYY`
//! 
//! Where:
//! - `MT1` is fixed and actually “MT1”
//! - `UUU` - is a 3 character MT-RX configurable ID – by default this is “001”
//! - `NNN` -is a 3 decimal digit cycling packet sequence number from 000 to 511. This sequence number
//! increments after each new test or distress message is received. After 511 the sequence cycles to 000
//! and begins again.
//! - `T` – is a single character message type 'T' or 'A' (test or distress alert)
//! - `F` – is a single character format flag 'S' or 'L' (short or long) – this relates to the 406 beacon
//! transmission specification.
//! - `HHHHHHHHHHHHHHH` – is a 15 character hex code used to define beacon owner and beacon
//! capabilities as per the 406 beacon specification.
//! - `SS` – is a 2 character signal strength indication – “00” if not used.
//! - `11` – is a 2 decimal character latitude degrees
//! - `22` – is a 2 decimal character latitude minutes
//! - `33` – is a 2 decimal character latitude seconds
//! - `N` – is 'N' or 'S'
//! - `444` – is a 3 decimal character longitude degrees
//! - `22` – is a 2 decimal character longitude minutes
//! - `55` – is a 2 decimal character longitude seconds
//! - `W–` is 'W' or 'E'
//! - `YYYY` – is a 4 character checksum (calculated from M – the first character)
//! - If all location characters are '-' then there is no location information available.
//! 
//! Legitimate example packet:
//! `MT1001000AL400C592753572B323433212S1723756E4706`

use crate::ParseError;

/// Represents a MT message type.
#[derive(Clone, Debug, PartialEq)]
pub enum MtMessageType {
    Test,
    Alert,
    Unknown
}

impl std::convert::From<char> for MtMessageType {
    fn from(c: char) -> Self {
        match c {
            'T' => MtMessageType::Test,
            'A' => MtMessageType::Alert,
            _ => MtMessageType::Unknown,
        }
    }
}

/// MT Serial Out Packet Format.
#[derive(Clone, Debug, PartialEq)]
pub struct MtStructured {
    /// `MT1`.
    pub header: String,

    /// MT-RX configurable ID.
    pub id: String,

    /// Cycling packet sequence number.
    pub sequence_number: usize,

    /// Message type.
    pub message_type: MtMessageType,

    /// Character format flag.
    pub format_flag: char,

    /// Beacon owner and capabilities.
    pub beacon: [u8; 15],

    /// Signal strength indication.
    pub signal_strength: [char; 2],

    /// Latitude degrees.
    pub lat_degrees: u8,

    /// Latitude minutes.
    pub lat_minutes: u8,

    /// Latitude seconds.
    pub lat_seconds: u8,

    /// `N` or `S`.
    pub n: char,

    /// Longitude degrees.
    pub long_degrees: u16,

    /// Longitude minutes.
    pub long_minutes: u8,

    /// Longitude seconds.
    pub long_seconds: u8,

    /// `W` or `E`.
    pub w: char,

    /// Checksum.
    /// If there is no location information available, hence the value will be `0`.
    pub checksum: u16,
}

    /// Returns whether `message` is a valid MT(1) message.
    ///
    /// ## Examples
    /// ```
    /// use wte_mt_rx_parser::mt_structured;
    /// println!("is it MT1? {}", mt_structured::is_mt("MT1001000AL400C592753572B323433212S1723756E4706"));
    /// ```
    pub fn is_mt(message: &str) -> bool {
        return message.starts_with("MT1");
    }

    /// Tries to parse a "MT Serial Out Packet Format" `message`.
    ///
    /// ## Notes
    /// - Checksum is not calculated here.
    ///
    /// ## Examples
    /// ```
    /// use wte_mt_rx_parser::mt_structured;
    /// let parsed = mt_structured::parse("MT1001000AL400C592753572B323433212S1723756E4706").unwrap();
    /// // ...
    /// ```
    ///
    /// ## Message format
    /// Data provided should be in the following format:
    /// - `MT1UUUNNNTFHHHHHHHHHHHHHHHSS112233N4445566WYYYY`
    pub fn parse(message: &str) -> Result<MtStructured, ParseError> {
        // 012 345 678 9 0 123456789012345 67 89 01 23 4 567 89 01 2 3456
        // MT1 UUU NNN T F HHHHHHHHHHHHHHH SS 11 22 33 N 444 55 66 W YYYY

        const MT1_LEN: usize = 47;
        if message.len() != MT1_LEN {
            return Err(ParseError::SizeNotMatch {
                expected: MT1_LEN,
                found: message.len(),
            });
        }

        let header = message[0..3].to_string();
        let id = message[3..6].to_string();
        let sequence_number = message[6..9].parse::<usize>()?;
        let message_type = (message.as_bytes()[9] as char).into();
        let format_flag = message.as_bytes()[10] as char;
        let beacon: [u8; 15] = message[11..26].as_bytes().try_into().unwrap();
        let signal_strength = [
            message.as_bytes()[26] as char,
            message.as_bytes()[27] as char,
        ];
        let lat_degrees = message[28..30].parse::<u8>()?;
        let lat_minutes = message[30..32].parse::<u8>()?;
        let lat_seconds = message[32..34].parse::<u8>()?;
        let n = message.as_bytes()[34] as char;
        let long_degrees = message[35..38].parse::<u16>()?;
        let long_minutes = message[38..40].parse::<u8>()?;
        let long_seconds = message[40..42].parse::<u8>()?;
        let w = message.as_bytes()[42] as char;
        let checksum = u16::from_str_radix(&message[43..47], 16).unwrap_or(0);

        // TODO: calculate checksum here?

        let result = MtStructured {
            header,
            id,
            sequence_number,
            message_type,
            format_flag,
            beacon,
            signal_strength,
            lat_degrees,
            lat_minutes,
            lat_seconds,
            n,
            long_degrees,
            long_minutes,
            long_seconds,
            w,
            checksum,
        };

        Ok(result)
    }
