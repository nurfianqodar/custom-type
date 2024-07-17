use core::fmt;
use std::error;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TypeError {
    ParseError(String),
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeError::ParseError(msg) => write!(f, "{}", msg),
        }
    }
}

impl error::Error for TypeError {
    fn description(&self) -> &str {
        match self {
            &TypeError::ParseError(_) => "parse error, invalid input!", // TODO! Better Error Description
        }
    }
}
