use error::ParseError;
use RoundingMode;
use RoundingMode::*;

pub trait ToI32 {

    fn to_i32_res(self)
        -> ParseResultI32;

    fn to_i32(self)
        -> i32;
}

pub trait ToI32RM {

    fn to_i32_rm_res(self, rm: RoundingMode)
        -> ParseResultI32;

    fn to_i32_rm(self, rm: RoundingMode)
        -> i32;
}

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
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
/// use rustils::parse::int::bool_to_i32_res;
///
/// assert_eq!(bool_to_i32_res(true), Ok(1_i32));
/// assert_eq!(bool_to_i32_res(false), Ok(0_i32));
/// ```
pub fn bool_to_i32_res(a: bool)
    -> ParseResultI32 {

    if a { Ok(1) } else { Ok(0) }
}

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
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
/// use rustils::parse::int::bool_to_i32;
///
/// assert_eq!(bool_to_i32(true), 1_i32);
/// assert_eq!(bool_to_i32(false), 0_i32);
/// ```
pub fn bool_to_i32(a: bool)
    -> i32 {

    if a { 1 } else { 0 }
}

pub fn u32_to_i32_res(a: u32)
    -> ParseResultI32 {

    let max = i32::max_value() as u32;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i32) }
}

pub fn u32_to_i32(a: u32)
    -> i32 {

    match u32_to_i32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn f32_to_i32_res(a: f32)
    -> ParseResultI32 {

    f32_to_i32_rm_res(a, Trunc)
}

pub fn f32_to_i32(a: f32)
    -> i32 {

    f32_to_i32_rm(a, Trunc)
}

pub fn f32_to_i32_rm_res(a: f32, rm: RoundingMode)
    -> ParseResultI32 {

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
    } else { Ok(x as i32) }
}

pub fn f32_to_i32_rm(a: f32, rm: RoundingMode)
    -> i32 {

    match f32_to_i32_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn i64_to_i32_res(a: i64)
    -> ParseResultI32 {

    let min = i32::min_value() as i64;
    let max = i32::max_value() as i64;

    if a < min || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i32) }
}

pub fn i64_to_i32(a: i64)
    -> i32 {

    match i64_to_i32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn u64_to_i32_res(a: u64)
    -> ParseResultI32 {

    let max = i32::max_value() as u64;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i32) }
}

pub fn u64_to_i32(a: u64)
    -> i32 {

    match u64_to_i32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn f64_to_i32_res(a: f64)
    -> ParseResultI32 {

    f64_to_i32_rm_res(a, Trunc)
}

pub fn f64_to_i32(a: f64)
    -> i32 {

    f64_to_i32_rm(a, Trunc)
}

pub fn f64_to_i32_rm_res(a: f64, rm: RoundingMode)
    -> ParseResultI32 {

    let min = i32::min_value() as f64;
    let max = i32::max_value() as f64;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < min || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as i32) }
}

pub fn f64_to_i32_rm(a: f64, rm: RoundingMode)
    -> i32 {

    match f64_to_i32_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn isize_to_i32_res(a: isize)
    -> ParseResultI32 {

    let min = i32::min_value() as isize;
    let max = i32::max_value() as isize;

    if a < min || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i32) }
}

pub fn isize_to_i32(a: isize)
    -> i32 {

    match isize_to_i32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn usize_to_i32_res(a: usize)
    -> ParseResultI32 {

    let max = i32::max_value() as usize;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i32) }
}

pub fn usize_to_i32(a: usize)
    -> i32 {

    match usize_to_i32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn string_to_i32_res(a: String)
    -> ParseResultI32 {

    match a.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

pub fn string_to_i32(a: String)
    -> i32 {

    match string_to_i32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn str_to_i32_res(a: &str)
    -> ParseResultI32 {

    match a.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

pub fn str_to_i32(a: &str)
    -> i32 {

    match str_to_i32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub type ParseResultI32 = Result<i32, ParseError>;
