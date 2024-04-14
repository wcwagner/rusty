//! Figi Identifier
//! https://www.omg.org/spec/FIGI/1.1/Beta1/PDF

use std::ops::RangeInclusive;
use std::str::FromStr;
use winnow::error::StrContext;
use winnow::error::StrContextValue;
use winnow::prelude::*;
use winnow::token::literal;
use winnow::token::one_of;
use winnow::token::take_while;

use std::fmt;

// NewType pattern inspired by https://www.worthe-it.co.za/blog/2020-10-31-newtype-pattern-in-rust.html
#[derive(Debug, PartialEq)]
pub struct Figi(String);

impl FromStr for Figi {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes_slice = s.as_bytes();
        // Now that we have a [u8; 12], we can pass it to the parser
        // Assuming `parse_figi` is adapted to work with a fixed-size byte array
        match parse_figi.parse(&mut bytes_slice) {
            Ok(_) => Ok(Figi(s.to_owned())), // If parsing succeeds, create a Figi instance
            Err(_) => Err(String::from("Failed to parse FIGI")), // Handle parsing errors appropriately
        }
    }
}

impl fmt::Display for Figi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn is_consonant(b: u8) -> bool {
    matches!(b, b'B'..=b'D' | b'F'..=b'H' | b'J'..=b'N' | b'P'..=b'T' | b'V'..=b'Z')
}

fn is_conso_numeric(b: u8) -> bool {
    is_consonant(b) || matches!(b, b'0'..=b'9')
}

fn is_valid_prefix(input: &[u8]) -> bool {
    match input {
        b"BS" | b"BM" | b"GG" | b"GB" | b"GH" | b"KY" | b"VG" => false,
        _ => true,
    }
}

#[inline(always)]
fn prefix<'s>(input: &mut &'s [u8]) -> PResult<&'s [u8]> {
    // Almost all Figi's are issued by Bloomberg and start with "BB"
    // Optimistic parsing here nets 17% performance gain
    use winnow::combinator::alt;
    alt((
        literal(b"BBG").void(),
        (
            take_while(2usize, is_consonant).verify(is_valid_prefix),
            b'G',
        )
            .void(),
    ))
    .context(StrContext::Expected(StrContextValue::Description(
        "Two valid consonants  follow by a 'G'",
    )))
    .recognize()
    .parse_next(input)
}

fn parse_figi<'s>(input: &mut &'s [u8]) -> PResult<&'s [u8]> {
    (
        prefix,
        take_while(8usize, is_conso_numeric).context(StrContext::Expected(
            StrContextValue::Description("Eight consonant or numeric characters"),
        )),
        one_of(b'0'..=b'9').context(StrContext::Expected(StrContextValue::Description(
            "Check digit",
        ))),
    )
        .recognize()
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
