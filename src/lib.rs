extern crate rand;
extern crate regex;

pub mod parse;
pub mod array;
pub mod boolean;
pub mod random;
pub mod sorting;
pub mod string;

pub enum RoundingMode{
    Trunc,Round,Ceil,Floor
}

pub enum SortingAlgorithmn{
    Bubble,Quick
}
