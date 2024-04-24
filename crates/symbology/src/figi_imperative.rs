use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Figi(pub String);

#[derive(Debug)]
pub enum FigiParseError {
    // 12 chars only
    InvalidLength,
    // Consonants/digits or 'G' at pos 3
    InvalidFormat,
    // BS, BM, GG, GB, GH, KY, VG
    InvalidComponent,
    // Checksum integrity
    InvalidChecksum,
}

impl FromStr for Figi {
    type Err = FigiParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valid_chars = "0123456789BCDFGHJKLMNPQRSTVWXYZ";
        // Length must be 12
        if s.len() != 12 {
            return Err(FigiParseError::InvalidLength);
        }
        // Each character must be valid
        if !s.chars().all(|c| valid_chars.contains(c)) {
            return Err(FigiParseError::InvalidFormat);
        }
        // Restricted prefixes
        match &s[0..2] {
            "BS" | "BM" | "GG" | "GB" | "VG" => return Err(FigiParseError::InvalidComponent),
            _ => {}
        }
        // Third character must be 'G'
        if &s[2..3] != "G" {
            return Err(FigiParseError::InvalidComponent);
        }
        // Last character must be a digit and ignore checksum for now
        if !s.chars().last().unwrap().is_digit(10) {
            return Err(FigiParseError::InvalidChecksum);
        }
        Ok(Self(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_figi() {
        let figi_str = "BBG000B9XVV8"; // Example FIGI (fabricated for this test)
        assert!(Figi::from_str(figi_str).is_ok());
    }

    #[test]
    fn test_invalid_length() {
        let figi_str = "BBG00B9XVV8"; // Missing one character
        assert!(matches!(
            Figi::from_str(figi_str),
            Err(FigiParseError::InvalidLength)
        ));
    }

    #[test]
    fn test_invalid_characters() {
        let figi_str = "BBG00B9XV?V8"; // Contains an invalid character
        assert!(matches!(
            Figi::from_str(figi_str),
            Err(FigiParseError::InvalidFormat)
        ));
    }

    #[test]
    fn test_invalid_prefix() {
        let figi_str = "BSG000B9XVV8"; // Invalid prefix
        assert!(matches!(
            Figi::from_str(figi_str),
            Err(FigiParseError::InvalidComponent)
        ));
    }

    #[test]
    fn test_missing_g_in_third_place() {
        let figi_str = "BBB000B9XVV8"; // 'G' missing at position 3
        assert!(matches!(
            Figi::from_str(figi_str),
            Err(FigiParseError::InvalidComponent)
        ));
    }

    #[test]
    fn test_invalid_checksum_digit() {
        let figi_str = "BBG000B9XVVX"; // Last character is not a digit
        assert!(matches!(
            Figi::from_str(figi_str),
            Err(FigiParseError::InvalidChecksum)
        ));
    }
}
