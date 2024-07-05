//! RSS - Received Signal Strength
//!
//! The serial output format for the RSS frequency is:
//! - `SS,1,NNN<CR>`
//!
//! where `NNN` is a value between 0 and 255. This value is not calibrated,
//! but is approximately `-130 + (NNN / 2)` dBm.
//!
//! When an RSS is above the SQUELCH level the following serial output is generated, regardless of
//! the alert setting.
//! - `SS,A,NNN<CR>`

use crate::ParseError;

/// Represents a RSS “Received Signal Strength" message type.
#[derive(Clone, Debug, PartialEq)]
pub enum RssType {
    /// Frequency message `SS,1,NNN<CR>`.
    Frequency,
    /// Alert message `SS,A,NNN<CR>`.
    Alert,
}

/// Represents a RSS “Received Signal Strength" message.
#[derive(Clone, Debug, PartialEq)]
pub struct Rss {
    /// Message type (see [`RssType`]).
    pub rss_type: RssType,

    /// NNN - Value between 0 and 255. This value is not calibrated, but is approximately `-130 + (NNN / 2)` dBm
    pub nnn: u8,
}

/// Returns whether `message` is a valid RSS message.
///
/// ## Examples
/// ```
/// use wte_mt_rx_parser::rss;
/// println!("is it rss? {}", rss::is_rss("SS,A,123"));
/// ```
pub fn is_rss(message: &str) -> bool {
    return message.starts_with("SS,");
}

/// Tries to parse a RSS `message`.
///
/// ## Examples
/// ```
/// use wte_mt_rx_parser::rss;
/// let parsed = rss::parse("SS,A,123").unwrap();
/// println!("parsed: {:?}", parsed);
/// ```
pub fn parse(message: &str) -> Result<Rss, ParseError> {
    // 01 2 3 4 567
    // SS , X , NNN

    const RSS_LEN: usize = 8;
    if message.len() != RSS_LEN {
        return Err(ParseError::SizeNotMatch {
            expected: RSS_LEN,
            found: message.len(),
        });
    }

    let x = message.as_bytes()[3];
    let nnn = message[5..8].parse::<u8>()?;

    match x {
        b'A' => Ok(Rss {
            nnn,
            rss_type: RssType::Alert,
        }),
        b'1' => Ok(Rss {
            nnn,
            rss_type: RssType::Frequency,
        }),
        _ => Err(ParseError::Invalid),
    }
}
