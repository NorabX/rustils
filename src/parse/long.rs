// <editor-fold> # Uses

use error::ParseError;
use RoundingMode;
use RoundingMode::*;
// </editor-fold>

// <editor-fold> # Traits

pub trait ToI64 {

    fn to_i64_res(self)
        -> ParseResultI64;

    fn to_i64(self)
        -> i64;
}

pub trait ToI64RM {

    fn to_i64_rm_res(self, rm: RoundingMode)
        -> ParseResultI64;

    fn to_i64_rm(self, rm: RoundingMode)
        -> i64;
}
// </editor-fold>

// <editor-fold> # Functions

// <editor-fold> ## bool

pub fn bool_to_i64_res(a: bool)
    -> ParseResultI64 {

    if a { Ok(1) } else { Ok(0) }
}

pub fn bool_to_i64(a: bool)
    -> i64 {

    if a { 1 } else { 0 }
}
// </editor-fold>

// <editor-fold> ## 32

pub fn f32_to_i64_res(a: f32)
    -> ParseResultI64 {

    f32_to_i64_rm_res(a, Trunc)
}

pub fn f32_to_i64(a: f32)
    -> i64 {

    f32_to_i64_rm(a, Trunc)
}

pub fn f32_to_i64_rm_res(a: f32, rm: RoundingMode)
    -> ParseResultI64 {

    let min = -16777215_f32;
    let max = 16777215_f32;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < min || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as i64) }
}

pub fn f32_to_i64_rm(a: f32, rm: RoundingMode)
    -> i64 {

    match a.to_i64_rm_res(rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## 64

pub fn u64_to_i64_res(a: u64)
    -> ParseResultI64 {

    let max = i64::max_value() as u64;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i64) }
}

pub fn u64_to_i64(a: u64)
    -> i64 {

    match u64_to_i64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn f64_to_i64_res(a: f64)
    -> ParseResultI64 {

    f64_to_i64_rm_res(a, Trunc)
}

pub fn f64_to_i64(a: f64)
    -> i64 {

    f64_to_i64_rm(a, Trunc)
}

pub fn f64_to_i64_rm_res(a: f64, rm: RoundingMode)
    -> ParseResultI64 {

    let min = -9007199254740991_f64;
    let max = 9007199254740991_f64;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < min || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as i64) }
}

pub fn f64_to_i64_rm(a: f64, rm: RoundingMode)
    -> i64 {

    match f64_to_i64_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## size

pub fn usize_to_i64_res(a: usize)
    -> ParseResultI64 {

    let max = i64::max_value() as usize;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i64) }
}

pub fn usize_to_i64(a: usize)
    -> i64 {

    match usize_to_i64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## string

pub fn string_to_i64_res(a: String)
    -> ParseResultI64 {

    match a.parse::<i64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

pub fn string_to_i64(a: String)
    -> i64 {

    match string_to_i64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn str_to_i64_res(a: &str)
    -> ParseResultI64 {

    match a.parse::<i64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

pub fn str_to_i64(a: &str)
    -> i64 {

    match str_to_i64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// </editor-fold>

// <editor-fold> # Types

pub type ParseResultI64 = Result<i64, ParseError>;
// </editor-fold>
