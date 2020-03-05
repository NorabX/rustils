use error::ParseError;
use RoundingMode;
use RoundingMode::*;

pub trait ToU64 {

    fn to_u64_res(self)
        -> ParseResultU64;

    fn to_u64(self)
        -> u64;
}

pub trait ToU64RM {

    fn to_u64_rm_res(self, rm: RoundingMode)
        -> ParseResultU64;

    fn to_u64_rm(self, rm: RoundingMode)
        -> u64;
}

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`u64`](https://doc.rust-lang.org/std/primitive.u64.html)
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
/// use rustils::parse::ulong::bool_to_u64_res;
///
/// assert_eq!(bool_to_u64_res(true), Ok(1_u64));
/// assert_eq!(bool_to_u64_res(false), Ok(0_u64));
/// ```
pub fn bool_to_u64_res(a: bool)
    -> ParseResultU64 {

    if a { Ok(1) } else { Ok(0) }
}

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`u64`](https://doc.rust-lang.org/std/primitive.u64.html)
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
/// use rustils::parse::ulong::bool_to_u64;
///
/// assert_eq!(bool_to_u64(true), 1_u64);
/// assert_eq!(bool_to_u64(false), 0_u64);
/// ```
pub fn bool_to_u64(a: bool)
    -> u64 {

    if a { 1 } else { 0 }
}

/// Parse [`i8`](https://doc.rust-lang.org/std/primitive.i8.html) to
/// [`u64`](https://doc.rust-lang.org/std/primitive.u64.html)
///
/// # Arguments
///
/// * `a` - Any [`i8`](https://doc.rust-lang.org/std/primitive.i8.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ulong::i8_to_u64_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(i8_to_u64_res(0_i8), Ok(0_u64));
/// assert_eq!(i8_to_u64_res(127_i8), Ok(127_u64));
/// assert_eq!(i8_to_u64_res(-1_i8), Err(InvalidNumber(String::from("-1"))));
/// ```
pub fn i8_to_u64_res(a: i8)
    -> ParseResultU64 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u64) }
}

/// Parse [`i8`](https://doc.rust-lang.org/std/primitive.i8.html) to
/// [`u64`](https://doc.rust-lang.org/std/primitive.u64.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ulong::i8_to_u64(-1_i8);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`i8`](https://doc.rust-lang.org/std/primitive.i8.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ulong::i8_to_u64;
///
/// assert_eq!(i8_to_u64(0_i8), 0_u64);
/// assert_eq!(i8_to_u64(127_i8), 127_u64);
/// ```
pub fn i8_to_u64(a: i8)
    -> u64 {

    match i8_to_u64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) to
/// [`u64`](https://doc.rust-lang.org/std/primitive.u64.html)
///
/// # Arguments
///
/// * `a` - Any [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ulong::i16_to_u64_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(i16_to_u64_res(0_i16), Ok(0_u64));
/// assert_eq!(i16_to_u64_res(32767_i16), Ok(32767_u64));
/// assert_eq!(i16_to_u64_res(-1_i16), Err(InvalidNumber(String::from("-1"))));
/// ```
pub fn i16_to_u64_res(a: i16)
    -> ParseResultU64 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u64) }
}

/// Parse [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) to
/// [`u64`](https://doc.rust-lang.org/std/primitive.u64.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ulong::i16_to_u64(-1_i16);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ulong::i16_to_u64;
///
/// assert_eq!(i16_to_u64(0_i16), 0_u64);
/// assert_eq!(i16_to_u64(32767_i16), 32767_u64);
/// ```
pub fn i16_to_u64(a: i16)
    -> u64 {

    match i16_to_u64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) to
/// [`u64`](https://doc.rust-lang.org/std/primitive.u64.html)
///
/// # Arguments
///
/// * `a` - Any [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ulong::i32_to_u64_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(i32_to_u64_res(0_i32), Ok(0_u64));
/// assert_eq!(i32_to_u64_res(2147483647_i32), Ok(2147483647_u64));
/// assert_eq!(i32_to_u64_res(-1_i32), Err(InvalidNumber(String::from("-1"))));
/// ```
pub fn i32_to_u64_res(a: i32)
    -> ParseResultU64 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u64) }
}

/// Parse [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) to
/// [`u64`](https://doc.rust-lang.org/std/primitive.u64.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ulong::i32_to_u64(-1_i32);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ulong::i32_to_u64;
///
/// assert_eq!(i32_to_u64(0_i32), 0_u64);
/// assert_eq!(i32_to_u64(2147483647_i32), 2147483647_u64);
/// ```
pub fn i32_to_u64(a: i32)
    -> u64 {

    match i32_to_u64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn f32_to_u64_res(a: f32)
    -> ParseResultU64 {

    f32_to_u64_rm_res(a, Trunc)
}

pub fn f32_to_u64(a: f32)
    -> u64 {

    f32_to_u64_rm(a, Trunc)
}

pub fn f32_to_u64_rm_res(a: f32, rm: RoundingMode)
    -> ParseResultU64 {

    let max = 16777215_f32;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < 0.0 || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as u64) }
}

pub fn f32_to_u64_rm(a: f32, rm: RoundingMode)
    -> u64 {

    match f32_to_u64_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) to
/// [`u64`](https://doc.rust-lang.org/std/primitive.u64.html)
///
/// # Arguments
///
/// * `a` - Any [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ulong::i64_to_u64_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(i64_to_u64_res(0_i64), Ok(0_u64));
/// assert_eq!(i64_to_u64_res(9223372036854775807_i64), Ok(9223372036854775807_u64));
/// assert_eq!(i64_to_u64_res(-1_i64), Err(InvalidNumber(String::from("-1"))));
/// ```
pub fn i64_to_u64_res(a: i64)
    -> ParseResultU64 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u64) }
}

