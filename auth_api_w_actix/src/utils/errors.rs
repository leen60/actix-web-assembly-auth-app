use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ErrorResponse {
    details: String
}

impl ErrorResponse {
    pub fn new(msg: &str) -> ErrorResponse {
        ErrorResponse{details: msg.to_string()}
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for ErrorResponse {
    fn description(&self) -> &str {
        &self.details
    }
}