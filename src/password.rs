use crate::error::TypeError;
use derive_more::Display;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// This crate provides functionality to parse and validate raw passwords with different levels of strength.
///
/// # Example
///
/// ```
/// use custom_type::RawPassword;
///
/// let weak_password = RawPassword::parse_weak("weakpass").unwrap();
/// println!("Weak Password: {}", weak_password);
///
/// let medium_password = RawPassword::parse_medium("Medium123").unwrap();
/// println!("Medium Password: {}", medium_password);
///
/// let strict_password = RawPassword::parse_strict("Strong1!23").unwrap();
/// println!("Strong Password: {}", strict_password);
/// ```
///
/// - `parse_weak`: Parses a password and ensures it is at least 8 characters long.
/// - `parse_medium`: Parses a password and ensures it is at least 8 characters long, contains both letters and digits.
/// - `parse_strict`: Parses a password and ensures it is at least 8 characters long, contains uppercase, lowercase, digits, and special characters.
///
/// Each method returns a `Result<RawPassword, TypeError>` where `TypeError` indicates a parsing error if the password does not meet the criteria.
///
/// # Features
///
/// - Parse and validate passwords with different strength levels (weak, medium, strict).
/// - Custom error type `TypeError` for handling invalid passwords.
/// ### RawPassword : Parse `impl ToString` Into a Valid Password
/// Provides methods to parse and validate passwords with different strength criteria.
#[derive(Debug, PartialEq, Display, Serialize, Deserialize)]
pub struct RawPassword(String);

impl RawPassword {
    /// Parses a given string into a weak password.
    ///
    /// A weak password must be at least 8 characters long.
    ///
    /// # Arguments
    ///
    /// * `password` - A string slice that holds the password to be parsed.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` if the password meets the weak criteria.
    /// * `Err(TypeError::ParseError)` if the password is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use custom_type::RawPassword;
    ///
    /// let password = RawPassword::parse_weak("validpass");
    /// assert!(password.is_ok());
    ///
    /// let invalid_password = RawPassword::parse_weak("short");
    /// assert!(invalid_password.is_err());
    /// ```
    pub fn parse_weak(password: impl ToString) -> Result<Self, TypeError> {
        let password_str = password.to_string();
        if password_str.len() >= 8 {
            Ok(Self(password_str))
        } else {
            Err(TypeError::ParseError(String::from(
                "Weak password: must be at least 8 characters long",
            )))
        }
    }

    /// Parses a given string into a medium password.
    ///
    /// A medium password must be at least 8 characters long and contain both letters and digits.
    ///
    /// # Arguments
    ///
    /// * `password` - A string slice that holds the password to be parsed.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` if the password meets the medium criteria.
    /// * `Err(TypeError::ParseError)` if the password is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use custom_type::RawPassword;
    ///
    /// let password = RawPassword::parse_medium("valid123");
    /// assert!(password.is_ok());
    ///
    /// let invalid_password = RawPassword::parse_medium("noDigits");
    /// assert!(invalid_password.is_err());
    /// ```
    pub fn parse_medium(password: impl ToString) -> Result<Self, TypeError> {
        let password_str = password.to_string();
        let re_digit = Regex::new(r"\d").unwrap();
        let re_alpha = Regex::new(r"[a-zA-Z]").unwrap();
        if password_str.len() >= 8
            && re_digit.is_match(&password_str)
            && re_alpha.is_match(&password_str)
        {
            Ok(Self(password_str))
        } else {
            Err(TypeError::ParseError(String::from("Medium password: must be at least 8 characters long and contain both letters and digits")))
        }
    }

    /// Parses a given string into a strict password.
    ///
    /// A strict password must be at least 8 characters long and contain uppercase, lowercase, digits, and special characters.
    ///
    /// # Arguments
    ///
    /// * `password` - A string slice that holds the password to be parsed.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` if the password meets the strict criteria.
    /// * `Err(TypeError::ParseError)` if the password is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use custom_type::RawPassword;
    ///
    /// let password = RawPassword::parse_strict("Valid123!");
    /// assert!(password.is_ok());
    ///
    /// let invalid_password = RawPassword::parse_strict("NoDigits!");
    /// assert!(invalid_password.is_err());
    /// ```
    pub fn parse_strict(password: impl ToString) -> Result<Self, TypeError> {
        let password_str = password.to_string();
        let re_upper = Regex::new(r"[A-Z]").unwrap();
        let re_lower = Regex::new(r"[a-z]").unwrap();
        let re_digit = Regex::new(r"\d").unwrap();
        let re_special = Regex::new(r"[^a-zA-Z\d]").unwrap();
        if password_str.len() >= 8
            && re_upper.is_match(&password_str)
            && re_lower.is_match(&password_str)
            && re_digit.is_match(&password_str)
            && re_special.is_match(&password_str)
        {
            Ok(Self(password_str))
        } else {
            Err(TypeError::ParseError(String::from("Strict password: must be at least 8 characters long and contain uppercase, lowercase, digits, and special characters")))
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
    fn test_parse_weak() {
        assert_eq!(
            RawPassword::parse_weak("short".to_string()),
            Err(TypeError::ParseError(String::from(
                "Weak password: must be at least 8 characters long"
            )))
        );
        assert_eq!(
            RawPassword::parse_weak("validpass".to_string()),
            Ok(RawPassword("validpass".to_string()))
        );
    }

    #[test]
    fn test_parse_medium() {
        assert_eq!(
            RawPassword::parse_medium("short".to_string()),
            Err(TypeError::ParseError(String::from("Medium password: must be at least 8 characters long and contain both letters and digits")))
        );
        assert_eq!(
            RawPassword::parse_medium("noDigits".to_string()),
            Err(TypeError::ParseError(String::from("Medium password: must be at least 8 characters long and contain both letters and digits")))
        );
        assert_eq!(
            RawPassword::parse_medium("valid123".to_string()),
            Ok(RawPassword("valid123".to_string()))
        );
    }

    #[test]
    fn test_parse_strict() {
        assert_eq!(
            RawPassword::parse_strict("short".to_string()),
            Err(TypeError::ParseError(String::from("Strict password: must be at least 8 characters long and contain uppercase, lowercase, digits, and special characters")))
        );
        assert_eq!(
            RawPassword::parse_strict("NoDigits!".to_string()),
            Err(TypeError::ParseError(String::from("Strict password: must be at least 8 characters long and contain uppercase, lowercase, digits, and special characters")))
        );
        assert_eq!(
            RawPassword::parse_strict("noupper1!".to_string()),
            Err(TypeError::ParseError(String::from("Strict password: must be at least 8 characters long and contain uppercase, lowercase, digits, and special characters")))
        );
        assert_eq!(
            RawPassword::parse_strict("VALID123".to_string()),
            Err(TypeError::ParseError(String::from("Strict password: must be at least 8 characters long and contain uppercase, lowercase, digits, and special characters")))
        );
        assert_eq!(
            RawPassword::parse_strict("Valid123!".to_string()),
            Ok(RawPassword("Valid123!".to_string()))
        );
    }
}
