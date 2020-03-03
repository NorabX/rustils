use error::ParseError;
use RoundingMode;
use RoundingMode::*;

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

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`i64`](https://doc.rust-lang.org/std/primitive.i64.html)
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
/// use rustils::parse::long::bool_to_i64_res;
///
/// assert_eq!(bool_to_i64_res(true), Ok(1_i64));
/// assert_eq!(bool_to_i64_res(false), Ok(0_i64));
/// ```
pub fn bool_to_i64_res(a: bool)
    -> ParseResultI64 {

    if a { Ok(1) } else { Ok(0) }
}

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`i64`](https://doc.rust-lang.org/std/primitive.i64.html)
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
/// use rustils::parse::long::bool_to_i64;
///
/// assert_eq!(bool_to_i64(true), 1_i64);
/// assert_eq!(bool_to_i64(false), 0_i64);
/// ```
pub fn bool_to_i64(a: bool)
    -> i64 {

    if a { 1 } else { 0 }
}

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

/// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
/// [`i64`](https://doc.rust-lang.org/std/primitive.i64.html)
///
/// # Arguments
///
/// * `a` - Any [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::long::u64_to_i64_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(u64_to_i64_res(0_u64), Ok(0_i64));
/// assert_eq!(u64_to_i64_res(9223372036854775807_u64), Ok(9223372036854775807_i64));
/// assert_eq!(
///     u64_to_i64_res(9223372036854775808_u64),
///     Err(InvalidNumber(String::from("9223372036854775808")))
/// );
/// ```
pub fn u64_to_i64_res(a: u64)
    -> ParseResultI64 {

    let max = i64::max_value() as u64;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i64) }
}

/// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
/// [`i64`](https://doc.rust-lang.org/std/primitive.i64.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::long::u64_to_i64(9223372036854775808_u64);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::long::u64_to_i64;
///
/// assert_eq!(u64_to_i64(0_u64), 0_i64);
/// assert_eq!(u64_to_i64(9223372036854775807_u64), 9223372036854775807_i64);
/// ```
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

/// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
/// [`i64`](https://doc.rust-lang.org/std/primitive.i64.html)
///
/// # Arguments
///
/// * `a` - Any [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::long::usize_to_i64_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(usize_to_i64_res(0_usize), Ok(0_i64));
/// assert_eq!(usize_to_i64_res(9223372036854775807_usize), Ok(9223372036854775807_i64));
/// assert_eq!(
///     usize_to_i64_res(9223372036854775808_usize),
///     Err(InvalidNumber(String::from("9223372036854775808")))
/// );
/// ```
pub fn usize_to_i64_res(a: usize)
    -> ParseResultI64 {

    let max = i64::max_value() as usize;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i64) }
}

/// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
/// [`i64`](https://doc.rust-lang.org/std/primitive.i64.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::long::usize_to_i64(9223372036854775808_usize);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::long::usize_to_i64;
///
/// assert_eq!(usize_to_i64(0_usize), 0_i64);
/// assert_eq!(usize_to_i64(9223372036854775807_usize), 9223372036854775807_i64);
/// ```
pub fn usize_to_i64(a: usize)
    -> i64 {

    match usize_to_i64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`String`](https://doc.rust-lang.org/std/string/struct.String.html) to
/// [`i64`](https://doc.rust-lang.org/std/primitive.i64.html)
///
/// # Arguments
///
/// * `a` - Any [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::long::string_to_i64_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(
///     string_to_i64_res("-9223372036854775808".to_string()),
///     Ok(-9223372036854775808_i64)
/// );
///
/// assert_eq!(
///     string_to_i64_res("9223372036854775807".to_string()),
///     Ok(9223372036854775807_i64)
/// );
///
/// assert_eq!(
///     string_to_i64_res("-9223372036854775809".to_string()),
///     Err(InvalidNumber(String::from("-9223372036854775809")))
/// );
///
/// assert_eq!(
///     string_to_i64_res("9223372036854775808".to_string()),
///     Err(InvalidNumber(String::from("9223372036854775808")))
/// );
/// ```
pub fn string_to_i64_res(a: String)
    -> ParseResultI64 {

    match a.parse::<i64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

/// Parse [`String`](https://doc.rust-lang.org/std/string/struct.String.html) to
/// [`i64`](https://doc.rust-lang.org/std/primitive.i64.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::long::string_to_i64("-9223372036854775809".to_string());
/// rustils::parse::long::string_to_i64("9223372036854775808".to_string());
/// ```
///
/// # Arguments
///
/// * `a` - Any [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::long::string_to_i64;
///
/// assert_eq!(string_to_i64("-9223372036854775808".to_string()), -9223372036854775808_i64);
/// assert_eq!(string_to_i64("9223372036854775807".to_string()), 9223372036854775807_i64);
/// ```
pub fn string_to_i64(a: String)
    -> i64 {

    match string_to_i64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`&str`](https://doc.rust-lang.org/std/primitive.str.html) to
/// [`i64`](https://doc.rust-lang.org/std/primitive.i64.html)
///
/// # Arguments
///
/// * `a` - Any [`&str`](https://doc.rust-lang.org/std/primitive.str.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::long::str_to_i64_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(str_to_i64_res("-9223372036854775808"), Ok(-9223372036854775808_i64));
/// assert_eq!(str_to_i64_res("9223372036854775807"), Ok(9223372036854775807_i64));
/// assert_eq!(
///     str_to_i64_res("-9223372036854775809"),
///     Err(InvalidNumber(String::from("-9223372036854775809")))
/// );
///
/// assert_eq!(
///     str_to_i64_res("9223372036854775808"),
///     Err(InvalidNumber(String::from("9223372036854775808")))
/// );
/// ```
pub fn str_to_i64_res(a: &str)
    -> ParseResultI64 {

    match a.parse::<i64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

/// Parse [`&str`](https://doc.rust-lang.org/std/primitive.str.html) to
/// [`i64`](https://doc.rust-lang.org/std/primitive.i64.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::long::str_to_i64("-9223372036854775809");
/// rustils::parse::long::str_to_i64("9223372036854775808");
/// ```
///
/// # Arguments
///
/// * `a` - Any [`&str`](https://doc.rust-lang.org/std/primitive.str.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::long::str_to_i64;
///
/// assert_eq!(str_to_i64("-9223372036854775808"), -9223372036854775808_i64);
/// assert_eq!(str_to_i64("9223372036854775807"), 9223372036854775807_i64);
/// ```
pub fn str_to_i64(a: &str)
    -> i64 {

    match str_to_i64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub type ParseResultI64 = Result<i64, ParseError>;
