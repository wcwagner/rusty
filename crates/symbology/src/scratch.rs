use ::winnow::ascii::{digit0, digit1, multispace0 as ws};
use winnow::combinator::{alt, delimited, opt, preceded, repeat, separated, terminated};
use winnow::ascii::Caseless;
use winnow::prelude::*;
use winnow::token::*;
use winnow::PResult;

#[derive(Debug, PartialEq)]
pub enum Factor {
    M,
    MM,
    MMM,
    MMMM,
    P,
}

#[derive(Debug, PartialEq)]
pub struct Qty {
    value: f64,
    factor: Option<Factor>,
}

impl std::str::FromStr for Qty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_quantity.parse(s).map_err(|e| e.to_string())
    }
}

pub fn prefix<'s>(input: &mut &'s str) -> PResult<&'s str> {
    // " 1", "-1", "(1", "$1"
    take_while(0.., ('$', '(', ' ', '-')).parse_next(input)
}

/// Factor multiplier applied to the numeric input quantity.
///
/// # Example
/// "1M" -> 1,000
/// "1MM" -> 1,000,000
/// "1.5M" -> 1,500
/// "1P" -> "1"
/// "1000P" -> "1,000"

pub fn multiplier(input: &mut &str) -> PResult<Option<Factor>> {
    opt(alt((
        "M".map(|_| Factor::M),
        "MM".map(|_| Factor::MM),
        "MMM".map(|_| Factor::MMM),
        "MMMM".map(|_| Factor::MMMM),
        'P'.map(|_| Factor::P),
    )))
    .parse_next(input)
}

pub fn parse_quantity(input: &mut &str) -> PResult<Qty> {
    let (number, factor) = delimited(prefix, (digit1, multiplier), opt(')')).parse_next(input)?;
    let value = number.parse::<f64>().unwrap();
    Ok(Qty { value, factor })
}

fn trivia<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_while(0.., ('+', '-', '(', ')', '$', ' ', '\t')).parse_next(input)
}


fn

fn number<'s>(input: &mut &'s str) -> PResult<&'s str> {
    delimited(trivia, , trivia)
}
#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    use super::*;

    #[test]
    fn test_trivia() {
        assert_eq!(trivia.parse_peek("(123)"), Ok(("123)", "(")));
    }
    #[test]
    fn test_valid() {
        assert_eq!(
            "100".parse(),
            Ok(Qty {
                value: 100.0,
                factor: None
            })
        );

        assert_eq!(
            "(100)".parse(),
            Ok(Qty {
                value: 100.0,
                factor: None
            })
        );

        assert_eq!(
            "   ($100)".parse(),
            Ok(Qty {
                value: 100.0,
                factor: None
            })
        );

        assert_eq!(
            "1MM".parse(),
            Ok(Qty {
                value: 1.0,
                factor: Some(Factor::MM)
            })
        )
    }
}
