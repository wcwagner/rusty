use std::str::FromStr;
use winnow::combinator::{alt, seq};
use winnow::prelude::*;
#[derive(Debug, Clone, PartialEq)]
pub enum Scheme {
    BLP,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Provider {
    RefData,
    MktData,
    MktBar,
}

// https://data.bloomberglp.com/professional/sites/10/2017/03/BLPAPI-Core-User-Guide.pdf
// services are all in the form “//blp/<servicename>”.
#[derive(Debug, PartialEq)]
pub struct Service {
    scheme: Scheme,
    provider: Provider,
}

fn scheme<'s>(i: &mut &'s str) -> PResult<Scheme> {
    "blp".value(Scheme::BLP).parse_next(i)
}

fn provider<'s>(i: &mut &'s str) -> PResult<Provider> {
    alt((
        "refdata".value(Provider::RefData),
        "mktdata".value(Provider::MktData),
        "mktbar".value(Provider::MktBar),
    ))
    .parse_next(i)
}

fn service<'s>(i: &mut &'s str) -> PResult<Service> {
    seq! {
        Service{
            _: "//",
            scheme: scheme,
            _: "/",
            provider: provider
        }
    }
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_refdata() {
        let mut input = "//blp/refdata";
        let expected = Service {
            scheme: Scheme::BLP,
            provider: Provider::RefData,
        };
        assert_eq!(service(&mut input), Ok(expected));
    }

    #[test]
    fn test_service_mktdata() {
        let mut input = "//blp/mktdata";
        let expected = Service {
            scheme: Scheme::BLP,
            provider: Provider::MktData,
        };
        assert_eq!(service(&mut input), Ok(expected));
    }

    #[test]
    fn test_service_mktbar() {
        let mut input = "//blp/mktbar";
        let expected = Service {
            scheme: Scheme::BLP,
            provider: Provider::MktBar,
        };
        assert_eq!(service(&mut input), Ok(expected));
    }

    #[test]
    fn test_service_invalid_scheme() {
        let mut input = "///blp/refdata";
        assert!(
            service(&mut input).is_err(),
            "Invalid scheme should not be parsed successfully."
        );
    }

    #[test]
    fn test_service_invalid_provider() {
        let mut input = "//blp/unknown";
        assert!(
            service(&mut input).is_err(),
            "Unknown provider should not be parsed successfully."
        );
    }

    #[test]
    fn test_service_missing_scheme() {
        let mut input = "refdata";
        assert!(
            service(&mut input).is_err(),
            "Missing scheme should not be parsed successfully."
        );
    }
}
