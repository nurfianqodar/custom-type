use derive_more::Display;
use regex::Regex;

use crate::{error::TypeError, CountryCode};

#[derive(Debug, PartialEq, Display)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn parse(
        country_code: CountryCode,
        phone_number: impl ToString,
    ) -> Result<Self, TypeError> {
        let phone_number = phone_number.to_string();
        let phone_regex = Regex::new(r"^[0-9]{10,15}$").unwrap();

        if phone_regex.is_match(&phone_number) {
            Ok(Self(format!("{}{}", country_code, phone_number)))
        } else {
            Err(TypeError::ParseError(
                "unable to parse phone number, invalid phone number.".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_phone_number() {
        assert_eq!(
            PhoneNumber::parse(CountryCode::USA, "1234567890"),
            Ok(PhoneNumber("+11234567890".to_string()))
        );
        assert_eq!(
            PhoneNumber::parse(CountryCode::UK, "123456789012"),
            Ok(PhoneNumber("+44123456789012".to_string()))
        );
        assert_eq!(
            PhoneNumber::parse(CountryCode::IND, "1234567890"),
            Ok(PhoneNumber("+911234567890".to_string()))
        );
        assert_eq!(
            PhoneNumber::parse(CountryCode::AUS, "1234567890"),
            Ok(PhoneNumber("+611234567890".to_string()))
        );
    }

    #[test]
    fn test_invalid_phone_number() {
        assert_eq!(
            PhoneNumber::parse(CountryCode::USA, "12345"),
            Err(TypeError::ParseError(
                "unable to parse phone number, invalid phone number.".to_string()
            ))
        );
        assert_eq!(
            PhoneNumber::parse(CountryCode::UK, "phone123456"),
            Err(TypeError::ParseError(
                "unable to parse phone number, invalid phone number.".to_string()
            ))
        );
        assert_eq!(
            PhoneNumber::parse(CountryCode::IND, "123-456-7890"),
            Err(TypeError::ParseError(
                "unable to parse phone number, invalid phone number.".to_string()
            ))
        );
    }
}
