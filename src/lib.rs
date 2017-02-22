/*!

# Usage

This crate is [on crates.io](https://crates.io/crates/rustils/) and can be
used by adding `rustils` to your dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
rustils = "0.0.8"
```

and this to your crate root:

```rust
extern crate rustils;
```

# Examples

```
use rustils::StringUtils;

let text = "你好。How are you?";

//Rust function
assert_eq!(text.find('好'), Some(3));

//rustils functions
assert_eq!(text.find_char_opt('好'), Some(1));
assert_eq!(text.find_char('好'), 1);
```

```
use rustils::StringUtils;

let text1 = &mut String::from("你好。How are you?");

//Rust function
assert_eq!(text1.remove(3), '好');
assert_eq!(text1, "你。How are you?");

let text3 = String::from("你好。How are you?");
let text4 = &mut String::from("你好。How are you?");
let regex = r"[aeiou]+|[好]+";

//rustils functions
assert_eq!(text3.remove_regex(regex), String::from("你。How are you?"));
assert_eq!(text3.remove_all_regex(regex), String::from("你。Hw r y?"));

assert_eq!(true, text4.remove_regex_mut(regex));
assert_eq!(text4, "你。How are you?");

assert_eq!(true, text4.remove_all_regex_mut(regex));
assert_eq!(text4, "你。Hw r y?");
```
*/

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
    fn adv_has_alpha(&self) -> (bool, Vec<bool>);
    fn adv_has_alphanumeric(&self) -> (bool, Vec<bool>);
    fn adv_has_alphanumeric_space(&self) -> (bool, Vec<bool>);
    fn adv_has_alpha_space(&self) -> (bool, Vec<bool>);
    fn adv_has_lowercase(&self) -> (bool, Vec<bool>);
    fn adv_has_numeric(&self) -> (bool, Vec<bool>);
    fn adv_has_uppercase(&self) -> (bool, Vec<bool>);
    fn adv_has_whitespace(&self) -> (bool, Vec<bool>);
    fn adv_is_alpha(&self) -> (bool, Vec<bool>);
    fn adv_is_alphanumeric(&self) -> (bool, Vec<bool>);
    fn adv_is_lowercase(&self) -> (bool, Vec<bool>);
    fn adv_is_numeric(&self) -> (bool, Vec<bool>);
    fn adv_is_uppercase(&self) -> (bool, Vec<bool>);
    fn adv_is_whitespace(&self) -> (bool, Vec<bool>);
    fn adv_remove_all_regex(&self, regex: &str)
        -> (bool, Vec<(usize, usize)>, String);
    fn adv_remove_regex(&self, regex: &str) -> (bool, usize, String);
    fn adv_starts_with(&self, search: &str) -> (bool, String);
    fn contains_all_chars(&self, search: &[char]) -> bool;
    fn contains_all_strs(&self, search: &[&str]) -> bool;
    fn contains_any_char(&self, search: &[char]) -> bool;
    fn contains_any_str(&self, search: &[&str]) -> bool;
    fn contains_none_char(&self, search: &[char]) -> bool;
    fn contains_none_str(&self, search: &[&str]) -> bool;
    fn cmp_ignore_case(&self, cmp: &str) -> bool;
    fn difference(&self, diff: &str) -> Vec<usize>;
    fn find_char(&self, search: char) -> usize;
    fn find_char_opt(&self, search: char) -> Option<usize>;
    fn has_alpha(&self) -> bool;
    fn has_alphanumeric(&self) -> bool;
    fn has_lowercase(&self) -> bool;
    fn has_numeric(&self) -> bool;
    fn has_uppercase(&self) -> bool;
    fn has_whitespace(&self) -> bool;
    fn is_alpha(&self) -> bool;
    fn is_alphanumeric(&self) -> bool;
    fn is_lowercase(&self) -> bool;
    fn is_numeric(&self) -> bool;
    fn is_uppercase(&self) -> bool;
    fn is_whitespace(&self) -> bool;
    fn peek(&self) -> char;
    fn peek_opt(&self) -> Option<char>;
    fn remove_all_regex(&self, regex: &str) -> String;
    fn remove_all_regex_mut(&mut self, regex: &str) -> bool;
    fn remove_regex(&self, regex: &str) -> String;
    fn remove_regex_mut(&mut self, regex: &str) -> bool;
    fn reverse(&self) -> String;
    fn reverse_mut(&mut self);
    fn reverse_str(&self) -> &'static str;
}

#[derive(Debug)]
enum CharProp { Alpha, AlphaNumeric, AlphaNumericSpace, AlphaSpace, Lower, Numeric, NumericSpace, Upper, Whitespace }

fn char_property(s: &str, prop: CharProp, logic: bool) -> (bool, Vec<bool>) {
    let mut b = logic;

    let mut c = (*s).chars();
    let mut vec = Vec::<bool>::new();

    loop {
        let n = c.next();
        if n == None { break; }
        else {
            let nu = n.unwrap();
            println!("{:?}", prop);
            let temp = match prop {
                CharProp::Alpha => nu.is_alphabetic(),
                CharProp::AlphaNumeric => nu.is_alphanumeric(),
                CharProp::AlphaNumericSpace => nu.is_alphanumeric() || nu.is_whitespace(),
                CharProp::AlphaSpace => nu.is_alphabetic() || nu.is_whitespace(),
                CharProp::Lower => nu.is_lowercase(),
                CharProp::Numeric => nu.is_numeric(),
                CharProp::NumericSpace => nu.is_numeric() || nu.is_whitespace(),
                CharProp::Upper => nu.is_uppercase(),
                CharProp::Whitespace => nu.is_whitespace()
            };

            println!("{:?} {:?}", temp, b);

            if logic { b &= temp; }
            else { b |= temp; }

            vec.push(temp);
        }
    }

    (b, vec)
}

fn has_char_property(s: &str, prop: CharProp) -> (bool, Vec<bool>){
    char_property(s, prop, false)
}

fn is_char_property(s: &str, prop: CharProp) -> (bool, Vec<bool>) {
    char_property(s, prop, true)
}
