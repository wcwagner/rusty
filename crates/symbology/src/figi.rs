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

#[derive(Debug, PartialEq)]
struct Figi {
    first_two: String,
    third: char,
    id: String,
    check: char,
}

impl Figi {
    fn new(first_two: String, third: char, id: String, check: char) -> Self {
        Self {
            first_two,
            third,
            id,
            check,
        }
    }
}

const CONSONANTS: &[char] = &[
    'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X',
    'Y', 'Z',
];

fn is_consonant(c: char) -> bool {
    CONSONANTS.contains(&c)
}

fn is_consnumeric(c: char) -> bool {
    is_consonant(c) || c.is_ascii_digit()
}

fn digit(input: &mut &str) -> PResult<char> {
    one_of('0'..='9').parse_next(input)
}

fn parse_figi<'s>(input: &mut &'s str) -> PResult<Figi> {
    trace("figi", |s: &mut &'s str| {
        let (first_two, g, id, check) = cut_err((
            take_while(2, is_consonant)
                .verify(|s: &str| !["BS", "BM", "GG", "GB", "GH", "KY", "VG"].contains(&s))
                .context(StrContext::Expected(StrContextValue::Description(
                    "any two consonants except for 'BS', 'BM', 'GG', 'GB', 'GH', 'KY', 'VG'",
                ))),
            'G'.context(StrContext::Expected(StrContextValue::Description("G"))),
            take_while(8, is_consnumeric).context(StrContext::Expected(
                StrContextValue::Description("eight consonant or numeric characters"),
            )),
            one_of('0'..='9').context(StrContext::Expected(StrContextValue::Description(
                "check digit",
            ))),
        ))
        .parse_next(s)?;
        Ok(Figi {
            first_two: first_two.to_owned(),
            third: g,
            id: id.to_owned(),
            check,
        })
    })
    .parse_next(input)
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
            let result = parse_figi.parse_peek(input);
            assert!(result.is_ok(), "Failed on valid input: {}", input);
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
            let result = parse_figi.parse_peek(input);
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
            let result = parse_figi.parse_peek(input);
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

        let result = parse_figi.parse_peek(invalid_third);
        assert!(
            result.is_err(),
            "Should fail due to invalid third character"
        );
    }

    #[test]
    fn invalid_id_section() {
        let invalid_ids = vec![
            "BBG000BNNH6", // ID section contains non-consonant/non-numeric
            "BBG0A0BLNNH6", // ID section contains invalid characters
                           // Add more invalid ID section FIGI examples as needed
        ];

        for input in invalid_ids {
            let result = parse_figi.parse_peek(input);
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

        let result = parse_figi.parse_peek(invalid_check_digit);
        assert!(result.is_err(), "Should fail due to invalid check digit");
    }

    #[test]
    fn mispositioned_characters() {
        let invalid_positions = vec![
            "B1G000BLNNH6", // Numeric character in first two positions
        ];

        for input in invalid_positions {
            let result = parse_figi.parse_peek(input);
            assert!(
                result.is_err(),
                "Should fail due to mispositioned characters: {}",
                input
            );
        }
    }
}