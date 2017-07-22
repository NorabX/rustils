// <editor-fold> # Uses

use std::fmt;
use std::fmt::{ Display, Formatter };
use std::error::Error;
use error::*;
// </editor-fold>

// <editor-fold> # Impl

// <editor-fold> ## ParseError

impl Error for ParseError{
    fn description(&self) -> &'static str {
        match self{
            &ParseError::InvalidNumber(_) => "Invalid Number",
            &ParseError::InvalidString(_) => "Invalid String"
        }
    }

    fn cause(&self) -> Option<&Error> { None }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self{
            &ParseError::InvalidNumber(ref i) => write!(f, "Invalid Number: {}", i),
            &ParseError::InvalidString(ref i) => write!(f, "Invalid String: {}", i)
        }
    }
}
// </editor-fold>

// <editor-fold> ## ArithmeticError

impl Error for ArithmeticError {
    fn description(&self) -> &'static str {
        match self{
            &ArithmeticError::DivideByZero => "DivideByZero"
        }
    }

    fn cause(&self) -> Option<&Error> { None }
}

impl Display for ArithmeticError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self{
            &ArithmeticError::DivideByZero => write!(f, "DivideByZero")
        }
    }
}
// </editor-fold>

// </editor-fold>
