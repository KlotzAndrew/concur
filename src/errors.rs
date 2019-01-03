use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CoodinationError {
    details: String
}

impl CoodinationError {
    pub fn new(msg: &str) -> CoodinationError {
        CoodinationError{details: msg.to_string()}
    }
}

impl fmt::Display for CoodinationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for CoodinationError {
    fn description(&self) -> &str {
        &self.details
    }
}
