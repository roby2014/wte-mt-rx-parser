//! A Rust parser for "MT-RX-3 AIS, 406 + 121.5/243 MHz ALERTING RECEIVER" messages.
//!
//! Please refer to [MT-RX-3 User Manual](https://www.wte.co.nz/uploads/9/9/8/6/99862766/mt-rx-3_406_epirb_receiver-manual_v2-62.pdf) for more information.

use std::num::ParseIntError;

use mt_raw::MtRaw;
use mt_structured::MtStructured;
use rss::Rss;
use thiserror::Error;

pub mod mt_raw;
pub mod mt_structured;
pub mod rss;

/// Represents an error when parsing a message went wrong.
#[derive(Error, Clone, Debug, PartialEq)]
pub enum ParseError {
    #[error("failed to parse number")]
    ParseIntError(#[from] ParseIntError),

    #[error("invalid message size (expected {expected:?}, found {found:?})")]
    SizeNotMatch { expected: usize, found: usize },

    #[error("invalid sentence, not parsable")]
    Invalid,
}

/// Represents the parsed message.
#[derive(Clone, Debug, PartialEq)]
pub enum ParsedMessage {
    /// RSS - Received Signal Strength message.
    Rss(Rss),

    /// MT Serial Out Packet Format message.
    MtStructured(MtStructured),

    /// MT Raw Data Serial Out Packet Format message.
    MtRaw(MtRaw),

    /// Invalid message.
    Invalid,
}

/// Tries to parse `message` into one of [`ParsedMessage`] types.
/// Returns [`ParsedMessage::Invalid`] if it's an invalid message, or [`ParseError`] if parsing went wrong.
///
/// ## Examples
/// ```
/// let samples = vec![
///     "MT1001000AL400C592753572B323433212S1723756E4706",
///     "MT6001001FFFE2FA00E0000CBAB959DB0903788C71B79F84B",
///     "SS,A,123",
///     "SS,1,123",
/// ];
///
/// for s in samples {
///     println!("{:?}", wte_mt_rx_parser::parse(s));
/// }
/// ```
pub fn parse(message: &str) -> Result<ParsedMessage, ParseError> {
    let parsed = match message.trim() {
        msg if rss::is_rss(msg) => ParsedMessage::Rss(rss::parse(msg)?),
        msg if mt_structured::is_mt(msg) => ParsedMessage::MtStructured(mt_structured::parse(msg)?),
        msg if mt_raw::is_mt(msg) => ParsedMessage::MtRaw(mt_raw::parse(msg)?),
        _ => ParsedMessage::Invalid,
    };
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mt_structured() {
        assert!(parse("MT1001000AL400C592753572B323433212S1723756E4706").is_ok());
        assert!(parse("MT1001000AL400C592753572B323433212S1723756E4706").is_ok());
    }

    #[test]
    fn mt_raw() {
        assert!(parse("MT6001001FFFE2FA00E0000CBAB959DB0903788C71B79F84B").is_ok());
        assert!(parse("MT6001001FFFE2FA0062C93A9AB959E55EE7788C71B791131").is_ok());
    }

    #[test]
    fn invalid_mt() {
        // invalid headers
        let invalid_samples = vec![
            "MT2001000AL400C592753572B323433212S1723756E4706",
            "MT3001000AL400C592753572B323433212S1723756E4706",
            "MT4001000AL400C592753572B323433212S1723756E4706",
            "MT5001000AL400C592753572B323433212S1723756E4706",
            "MT8001000AL400C592753572B323433212S1723756E4706",
            "MT9001000AL400C592753572B323433212S1723756E4706",
            "MT0001000AL400C592753572B323433212S1723756E4706",
        ];
        for s in invalid_samples {
            assert_eq!(parse(s).unwrap(), ParsedMessage::Invalid);
        }

        // smaller than expected
        assert!(parse("MT6001001FFFE2FA00E0000CBAB959DB0903788C71B").is_err());
        assert!(parse("MT1001000AL400C592753572B323433212S172375").is_err());

        // MT1 UUU NNN T F HHHHHHHHHHHHHHH SS 11 22 33 N 444 55 66 W YYYY
        // MT6 UUU NNN RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR YYYY

        // not parsable numbers

        // unparsable NNN
        assert!(parse("MT1001aaaAL400C592753572B323433212S1723756E4706").is_err());
        assert!(parse("MT6001aaaFFFE2FA00E0000CBAB959DB0903788C71B79F84B").is_err());

        // unparsable checksum
        assert!(parse("MT6001001FFFE2FA00E0000CBAB959DB0903788C71B79ZZZZ").is_err());
    }

    #[test]
    fn rss() {
        assert!(parse("SS,A,123\n").is_ok());
        assert!(parse("SS,1,123\n").is_ok());
    }

    #[test]
    fn invalid_rss() {
        assert!(parse("SS,1,666\n").is_err()); // nnn too big
        assert!(parse("SS,1,12367\n").is_err()); // nnn too big
        assert!(parse("SS,2,123\n").is_err()); // invalid type
        assert!(parse("SS,A,1234\n").is_err()); // nnn too big
        assert!(parse("SS,X,123\n").is_err()); // invalid type
    }

    #[test]
    fn hardcoded_checksum() {
        assert_eq!(
            mt_raw::compute_checksum("FFFE2FA00E0000CBAB959DB0903788C71B79".as_bytes()),
            0xf84b
        );
    }

    #[test]
    fn mt_raw_checksum() {
        if let Ok(ParsedMessage::MtRaw(v)) =
            parse("MT6001001FFFE2FA00E0000CBAB959DB0903788C71B79F84B")
        {
            assert_eq!(mt_raw::compute_checksum(&v.data.as_bytes()), v.checksum);
        }
    }
}
