//! This crate provides utilities for parsing and validating various types of data such as emails, passwords, and phone numbers with country codes.
//!
//! # Example
//!
//! ```
//! use your_crate_name::{Email, PhoneNumber, RawPassword, CountryCode};
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

pub use country_code::CountryCode;
pub use email::Email;
pub use password::RawPassword;
pub use phone::PhoneNumber;

/// ======================================================================
/// ========================= Integrated Test
/// ======================================================================
#[cfg(test)]
mod tests {

    #[test]
    fn test_integration() {
        todo!()
        // TODO!
    }
}
