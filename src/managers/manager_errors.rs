use std::error;
use std::fmt;

#[derive(Debug)]
pub enum BikesManagerError {
    RepositoryError(String),
    Other(String),
}

impl fmt::Display for BikesManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BikesManagerError::RepositoryError(s) => write!(f, "Repo Error: {}", s),
            BikesManagerError::Other(s) => write!(f, "Other Error!: {}", s),
        }
    }
}

impl error::Error for BikesManagerError {}
