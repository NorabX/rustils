extern crate rand;
extern crate regex;
extern crate core;

pub mod parse;
pub mod array;
pub mod boolean;
pub mod random;
pub mod sorting;
pub mod string;
pub mod math;

use std::fmt;
use std::fmt::{ Debug, Display, Formatter };
use std::error::Error;

pub enum RoundingMode{
    Trunc,Round,Ceil,Floor
}

pub enum SortingAlgorithmn{
    Bubble,Quick
}

#[derive(PartialEq)]
pub enum ParseError {
    InvalidNumber(String)
}

impl Error for ParseError{
    fn description(&self) -> &'static str { "Invalid Number" }
    fn cause(&self) -> Option<&Error> { None }
}

impl Debug for ParseError {
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
