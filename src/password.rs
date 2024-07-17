use crate::error::TypeError;
use derive_more::Display;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Display, Serialize, Deserialize)]
pub struct RawPassword(String);

impl RawPassword {
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
