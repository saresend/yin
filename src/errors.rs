use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct UnknownValueError {}

impl fmt::Display for UnknownValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse sample for frequency")
    }
}

impl Error for UnknownValueError {}