/// Parse [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) to
/// [`u64`](https://doc.rust-lang.org/std/primitive.u64.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ulong::i64_to_u64(-1_i64);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ulong::i64_to_u64;
///
/// assert_eq!(i64_to_u64(0_i64), 0_u64);
/// assert_eq!(i64_to_u64(9223372036854775807_i64), 9223372036854775807_u64);
/// ```
pub fn i64_to_u64(a: i64)
    -> u64 {

    match i64_to_u64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn f64_to_u64_res(a: f64)
    -> ParseResultU64 {

    f64_to_u64_rm_res(a, Trunc)
}

pub fn f64_to_u64(a: f64)
    -> u64 {

    f64_to_u64_rm(a, Trunc)
}

pub fn f64_to_u64_rm_res(a: f64, rm: RoundingMode)
    -> ParseResultU64 {

    let max = 9007199254740991_f64;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < 0.0 || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as u64) }
}

pub fn f64_to_u64_rm(a: f64, rm: RoundingMode)
    -> u64 {

    match f64_to_u64_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) to
/// [`u64`](https://doc.rust-lang.org/std/primitive.u64.html)
///
/// # Arguments
///
/// * `a` - Any [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ulong::isize_to_u64_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(isize_to_u64_res(0_isize), Ok(0_u64));
/// assert_eq!(isize_to_u64_res(9223372036854775807_isize), Ok(9223372036854775807_u64));
/// assert_eq!(isize_to_u64_res(-1_isize), Err(InvalidNumber(String::from("-1"))));
/// ```
pub fn isize_to_u64_res(a: isize)
    -> ParseResultU64 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u64) }
}

/// Parse [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) to
/// [`u64`](https://doc.rust-lang.org/std/primitive.u64.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ulong::isize_to_u64(-1_isize);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ulong::isize_to_u64;
///
/// assert_eq!(isize_to_u64(0_isize), 0_u64);
/// assert_eq!(isize_to_u64(9223372036854775807_isize), 9223372036854775807_u64);
/// ```
pub fn isize_to_u64(a: isize)
    -> u64 {

    match isize_to_u64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`String`](https://doc.rust-lang.org/std/string/struct.String.html) to
/// [`u64`](https://doc.rust-lang.org/std/primitive.u64.html)
///
/// # Arguments
///
/// * `a` - Any [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::ulong::string_to_u64_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(string_to_u64_res("0".to_string()), Ok(0_u64));
/// assert_eq!(string_to_u64_res("18446744073709551615".to_string()), Ok(18446744073709551615_u64));
/// assert_eq!(string_to_u64_res("-1".to_string()), Err(InvalidNumber(String::from("-1"))));
/// assert_eq!(
///     string_to_u64_res("18446744073709551616".to_string()),
///     Err(InvalidNumber(String::from("18446744073709551616")))
/// );
/// ```
pub fn string_to_u64_res(a: String)
    -> ParseResultU64 {

    match a.parse::<u64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

/// Parse [`String`](https://doc.rust-lang.org/std/string/struct.String.html) to
/// [`u64`](https://doc.rust-lang.org/std/primitive.u64.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ulong::string_to_u64("-1".to_string());
/// rustils::parse::ulong::string_to_u64("18446744073709551616".to_string());
/// ```
///
/// # Arguments
///
/// * `a` - Any [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::ulong::string_to_u64;
///
/// assert_eq!(string_to_u64("0".to_string()), 0_u64);
/// assert_eq!(string_to_u64("18446744073709551615".to_string()), 18446744073709551615_u64);
/// ```
pub fn string_to_u64(a: String)
    -> u64 {

    match string_to_u64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`&str`](https://doc.rust-lang.org/std/primitive.str.html) to
/// [`u64`](https://doc.rust-lang.org/std/primitive.u64.html)
///
/// # Arguments
///
/// * `a` - Any [`&str`](https://doc.rust-lang.org/std/primitive.str.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::ulong::str_to_u64_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(str_to_u64_res("0"), Ok(0_u64));
/// assert_eq!(str_to_u64_res("18446744073709551615"), Ok(18446744073709551615_u64));
/// assert_eq!(str_to_u64_res("-1"), Err(InvalidNumber(String::from("-1"))));
/// assert_eq!(
///     str_to_u64_res("18446744073709551616"),
///     Err(InvalidNumber(String::from("18446744073709551616")))
/// );
/// ```
pub fn str_to_u64_res(a: &str)
    -> ParseResultU64 {

    match a.parse::<u64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

/// Parse [`&str`](https://doc.rust-lang.org/std/primitive.str.html) to
/// [`u64`](https://doc.rust-lang.org/std/primitive.u64.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ulong::str_to_u64("-1");
/// rustils::parse::ulong::str_to_u64("18446744073709551616");
/// ```
///
/// # Arguments
///
/// * `a` - Any [`&str`](https://doc.rust-lang.org/std/primitive.str.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::ulong::str_to_u64;
///
/// assert_eq!(str_to_u64("0"), 0_u64);
/// assert_eq!(str_to_u64("18446744073709551615"), 18446744073709551615_u64);
/// ```
pub fn str_to_u64(a: &str)
    -> u64 {

    match str_to_u64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub type ParseResultU64 = Result<u64, ParseError>;
