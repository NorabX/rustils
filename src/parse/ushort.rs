use error::ParseError;
use RoundingMode;
use RoundingMode::*;

pub trait ToU16 {

    fn to_u16_res(self)
        -> ParseResultU16;

    fn to_u16(self)
        -> u16;
}

pub trait ToU16RM {

    fn to_u16_rm_res(self, rm: RoundingMode)
        -> ParseResultU16;

    fn to_u16_rm(self, rm: RoundingMode)
        -> u16;
}

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// If `a` is `false` then returns `Ok(0)`.<br>
/// If `a` is `true` then returns `Ok(1)`.
///
/// # Arguments
///
/// * `a` - [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::bool_to_u16_res;
///
/// assert_eq!(bool_to_u16_res(true), Ok(1_u16));
/// assert_eq!(bool_to_u16_res(false), Ok(0_u16));
/// ```
pub fn bool_to_u16_res(a: bool)
    -> ParseResultU16 {

    if a { Ok(1) } else { Ok(0) }
}

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// If `a` is `false` then returns 0.<br>
/// If `a` is `true` then returns 1.
///
/// # Arguments
///
/// * `a` - [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::bool_to_u16;
///
/// assert_eq!(bool_to_u16(true), 1_u16);
/// assert_eq!(bool_to_u16(false), 0_u16);
/// ```
pub fn bool_to_u16(a: bool)
    -> u16 {

    if a { 1 } else { 0 }
}

pub fn i8_to_u16_res(a: i8)
    -> ParseResultU16 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u16) }
}

pub fn i8_to_u16(a: i8)
    -> u16 {

    match i8_to_u16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn i16_to_u16_res(a: i16)
    -> ParseResultU16 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u16) }
}

pub fn i16_to_u16(a: i16)
    -> u16 {

    match i16_to_u16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn i32_to_u16_res(a: i32)
    -> ParseResultU16 {

    let max = u16::max_value() as i32;

    if a < 0 || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u16) }
}

pub fn i32_to_u16(a: i32)
    -> u16 {

    match i32_to_u16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn u32_to_u16_res(a: u32)
    -> ParseResultU16 {

    let max = u16::max_value() as u32;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u16) }
}

pub fn u32_to_u16(a: u32)
    -> u16 {

    match u32_to_u16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn f32_to_u16_res(a: f32)
    -> ParseResultU16 {

    f32_to_u16_rm_res(a, Trunc)
}

pub fn f32_to_u16(a: f32)
    -> u16 {

    f32_to_u16_rm(a, Trunc)
}

pub fn f32_to_u16_rm_res(a: f32, rm: RoundingMode)
    -> ParseResultU16 {

    let max = u16::max_value() as f32;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < 0.0 || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as u16) }
}

pub fn f32_to_u16_rm(a: f32, rm: RoundingMode)
    -> u16 {

    match f32_to_u16_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn i64_to_u16_res(a: i64)
    -> ParseResultU16 {

    let max = u16::max_value() as i64;

    if a < 0 || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u16) }
}

pub fn i64_to_u16(a: i64)
    -> u16 {

    match i64_to_u16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn u64_to_u16_res(a: u64)
    -> ParseResultU16 {

    let max = u16::max_value() as u64;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u16) }
}

pub fn u64_to_u16(a: u64)
    -> u16 {

    match u64_to_u16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn f64_to_u16_res(a: f64)
    -> ParseResultU16 {

    f64_to_u16_rm_res(a, Trunc)
}

pub fn f64_to_u16(a: f64)
    -> u16 {

    f64_to_u16_rm(a, Trunc)
}

pub fn f64_to_u16_rm_res(a: f64, rm: RoundingMode)
    -> ParseResultU16 {

    let max = u16::max_value() as f64;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < 0.0 || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as u16) }
}

pub fn f64_to_u16_rm(a: f64, rm: RoundingMode)
    -> u16 {

    match f64_to_u16_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn isize_to_u16_res(a: isize)
    -> ParseResultU16 {

    let max = u16::max_value() as isize;

    if a < 0 || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u16) }
}

pub fn isize_to_u16(a: isize)
    -> u16 {

    match isize_to_u16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn usize_to_u16_res(a: usize)
    -> ParseResultU16 {

    let max = u16::max_value() as usize;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u16) }
}

pub fn usize_to_u16(a: usize)
    -> u16 {

    match usize_to_u16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn string_to_u16_res(a: String)
    -> ParseResultU16 {

    match a.parse::<u16>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

pub fn string_to_u16(a: String)
    -> u16 {

    match string_to_u16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn str_to_u16_res(a: &str)
    -> ParseResultU16 {

    match a.parse::<u16>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

pub fn str_to_u16(a: & str)
    -> u16 {

    match str_to_u16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub type ParseResultU16 = Result<u16, ParseError>;
