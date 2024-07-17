use derive_more::Display;
use regex::Regex;

use crate::error::TypeError;

#[derive(Debug, Display)]
pub struct Email(String);

impl Email {
    pub fn parse(email: impl ToString) -> Result<Self, TypeError> {
        let email_regex =
            Regex::new(r"(?i)^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$")
                .unwrap();
        let email = email.to_string().to_lowercase();
        let is_valid = email_regex.is_match(&email);
        if is_valid {
            Ok(Self(email))
        } else {
            Err(TypeError::ParseError(
                "unable to parse email, invalid email.".to_string(),
            ))
        }
    }
}
