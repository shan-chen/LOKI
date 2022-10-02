//! The error handling mod of LOKI

use std::error::Error;
use std::fmt;

#[derive(Debug)]
/// The struct of LOKI error
pub struct LOKIError {
    details: String,
}

impl LOKIError {
    /// construct a new loki error
    pub fn new(msg: &str) -> LOKIError {
        LOKIError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for LOKIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for LOKIError {
    fn description(&self) -> &str {
        &self.details
    }
}
