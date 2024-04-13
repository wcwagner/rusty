//! Figi Identifier
//! https://www.omg.org/spec/FIGI/1.1/Beta1/PDF
//! # Structure (pg. 31)
//! Allow characters (pg. 12):
//!    - "All upper case ISO 8859-1 consonants (including Y)."
//!    - "The single digit integers 0 – 9"
//! Characters 1 and 2
//!   - Any combination of upper case consonants with the following exceptions:
//!      - BS, BM, GG, GB, GH, KY, VG
//!      - "The purpose of the restriction is to reduce the changes that the resulting identifier may be identical to an ISIN string."
//!

#![allow(dead_code, unused_imports)]

use std::str::FromStr;
use winnow::ascii::alphanumeric1;
use winnow::ascii::digit1;
use winnow::combinator::cut_err;
use winnow::combinator::repeat;
use winnow::combinator::trace;
use winnow::error::StrContext;
use winnow::error::StrContextValue;
use winnow::prelude::*;
use winnow::stream::AsChar;
use winnow::token::none_of;
use winnow::token::one_of;
use winnow::token::take_while;

use winnow::combinator::seq;
use winnow::error::ErrMode;
use winnow::error::ErrorKind;
use winnow::error::ParserError;
use winnow::stream::Stream;

use std::fmt;

// NewType pattern inspired by https://www.worthe-it.co.za/blog/2020-10-31-newtype-pattern-in-rust.html
#[derive(Debug, PartialEq)]
pub struct Figi(String);

impl FromStr for Figi {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_figi.parse(s).map_err(|e| e.to_string())
    }
}

impl fmt::Display for Figi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

const CONSONANTS: &[char] = &[
    'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X',
    'Y', 'Z',
];

fn is_consonant(c: char) -> bool {
    CONSONANTS.contains(&c)
}

fn digit(input: &mut &str) -> PResult<char> {
    one_of('0'..='9').parse_next(input)
}

fn parse_figi<'s>(input: &mut &'s str) -> PResult<Figi> {
    let s = trace("figi", |s: &mut &'s str| {
        (
            take_while(2, is_consonant)
                .verify(|s: &str| !["BS", "BM", "GG", "GB", "GH", "KY", "VG"].contains(&s))
                .map(String::from)
                .context(StrContext::Expected(StrContextValue::Description(
                    "Two valid consonants not in restricted set",
                ))),
            'G'.context(StrContext::Expected(StrContextValue::Description("G"))),
            take_while(8, |c: char| is_consonant(c) || c.is_ascii_digit())
                .map(String::from)
                .context(StrContext::Expected(StrContextValue::Description(
                    "Eight consonant or numeric characters",
                ))),
            one_of('0'..='9').context(StrContext::Expected(StrContextValue::Description(
                "Check digit",
            ))),
        )
            .recognize()
            .parse_next(s)
    })
    .parse_next(input)?;
    Ok(Figi(s.to_string()))
}

#[cfg(test)]
mod exhaustive_tests {
    use super::*;

    #[test]
    fn valid_figi_examples() {
        let valid_figis = vec![
            "BBG000BLNNH6",
            "XCG00GFXXMR3",
            "XYG000PSJNQ9",
            // Add more valid FIGI examples as needed
        ];

        for input in valid_figis {
            let result = Figi::from_str(input).unwrap();
            assert_eq!(result.to_string(), input);
        }
    }

    #[test]
    fn invalid_length() {
        let invalid_figis = vec![
            "BBG00BLNNH6", // Too short
            "BBG000BLNNNHH6", // Too long
                           // Add more invalid length FIGI examples as needed
        ];

        for input in invalid_figis {
            let result = Figi::from_str(input);
            assert!(
                result.is_err(),
                "Should fail due to invalid length: {}",
                input
            );
        }
    }

    #[test]
    fn invalid_first_two_characters() {
        let invalid_starts = vec![
            "BSG000BLNNH6", // Forbidden start
            "BMG000BLNNH6", // Forbidden start
                            // Add more examples of invalid starting characters as needed
        ];

        for input in invalid_starts {
            let result = Figi::from_str(input);
            assert!(
                result.is_err(),
                "Should fail due to invalid start: {}",
                input
            );
        }
    }

    #[test]
    fn invalid_third_character() {
        let invalid_third = "BBX000BLNNH6"; // Third character is not 'G'

        let result = Figi::from_str(invalid_third);
        assert!(
            result.is_err(),
            "Should fail due to invalid third character"
        );
    }

    #[test]
    fn invalid_id_section() {
        let invalid_ids = vec![
            "BBG000BNNH6", // ID section too short/non-consonant/non-numeric
            "BBG0A0BLNNH6", // ID section contains invalid characters
                           // Add more invalid ID section FIGI examples as needed
        ];

        for input in invalid_ids {
            let result = Figi::from_str(input);
            assert!(
                result.is_err(),
                "Should fail due to invalid ID section: {}",
                input
            );
        }
    }

    #[test]
    fn invalid_check_digit() {
        let invalid_check_digit = "BBG000BLNNHH"; // Non-numeric check digit

        let result = Figi::from_str(invalid_check_digit);
        assert!(result.is_err(), "Should fail due to invalid check digit");
    }

    #[test]
    fn mispositioned_characters() {
        let invalid_positions = vec![
            "B1G000BLNNH6", // Numeric character in first two positions
        ];

        for input in invalid_positions {
            let result = Figi::from_str(input);
            assert!(
                result.is_err(),
                "Should fail due to mispositioned characters: {}",
                input
            );
        }
    }

    #[test]
    fn valid_figi_with_garbage_following() {
        let inputs_with_garbage = vec![
            "BBG000BLNNH6EXTRA", // Valid FIGI with extra characters at the end
        ];

        for input in inputs_with_garbage {
            let result = Figi::from_str(input);
            assert!(
                result.is_err(),
                "Should fail as it contains additional garbage values: {}",
                input
            );
        }
    }

    #[test]
    fn empty_string() {
        let result = Figi::from_str("");
        assert!(result.is_err(), "Should fail due to being an empty string");
    }

    #[test]
    fn non_ascii_characters() {
        let non_ascii_inputs = vec![
            "ББG000BLNNH6",        // Cyrillic characters
            "嗨G000BLNNH6",        // Chinese character
            "BBG😀00BLNNH6",       // Emoji
            "\u{200D}BG000BLNNH6", // Zero-width joiner
            "BBG0🚀0BLNNH6",       // Emoji within the ID section
        ];

        for input in non_ascii_inputs {
            let result = Figi::from_str(input);
            assert!(
                result.is_err(),
                "Should fail due to non-ASCII characters: {}",
                input
            );
        }
    }

    #[test]
    fn truly_garbage_inputs() {
        let garbage_inputs = vec![
            "!@#$%^&*()_+", // Special characters
            "    ",         // Whitespace characters
            "\0\0\0\0",     // Null bytes
            "LONG_STRING_WITHOUT_ANY_VALID_CHARS_OR_STRUCTURE_TO_IT_SHOULD_FAIL", // Long irrelevant string
            "\u{FFFF}BBG000BLNNH6", // Invalid Unicode character
        ];

        for input in garbage_inputs {
            let result = Figi::from_str(input);
            assert!(
                result.is_err(),
                "Should fail due to garbage input values: {}",
                input
            );
        }
    }
}
