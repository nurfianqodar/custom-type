use crate::error::TypeError;
use derive_more::Display;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// ### Url: Parse `impl ToString` into a valid URL
/// Call the `parse()` method to parse `impl ToString` into a valid URL.
#[derive(Debug, PartialEq, Display, Serialize, Deserialize)]
pub struct Url(String);

impl Url {
    /// Parses a string into a valid URL.
    ///
    /// # Arguments
    ///
    /// * `url` - A string that implements `ToString`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Url)` if the input string is a valid URL, otherwise returns `Err(TypeError)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use custom_type::Url;
    ///
    /// let valid_url = Url::parse("https://example.com");
    /// assert!(valid_url.is_ok());
    ///
    /// let invalid_url = Url::parse("example.com");
    /// assert!(invalid_url.is_err());
    /// ```
    pub fn parse(url: impl ToString) -> Result<Self, TypeError> {
        let url = url.to_string();
        let url_regex = Regex::new(r"^(https?|ftp)://[^\s/$.?#]+\.[^\s]*$").unwrap();

        if url_regex.is_match(&url) {
            Ok(Self(url))
        } else {
            Err(TypeError::ParseError(
                "unable to parse URL, invalid URL.".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_urls() {
        assert_eq!(
            Url::parse("https://example.com"),
            Ok(Url("https://example.com".to_string()))
        );
        assert_eq!(
            Url::parse("http://example.com"),
            Ok(Url("http://example.com".to_string()))
        );
        assert_eq!(
            Url::parse("ftp://example.com"),
            Ok(Url("ftp://example.com".to_string()))
        );
    }

    #[test]
    fn test_invalid_urls() {
        assert_eq!(
            Url::parse("example.com"),
            Err(TypeError::ParseError(
                "unable to parse URL, invalid URL.".to_string()
            ))
        );
        assert_eq!(
            Url::parse("http://example"),
            Err(TypeError::ParseError(
                "unable to parse URL, invalid URL.".to_string()
            ))
        );
        assert_eq!(
            Url::parse("ftp://example.com/path with spaces"),
            Err(TypeError::ParseError(
                "unable to parse URL, invalid URL.".to_string()
            ))
        );
    }
}
