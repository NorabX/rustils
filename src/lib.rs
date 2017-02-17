extern crate rand;
extern crate regex;
extern crate core;

pub mod parse;
pub mod boolean;
pub mod random;
pub mod sorting;

#[doc(hidden)] pub mod array;
#[doc(hidden)] pub mod string;

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

pub trait ArrayUtils<T> {
    fn swaping(&mut self, a: usize, b: usize) -> bool;
    fn shuffle(&mut self);
    fn shuffle_seed(&mut self, seed: &[usize]);
    fn index_of(&self, search: &T) -> usize;
}

pub trait StringUtils {
    fn adv_contains_all_chars(&self, search: &[char])
        -> (bool, Vec<usize>, Vec<char>);
    fn adv_contains_all_strs(&self, search: &[&str])
        -> (bool,Vec<usize>,Vec<String>);
    fn adv_contains_any_char(&self, search: &[char]) -> (bool, usize, char);
    fn adv_contains_any_str(&self, search: &[&str]) -> (bool, usize, String);
    fn adv_contains_none_char(&self, search: &[char]) -> (bool, usize, char);
    fn adv_contains_none_str(&self, search: &[&str]) -> (bool, usize, String);
    fn adv_ends_with(&self, search: &str) -> (bool, String);
    fn adv_starts_with(&self, search: &str) -> (bool, String);
    fn contains_all_chars(&self, search: &[char]) -> bool;
    fn contains_all_strs(&self, search: &[&str]) -> bool;
    fn contains_any_char(&self, search: &[char]) -> bool;
    fn contains_any_str(&self, search: &[&str]) -> bool;
    fn contains_none_char(&self, search: &[char]) -> bool;
    fn contains_none_str(&self, search: &[&str]) -> bool;
    fn cmp_ignore_case(&self, cmp: &str) -> bool;
    fn peek(&self) -> char;
    fn peek_opt(&self) -> Option<char>;
    fn reverse_mut(&mut self);
    fn reverse_str(&self) -> &'static str;
    fn reverse(&self) -> String;
}
