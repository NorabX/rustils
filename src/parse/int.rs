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

/// Parse [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) to
/// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
///
/// # Arguments
///
/// * `a` - Any [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::int::u32_to_i32_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(u32_to_i32_res(0_u32), Ok(0_i32));
/// assert_eq!(u32_to_i32_res(2147483647_u32), Ok(2147483647_i32));
/// assert_eq!(u32_to_i32_res(2147483648_u32), Err(InvalidNumber(String::from("2147483648"))));
/// ```
pub fn u32_to_i32_res(a: u32)
    -> ParseResultI32 {

    let max = i32::max_value() as u32;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i32) }
}

/// Parse [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) to
/// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::int::u32_to_i32(2147483648_u32);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::int::u32_to_i32;
///
/// assert_eq!(u32_to_i32(0_u32), 0_i32);
/// assert_eq!(u32_to_i32(2147483647_u32), 2147483647_i32);
/// ```
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

/// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
/// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
///
/// # Arguments
///
/// * `a` - Any [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::int::u64_to_i32_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(u64_to_i32_res(0_u64), Ok(0_i32));
/// assert_eq!(u64_to_i32_res(2147483647_u64), Ok(2147483647_i32));
/// assert_eq!(u64_to_i32_res(2147483648_u64), Err(InvalidNumber(String::from("2147483648"))));
/// ```
pub fn u64_to_i32_res(a: u64)
    -> ParseResultI32 {

    let max = i32::max_value() as u64;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i32) }
}

/// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
/// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::int::u64_to_i32(2147483648_u64);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::int::u64_to_i32;
///
/// assert_eq!(u64_to_i32(0_u64), 0_i32);
/// assert_eq!(u64_to_i32(2147483647_u64), 2147483647_i32);
/// ```
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

/// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
/// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
///
/// # Arguments
///
/// * `a` - Any [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::int::usize_to_i32_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(usize_to_i32_res(0_usize), Ok(0_i32));
/// assert_eq!(usize_to_i32_res(2147483647_usize), Ok(2147483647_i32));
/// assert_eq!(usize_to_i32_res(2147483648_usize), Err(InvalidNumber(String::from("2147483648"))));
/// ```
pub fn usize_to_i32_res(a: usize)
    -> ParseResultI32 {

    let max = i32::max_value() as usize;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i32) }
}

/// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
/// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::int::usize_to_i32(2147483648_usize);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::int::usize_to_i32;
///
/// assert_eq!(usize_to_i32(0_usize), 0_i32);
/// assert_eq!(usize_to_i32(2147483647_usize), 2147483647_i32);
/// ```
pub fn usize_to_i32(a: usize)
    -> i32 {

    match usize_to_i32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`String`](https://doc.rust-lang.org/std/string/struct.String.html) to
/// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
///
/// # Arguments
///
/// * `a` - Any [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::int::string_to_i32_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(string_to_i32_res("-2147483648".to_string()), Ok(-2147483648_i32));
/// assert_eq!(string_to_i32_res("2147483647".to_string()), Ok(2147483647_i32));
/// assert_eq!(
///     string_to_i32_res("-2147483649".to_string()),
///     Err(InvalidNumber(String::from("-2147483649")))
/// );
///
/// assert_eq!(
///     string_to_i32_res("2147483648".to_string()),
///     Err(InvalidNumber(String::from("2147483648")))
/// );
/// ```
pub fn string_to_i32_res(a: String)
    -> ParseResultI32 {

    match a.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

/// Parse [`String`](https://doc.rust-lang.org/std/string/struct.String.html) to
/// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::int::string_to_i32("-2147483649".to_string());
/// rustils::parse::int::string_to_i32("2147483648".to_string());
/// ```
///
/// # Arguments
///
/// * `a` - Any [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::int::string_to_i32;
///
/// assert_eq!(string_to_i32("-2147483648".to_string()), -2147483648_i32);
/// assert_eq!(string_to_i32("2147483647".to_string()), 2147483647_i32);
/// ```
pub fn string_to_i32(a: String)
    -> i32 {

    match string_to_i32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`&str`](https://doc.rust-lang.org/std/primitive.str.html) to
/// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
///
/// # Arguments
///
/// * `a` - Any [`&str`](https://doc.rust-lang.org/std/primitive.str.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::int::str_to_i32_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(str_to_i32_res("-2147483648"), Ok(-2147483648_i32));
/// assert_eq!(str_to_i32_res("2147483647"), Ok(2147483647_i32));
/// assert_eq!(str_to_i32_res("-2147483649"), Err(InvalidNumber(String::from("-2147483649"))));
/// assert_eq!(str_to_i32_res("2147483648"), Err(InvalidNumber(String::from("2147483648"))));
/// ```
pub fn str_to_i32_res(a: &str)
    -> ParseResultI32 {

    match a.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

/// Parse [`&str`](https://doc.rust-lang.org/std/primitive.str.html) to
/// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::int::str_to_i32("-2147483649");
/// rustils::parse::int::str_to_i32("2147483648");
/// ```
///
/// # Arguments
///
/// * `a` - Any [`&str`](https://doc.rust-lang.org/std/primitive.str.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::int::str_to_i32;
///
/// assert_eq!(str_to_i32("-2147483648"), -2147483648_i32);
/// assert_eq!(str_to_i32("2147483647"), 2147483647_i32);
/// ```
pub fn str_to_i32(a: &str)
    -> i32 {

    match str_to_i32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub type ParseResultI32 = Result<i32, ParseError>;
