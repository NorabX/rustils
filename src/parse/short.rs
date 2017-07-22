// <editor-fold> Uses

use error::ParseError;
use RoundingMode;
use RoundingMode::*;
// </editor-fold>

// <editor-fold> # Traits

pub trait ToI16 {

    fn to_i16_res(self)
        -> ParseResultI16;

    fn to_i16(self)
        -> i16;
}

pub trait ToI16RM {

    fn to_i16_rm_res(self, rm: RoundingMode)
        -> ParseResultI16;

    fn to_i16_rm(self, rm: RoundingMode)
        -> i16;
}
// </editor-fold>

// <editor-fold> # Functions

// <editor-fold> ## bool

pub fn bool_to_i16_res(a: bool)
    -> ParseResultI16 {

    if a { Ok(1) } else { Ok(0) }
}

pub fn bool_to_i16(a: bool)
    -> i16 {

    if a { 1 } else { 0 }
}

// </editor-fold>

// <editor-fold> ## 16

pub fn u16_to_i16_res(a: u16)
    -> ParseResultI16 {

    let max = i16::max_value() as u16;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i16) }
}

pub fn u16_to_i16(a: u16)
    -> i16 {

    match u16_to_i16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## 32

pub fn i32_to_i16_res(a: i32)
    -> ParseResultI16 {

    let min = i16::min_value() as i32;
    let max = i16::max_value() as i32;

    if a < min || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i16) }
}

pub fn i32_to_i16(a: i32)
    -> i16 {

    match i32_to_i16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn u32_to_i16_res(a: u32)
    -> ParseResultI16 {

    let max = i16::max_value() as u32;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i16) }
}

pub fn u32_to_i16(a: u32)
    -> i16 {

    match u32_to_i16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn f32_to_i16_res(a: f32)
    -> ParseResultI16 {

    f32_to_i16_rm_res(a, Trunc)
}

pub fn f32_to_i16(a: f32)
    -> i16 {

    f32_to_i16_rm(a, Trunc)
}

pub fn f32_to_i16_rm_res(a: f32, rm: RoundingMode)
    -> ParseResultI16 {

    let min = i16::min_value() as f32;
    let max = i16::max_value() as f32;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < min || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as i16) }
}

pub fn f32_to_i16_rm(a: f32, rm: RoundingMode)
    -> i16 {

    match f32_to_i16_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## 64

pub fn i64_to_i16_res(a: i64)
    -> ParseResultI16 {

    let min = i16::min_value() as i64;
    let max = i16::max_value() as i64;

    if a < min || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i16) }
}

pub fn i64_to_i16(a: i64)
    -> i16 {

    match i64_to_i16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn u64_to_i16_res(a: u64)
    -> ParseResultI16 {

    let max = i16::max_value() as u64;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i16) }
}

pub fn u64_to_i16(a: u64)
    -> i16 {

    match u64_to_i16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn f64_to_i16_res(a: f64)
    -> ParseResultI16 {

    f64_to_i16_rm_res(a, Trunc)
}

pub fn f64_to_i16(a: f64)
    -> i16 {

    f64_to_i16_rm(a, Trunc)
}

pub fn f64_to_i16_rm_res(a: f64, rm: RoundingMode)
    -> ParseResultI16 {

    let min = i16::min_value() as f64;
    let max = i16::max_value() as f64;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < min || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as i16) }
}

pub fn f64_to_i16_rm(a: f64, rm: RoundingMode)
    -> i16 {

    match f64_to_i16_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## size

pub fn isize_to_i16_res(a: isize)
    -> ParseResultI16 {

    let min = i16::min_value() as isize;
    let max = i16::max_value() as isize;

    if a < min || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i16) }
}

pub fn isize_to_i16(a: isize)
    -> i16 {

    match isize_to_i16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn usize_to_i16_res(a: usize)
    -> ParseResultI16 {

    let max = i16::max_value() as usize;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i16) }
}

pub fn usize_to_i16(a: usize)
    -> i16 {

    match usize_to_i16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## string

pub fn string_to_i16_res(a: String)
    -> ParseResultI16 {

    match a.parse::<i16>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

pub fn string_to_i16(a: String)
    -> i16 {

    match string_to_i16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn str_to_i16_res(a: &str)
    -> ParseResultI16 {

    match a.parse::<i16>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

pub fn str_to_i16(a: &str)
    -> i16 {

    match str_to_i16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// </editor-fold>

// <editor-fold> # Types

pub type ParseResultI16 = Result<i16, ParseError>;
// </editor-fold>
