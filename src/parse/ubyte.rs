// <editor-fold> # Uses

use error::ParseError;
use RoundingMode;
use RoundingMode::*;
// </editor-fold>

// <editor-fold> # Traits

pub trait ToU8 {

    fn to_u8_res(self)
        -> ParseResultU8;

    fn to_u8(self)
        -> u8;
}

pub trait ToU8RM {

    fn to_u8_rm_res(self, rm: RoundingMode)
        -> ParseResultU8;

    fn to_u8_rm(self, rm: RoundingMode)
        -> u8;
}
// </editor-fold>

// <editor-fold> # Functions

// <editor-fold> ## bool

pub fn bool_to_u8_res(a: bool)
    -> ParseResultU8 {

    if a { Ok(1) } else { Ok(0) }
}

pub fn bool_to_u8(a: bool)
    -> u8 {

    if a { 1 } else { 0 }
}
// </editor-fold>

// <editor-fold> ## 8

pub fn i8_to_u8_res(a: i8)
    -> ParseResultU8 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u8) }
}

pub fn i8_to_u8(a: i8)
    -> u8 {

    match i8_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## 16

pub fn i16_to_u8_res(a: i16)
    -> ParseResultU8 {

    let max = u8::max_value() as i16;

    if a < 0 || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u8) }
}

pub fn i16_to_u8(a: i16)
    -> u8 {

    match i16_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn u16_to_u8_res(a: u16)
    -> ParseResultU8 {

    let max = u8::max_value() as u16;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u8) }
}

pub fn u16_to_u8(a: u16)
    -> u8 {

    match u16_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## f32

pub fn i32_to_u8_res(a: i32)
    -> ParseResultU8 {

    let max = u8::max_value() as i32;

    if a < 0 || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u8) }
}

pub fn i32_to_u8(a: i32)
    -> u8 {

    match i32_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn u32_to_u8_res(a: u32)
    -> ParseResultU8 {

    let max = u8::max_value() as u32;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u8) }
}

pub fn u32_to_u8(a: u32)
    -> u8 {

    match u32_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn f32_to_u8_res(a: f32)
    -> ParseResultU8{

    f32_to_u8_rm_res(a, Trunc)
}

pub fn f32_to_u8(a: f32)
    -> u8 {

    f32_to_u8_rm(a, Trunc)
}

pub fn f32_to_u8_rm_res(a: f32, rm: RoundingMode)
    -> ParseResultU8 {

    let max = u8::max_value() as f32;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < 0.0 || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as u8) }
}

pub fn f32_to_u8_rm(a: f32, rm: RoundingMode)
    -> u8 {

    match f32_to_u8_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## f64

pub fn i64_to_u8_res(a: i64)
    -> ParseResultU8 {

    let max = u8::max_value() as i64;

    if a < 0 || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u8) }
}

pub fn i64_to_u8(a: i64)
    -> u8 {

    match i64_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn u64_to_u8_res(a: u64)
    -> ParseResultU8 {

    let max = u8::max_value() as u64;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u8) }
}

pub fn u64_to_u8(a: u64)
    -> u8 {

    match u64_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn f64_to_u8_res(a: f64)
    -> ParseResultU8{

    f64_to_u8_rm_res(a, Trunc)
}

pub fn f64_to_u8(a: f64)
    -> u8 {

    f64_to_u8_rm(a, Trunc)
}

pub fn f64_to_u8_rm_res(a: f64, rm: RoundingMode)
    -> ParseResultU8 {

    let max = u8::max_value() as f64;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < 0.0 || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as u8) }
}

pub fn f64_to_u8_rm(a: f64, rm: RoundingMode)
    -> u8 {

    match f64_to_u8_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## size

pub fn usize_to_u8_res(a: usize)
    -> ParseResultU8 {

    let max = u8::max_value() as usize;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u8) }
}

pub fn usize_to_u8(a: usize)
    -> u8 {

    match usize_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn isize_to_u8_res(a: isize)
    -> ParseResultU8 {

    let max = u8::max_value() as isize;

    if a < 0 || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u8) }
}

pub fn isize_to_u8(a: isize)
    -> u8 {

    match isize_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## string

pub fn string_to_u8_res(a: String)
    -> ParseResultU8 {

    match a.parse::<u8>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

pub fn string_to_u8(a: String)
    -> u8 {

    match string_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn str_to_u8_res(a: &str)
    -> ParseResultU8 {

    match a.parse::<u8>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

pub fn str_to_u8(a: &str)
    -> u8 {

    match str_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// </editor-fold>

// <editor-fold> # Types

pub type ParseResultU8 = Result<u8, ParseError>;
// </editor-fold>
