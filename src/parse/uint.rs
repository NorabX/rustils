use error::ParseError;
use RoundingMode;
use RoundingMode::*;

pub trait ToU32 {

    fn to_u32_res(self)
        -> ParseResultU32;

    fn to_u32(self)
        -> u32;
}

pub trait ToU32RM {

    fn to_u32_rm_res(self, rm: RoundingMode)
        -> ParseResultU32;

    fn to_u32_rm(self, rm: RoundingMode)
        -> u32;
}

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
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
/// use rustils::parse::uint::bool_to_u32_res;
///
/// assert_eq!(bool_to_u32_res(true), Ok(1_u32));
/// assert_eq!(bool_to_u32_res(false), Ok(0_u32));
/// ```
pub fn bool_to_u32_res(a: bool)
    -> ParseResultU32 {

    if a { Ok(1) } else { Ok(0) }
}

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
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
/// use rustils::parse::uint::bool_to_u32;
///
/// assert_eq!(bool_to_u32(true), 1_u32);
/// assert_eq!(bool_to_u32(false), 0_u32);
/// ```
pub fn bool_to_u32(a: bool)
    -> u32 {

    if a { 1 } else { 0 }
}

pub fn i8_to_u32_res(a: i8)
    -> ParseResultU32 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u32) }
}

pub fn i8_to_u32(a: i8)
    -> u32 {

    match i8_to_u32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn i16_to_u32_res(a: i16)
    -> ParseResultU32 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u32) }
}

pub fn i16_to_u32(a: i16)
    -> u32 {

    match i16_to_u32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn i32_to_u32_res(a: i32)
    -> ParseResultU32 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u32) }
}

pub fn i32_to_u32(a: i32)
    -> u32 {

    match i32_to_u32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn f32_to_u32_res(a: f32)
    -> ParseResultU32 {

    f32_to_u32_rm_res(a, Trunc)
}

pub fn f32_to_u32(a: f32)
    -> u32 {

    f32_to_u32_rm(a, Trunc)
}

pub fn f32_to_u32_rm_res(a: f32, rm: RoundingMode)
    -> ParseResultU32 {

    let max = 16777215_f32;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < 0.0 || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as u32) }
}

pub fn f32_to_u32_rm(a: f32, rm: RoundingMode)
    -> u32 {

    match f32_to_u32_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn i64_to_u32_res(a: i64)
    -> ParseResultU32 {

    let max = u32::max_value() as i64;

    if a < 0 || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u32) }
}

pub fn i64_to_u32(a: i64)
    -> u32 {

    match i64_to_u32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn u64_to_u32_res(a: u64)
    -> ParseResultU32 {

    let max = u32::max_value() as u64;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u32) }
}

pub fn u64_to_u32(a: u64)
    -> u32 {

    match u64_to_u32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn f64_to_u32_res(a: f64)
    -> ParseResultU32 {

    f64_to_u32_rm_res(a, Trunc)
}

pub fn f64_to_u32(a: f64)
    -> u32 {

    f64_to_u32_rm(a, Trunc)
}

pub fn f64_to_u32_rm_res(a: f64, rm: RoundingMode)
    -> ParseResultU32 {

    let max = u32::max_value() as f64;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < 0.0 || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as u32) }
}

pub fn f64_to_u32_rm(a: f64, rm: RoundingMode)
    -> u32 {

    match f64_to_u32_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn isize_to_u32_res(a: isize)
    -> ParseResultU32 {

    let max = u32::max_value() as isize;

    if a < 0 || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u32) }
}

pub fn isize_to_u32(a: isize)
    -> u32 {

    match isize_to_u32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn usize_to_u32_res(a: usize)
    -> ParseResultU32 {

    let max = u32::max_value() as usize;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u32) }
}

pub fn usize_to_u32(a: usize)
    -> u32 {

    match usize_to_u32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn string_to_u32_res(a: String)
    -> ParseResultU32 {

    match a.parse::<u32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

pub fn string_to_u32(a: String)
    -> u32 {

    match string_to_u32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn str_to_u32_res(a: &str)
    -> ParseResultU32 {

    match a.parse::<u32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

pub fn str_to_u32(a: &str)
    -> u32 {

    match str_to_u32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub type ParseResultU32 = Result<u32, ParseError>;
