// <editor-fold> # Uses

use error::ParseError;
use RoundingMode;
use RoundingMode::*;
// </editor-fold>

// <editor-fold> # Traits

pub trait ToUsize {

    fn to_usize_res(self)
        -> ParseResultUsize;

    fn to_usize(self)
        -> usize;
}

pub trait ToUsizeRM {

    fn to_usize_rm_res(self, rm: RoundingMode)
        -> ParseResultUsize;

    fn to_usize_rm(self, rm: RoundingMode)
        -> usize;
}
// </editor-fold>

// <editor-fold> # Functions

// <editor-fold> ## bool

pub fn bool_to_usize_res(a: bool)
    -> ParseResultUsize {

    if a { Ok(1) } else { Ok(0) }
}

pub fn bool_to_usize(a: bool)
    -> usize {

    if a { 1 } else { 0 }
}
// </editor-fold>

// <editor-fold> ## 8

pub fn i8_to_usize_res(a: i8)
    -> ParseResultUsize {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as usize) }
}

pub fn i8_to_usize(a: i8)
    -> usize {

    match i8_to_usize_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## 16

pub fn i16_to_usize_res(a: i16)
    -> ParseResultUsize {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as usize) }
}

pub fn i16_to_usize(a: i16)
    -> usize {

    match i16_to_usize_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## 32

pub fn i32_to_usize_res(a: i32)
    -> ParseResultUsize {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as usize) }
}

pub fn i32_to_usize(a: i32)
    -> usize {

    match i32_to_usize_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn f32_to_usize_res(a: f32)
    -> ParseResultUsize {

    f32_to_usize_rm_res(a, Trunc)
}

pub fn f32_to_usize(a: f32)
    -> usize {

    f32_to_usize_rm(a, Trunc)
}

pub fn f32_to_usize_rm_res(a: f32, rm: RoundingMode)
    -> ParseResultUsize {

    let max = 16777215_f32;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < 0.0 || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as usize) }
}

pub fn f32_to_usize_rm(a: f32, rm: RoundingMode)
    -> usize {

    match f32_to_usize_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## 64

pub fn f64_to_usize_res(a: f64)
    -> ParseResultUsize {

    f64_to_usize_rm_res(a, Trunc)
}

pub fn f64_to_usize(a: f64)
    -> usize {

    f64_to_usize_rm(a, Trunc)
}

pub fn f64_to_usize_rm_res(a: f64, rm: RoundingMode)
    -> ParseResultUsize {

    let max = 9007199254740991_f64;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < 0.0 || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as usize) }
}

pub fn f64_to_usize_rm(a: f64, rm: RoundingMode)
    -> usize {

    match f64_to_usize_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## size

pub fn isize_to_usize_res(a: isize)
    -> ParseResultUsize {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as usize) }
}

pub fn isize_to_usize(a: isize)
    -> usize {

    match isize_to_usize_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## string

pub fn string_to_usize_res(a: String)
    -> ParseResultUsize {

    match a.parse::<usize>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

pub fn string_to_usize(a: String)
    -> usize {

    match string_to_usize_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn str_to_usize_res(a: &str)
    -> ParseResultUsize {

    match a.parse::<usize>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

pub fn str_to_usize(a: &str)
    -> usize {

    match str_to_usize_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// </editor-fold>

// <editor-fold> # Types

pub type ParseResultUsize = Result<usize, ParseError>;
// </editor-fold>
