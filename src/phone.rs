use derive_more::Display;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{error::TypeError, CountryCode};

/// This crate provides functionality to parse and validate phone numbers with country codes.
///
/// # Example
///
/// ```
/// use custom_type::{PhoneNumber, CountryCode};
///
///     let phone_number = PhoneNumber::parse(CountryCode::USA, "1234567890").unwrap();
///     println!("{}", phone_number);
/// ```
///
/// # Features
///
/// - Parse and validate phone numbers with a specified country code.
/// - Custom error type `TypeError` for handling invalid phone numbers.
/// ### PhoneNumber : Parse `impl ToString` Into a Valid Phone Number
/// Provides a method to parse and validate phone numbers with specified country codes.
#[derive(Debug, PartialEq, Display, Serialize, Deserialize)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    /// Parses a given string into a phone number with the specified country code.
    ///
    /// A valid phone number must be between 10 to 15 digits long.
    ///
    /// # Arguments
    ///
    /// * `country_code` - The country code to be prepended to the phone number.
    /// * `phone_number` - A string slice that holds the phone number to be parsed.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` if the phone number meets the criteria.
    /// * `Err(TypeError::ParseError)` if the phone number is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use custom_type::{PhoneNumber, CountryCode};
    ///
    /// let phone_number = PhoneNumber::parse(CountryCode::USA, "1234567890");
    /// assert!(phone_number.is_ok());
    ///
    /// let invalid_phone_number = PhoneNumber::parse(CountryCode::USA, "12345");
    /// assert!(invalid_phone_number.is_err());
    /// ```
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

/// ======================================================================
/// ========================= Unit Test
/// ======================================================================
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
