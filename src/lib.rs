extern crate rand;
extern crate regex;
extern crate core;

pub mod parse;
pub mod array;
pub mod boolean;
pub mod random;
pub mod sorting;
pub mod string;

use std::fmt;
use std::fmt::{ Display, Formatter };
use std::error::Error;

pub enum RoundingMode { Trunc, Round, Ceil, Floor }

pub enum SortingAlgorithmn { Bubble, Quick }

#[derive(PartialEq, Debug)]
pub enum ParseError {
    InvalidNumber(String),
    InvalidString(String)
}

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


#[derive(PartialEq, Debug)]
pub enum ArithmeticError { DivideByZero }

impl Error for ArithmeticError {
    fn description(&self) -> &'static str { "Invalid Number" }
    fn cause(&self) -> Option<&Error> { None }
}

impl Display for ArithmeticError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self{
            &ArithmeticError::DivideByZero => write!(f, "DivideByZero")
        }
    }
}
