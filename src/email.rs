use crate::error::TypeError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;

/// ### Parse String To Valid Email
/// Call the `parse()` method to parse `impl ToString` into a valid email.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Email(String);

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Email {
    pub fn parse(email: impl ToString) -> Result<Self, TypeError> {
        // Normalize email
        let email = email.to_string().to_lowercase();

        // Email pattern on regular expression
        let email_regex =
            Regex::new(r"(?i)^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$")
                .unwrap();

        // Validate email using regular expression
        if email_regex.is_match(&email) {
            Ok(Self(email))
        } else {
            Err(TypeError::ParseError(
                "unable to parse email, invalid email.".to_string(),
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
    fn test_valid_emails() {
        assert_eq!(
            Email::parse("example@example.com"),
            Ok(Email("example@example.com".to_string()))
        );
        assert_eq!(
            Email::parse("user.name+tag@example.co.id"),
            Ok(Email("user.name+tag@example.co.id".to_string()))
        );
        assert_eq!(
            Email::parse("user_name@example.co.uk"),
            Ok(Email("user_name@example.co.uk".to_string()))
        );
    }

    #[test]
    fn test_invalid_emails() {
        assert_eq!(
            Email::parse("plainaddress"),
            Err(TypeError::ParseError(
                "unable to parse email, invalid email.".to_string()
            ))
        );
        assert_eq!(
            Email::parse("@missingusername.com"),
            Err(TypeError::ParseError(
                "unable to parse email, invalid email.".to_string()
            ))
        );
        assert_eq!(
            Email::parse("username@.com"),
            Err(TypeError::ParseError(
                "unable to parse email, invalid email.".to_string()
            ))
        );
        assert_eq!(
            Email::parse("username@.com."),
            Err(TypeError::ParseError(
                "unable to parse email, invalid email.".to_string()
            ))
        );
        assert_eq!(
            Email::parse("username@-ex@ample.com"),
            Err(TypeError::ParseError(
                "unable to parse email, invalid email.".to_string()
            ))
        );
        assert_eq!(
            Email::parse("username@example..com"),
            Err(TypeError::ParseError(
                "unable to parse email, invalid email.".to_string()
            ))
        );
    }
}
