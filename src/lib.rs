//! This crate provides utilities for parsing and validating various types of data such as emails, passwords, and phone numbers with country codes.
//!
//! # Example
//!
//! ```
//! use custom_type::{Email, PhoneNumber, RawPassword, CountryCode};
//!
//! let email = Email::parse("example@example.com").unwrap();
//! println!("{}", email);
//!
//! let phone_number = PhoneNumber::parse(CountryCode::USA, "1234567890").unwrap();
//! println!("{}", phone_number);
//!
//! let password = RawPassword::parse_strict("Valid123!").unwrap();
//! println!("{}", password);
//! ```
//!
//! # Features
//!
//! - Parse and validate email addresses.
//! - Parse and validate phone numbers with country codes.
//! - Parse and validate passwords with different strength levels.

mod country_code;
mod email;
pub mod error;
mod password;
mod phone;
mod url;

pub use country_code::CountryCode;
pub use email::Email;
pub use password::RawPassword;
pub use phone::PhoneNumber;
pub use url::Url;

#[cfg(test)]
mod integration_test {
    use crate::{Email, RawPassword, Url};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct User {
        username: String,
        email: Email,
        password: RawPassword,
        socmed_url: Url,
    }

    struct RegisterUserRequest {
        username: String,
        email: String,
        password: String,
        socmed_url: String,
    }

    impl From<RegisterUserRequest> for User {
        fn from(value: RegisterUserRequest) -> Self {
            let email = Email::parse(&value.email)
                .unwrap_or_else(|e| panic!("Failed to parse email: {}", e));
            let password = RawPassword::parse_strict(&value.password)
                .unwrap_or_else(|e| panic!("Failed to parse password: {}", e));
            let socmed_url = Url::parse(&value.socmed_url)
                .unwrap_or_else(|e| panic!("Failed to parse URL: {}", e));

            Self {
                username: value.username,
                email,
                password,
                socmed_url,
            }
        }
    }

    #[test]
    fn test_user() {
        let req = RegisterUserRequest {
            email: "example@example.com".to_string(),
            password: "Valid123!".to_string(),
            socmed_url: "https://example.com/useridex".to_string(),
            username: "user123".to_string(),
        };

        let user = User::from(req);
        let json = serde_json::to_string_pretty(&user).unwrap();
        println!("{}", json);
    }
}
