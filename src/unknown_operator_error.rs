use std::{error::Error, fmt};

//pub type Result<T> = std::result::Result<T, UnknownOperatorError>;

#[derive(Debug, Clone)]
pub struct UnknownOperatorError;

impl Error for UnknownOperatorError {}

impl std::fmt::Display for UnknownOperatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid calculator operator")
    }
}
