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

/// Represents a cardinal direction.
#[derive(Clone, Debug, PartialEq)]
pub enum CardinalDirection {
    North,
    South,
    West,
    East,
    Unknown,
}

impl std::convert::From<char> for CardinalDirection {
    fn from(c: char) -> Self {
        match c {
            'W' => CardinalDirection::West,
            'E' => CardinalDirection::East,
            'N' => CardinalDirection::North,
            'S' => CardinalDirection::South,
            _ => CardinalDirection::Unknown,
        }
    }
}

/// Represents a MT message type.
#[derive(Clone, Debug, PartialEq)]
pub enum MtMessageType {
    Test,
    Alert,
    Unknown,
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

    /// Beacon hex code.
    pub beacon: String,

    /// Signal strength indication.
    pub signal_strength: String,

    /// Latitude degrees. `None` if not available.
    pub lat_degrees: Option<u8>,

    /// Latitude minutes. `None` if not available.
    pub lat_minutes: Option<u8>,

    /// Latitude seconds. `None` if not available.
    pub lat_seconds: Option<u8>,

    /// North or South.
    pub lat_direction: CardinalDirection,

    /// Longitude degrees. `None` if not available.
    pub long_degrees: Option<u16>,

    /// Longitude minutes. `None` if not available.
    pub long_minutes: Option<u8>,

    /// Longitude seconds. `None` if not available.
    pub long_seconds: Option<u8>,

    /// West or East.
    pub long_direction: CardinalDirection,

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
    let beacon = message[11..26].to_string();
    let signal_strength = message[26..28].to_string();
    let lat_degrees = message[28..30].parse::<u8>().ok();
    let lat_minutes = message[30..32].parse::<u8>().ok();
    let lat_seconds = message[32..34].parse::<u8>().ok();
    let lat_direction = (message.as_bytes()[34] as char).into();
    let long_degrees = message[35..38].parse::<u16>().ok();
    let long_minutes = message[38..40].parse::<u8>().ok();
    let long_seconds = message[40..42].parse::<u8>().ok();
    let long_direction = (message.as_bytes()[42] as char).into();
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
        lat_direction,
        long_degrees,
        long_minutes,
        long_seconds,
        long_direction,
        checksum,
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fields() {
        // MT1 UUU NNN T F HHHHHHHHHHHHHHH SS 11 22 33 N 444 55 66 W YYYY

        // MT1 001 000 A L 400C592753572B3 23 43 32 12 S 172 37 56 E 4706 <- valid
        let parsed = parse("MT1001000AL400C592753572B323433212S1723756E4706").unwrap();
        assert_eq!(parsed.id, "001");
        assert_eq!(parsed.sequence_number, 0);
        assert_eq!(parsed.message_type, MtMessageType::Alert);
        assert_eq!(parsed.format_flag, 'L');
        assert_eq!(parsed.beacon, "400C592753572B3");
        assert_eq!(parsed.signal_strength, "23");
    }

    #[test]
    fn valid_location_and_direction() {
        // MT1 UUU NNN T F HHHHHHHHHHHHHHH SS 11 22 33 N 444 55 66 W YYYY

        // MT1 001 000 A L 400C592753572B3 23 43 32 12 S 172 37 56 E 4706 <- valid
        let parsed = parse("MT1001000AL400C592753572B323433212S1723756E4706").unwrap();

        assert_eq!(parsed.lat_degrees.unwrap(), 43);
        assert_eq!(parsed.lat_minutes.unwrap(), 32);
        assert_eq!(parsed.lat_seconds.unwrap(), 12);
        assert_eq!(parsed.lat_direction, CardinalDirection::South);

        assert_eq!(parsed.long_degrees.unwrap(), 172);
        assert_eq!(parsed.long_minutes.unwrap(), 37);
        assert_eq!(parsed.long_seconds.unwrap(), 56);
        assert_eq!(parsed.long_direction, CardinalDirection::East);
    }

    #[test]
    fn unknown_location() {
        // MT1 UUU NNN T F HHHHHHHHHHHHHHH SS 11 22 33 N 444 55 66 W YYYY

        // MT1 001 000 A L 400C592753572B3 23 43 32 12 S --- -- -- E 4706 <- invalid
        let parsed = parse("MT1001000AL400C592753572B323433212S-------E4706").unwrap();
        assert!(parsed.long_degrees.is_none());
        assert!(parsed.long_minutes.is_none());
        assert!(parsed.long_seconds.is_none());

        // MT1 001 000 A L 400C592753572B3 23 -- -- -- S 172 37 56 E 4706 <- invalid
        let parsed = parse("MT1001000AL400C592753572B323------S1723756E4706").unwrap();
        assert!(parsed.lat_degrees.is_none());
        assert!(parsed.lat_minutes.is_none());
        assert!(parsed.lat_seconds.is_none());
    }
}
