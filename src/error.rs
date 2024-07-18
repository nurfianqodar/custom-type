use core::fmt;
use serde::{Deserialize, Serialize};

/// This crate defines custom error types for handling different kinds of errors.
///
/// # Example
///
/// ```
/// use custom_type::TypeError;
///
/// let error = TypeError::ParseError("Invalid input".to_string());
/// println!("{}", error);
/// ```
///
/// # Features
///
/// - Custom error type `TypeError` for handling parse errors.
/// - Implements `fmt::Display` and `std::error::Error` for `TypeError`.
/// Enum representing different types of errors.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TypeError {
    /// Represents an error that occurs during parsing.
    ParseError(String),
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeError::ParseError(msg) => write!(f, "{}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::TypeError;

    #[test]
    fn test_parse_error_display() {
        let error = TypeError::ParseError("Invalid input".to_string());
        assert_eq!(format!("{}", error), "Invalid input");
    }
}
