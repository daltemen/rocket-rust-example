use std::error;
use std::fmt;

#[derive(Debug)]
pub enum BikesError {
    DBError(String),
    Other(String),
}

impl fmt::Display for BikesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BikesError::DBError(s) => write!(f, "DB Error: {}", s),
            BikesError::Other(s) => write!(f, "Other Error!: {}", s),
        }
    }
}

impl error::Error for BikesError {}
