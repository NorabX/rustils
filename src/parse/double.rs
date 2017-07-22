// <editor-fold> # Uses

use error::ParseError;
use RoundingMode;
// </editor-fold>

// <editor-fold> # Traits

pub trait ToF64 {

    fn to_f64_res(self)
        -> ParseResultF64;

    fn to_f64(self)
        -> f64;
}

pub trait ToF64RM {

    fn to_f64_rm_res(self, rm: RoundingMode)
        -> ParseResultF64;

    fn to_f64_rm(self, rm: RoundingMode)
        -> f64;
}
// </editor-fold>

// <editor-fold> # Functions

// <editor-fold> ## bool

pub fn bool_to_f64_res(a: bool)
    -> ParseResultF64 {

    if a { Ok(1.0) } else { Ok(0.0) }
}

pub fn bool_to_f64(a: bool)
    -> f64 {

    if a { 1.0 } else { 0.0 }
}
// </editor-fold>

// <editor-fold> ## string

pub fn string_to_f64_res(a: String)
    -> ParseResultF64 {

    match a.parse::<f64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

pub fn string_to_f64(a: String)
    -> f64 {

    match string_to_f64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn str_to_f64_res(a: &str)
    -> ParseResultF64 {

    match a.parse::<f64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

pub fn str_to_f64(a: &str)
    -> f64 {

    match str_to_f64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// </editor-fold>

// <editor-fold> # Types

pub type ParseResultF64 = Result<f64, ParseError>;
// </editor-fold>
