use std::fmt;
use std::fmt::{
    Debug,Display,Formatter
};
use std::error::Error;

pub enum ParseError{
    InvalidNumber(String)
}

impl Error for ParseError{
    fn description(&self) -> &str{ "Invalid Number" }
}

impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self{
            &ParseError::InvalidNumber(ref i) => write!(f, "Invalid Number: {}", i)
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self{
            &ParseError::InvalidNumber(ref i) => write!(f, "Invalid Number: {}", i)
        }
    }
}
