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

/// Parse [`i8`](https://doc.rust-lang.org/std/primitive.i8.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
///
/// # Arguments
///
/// * `a` - Any [`i8`](https://doc.rust-lang.org/std/primitive.i8.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::uint::i8_to_u32_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(i8_to_u32_res(0_i8), Ok(0_u32));
/// assert_eq!(i8_to_u32_res(127_i8), Ok(127_u32));
/// assert_eq!(i8_to_u32_res(-1_i8), Err(InvalidNumber(String::from("-1"))));
/// ```
pub fn i8_to_u32_res(a: i8)
    -> ParseResultU32 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u32) }
}

/// Parse [`i8`](https://doc.rust-lang.org/std/primitive.i8.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::uint::i8_to_u32(-1_i8);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`i8`](https://doc.rust-lang.org/std/primitive.i8.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::uint::i8_to_u32;
///
/// assert_eq!(i8_to_u32(0_i8), 0_u32);
/// assert_eq!(i8_to_u32(127_i8), 127_u32);
/// ```
pub fn i8_to_u32(a: i8)
    -> u32 {

    match i8_to_u32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
///
/// # Arguments
///
/// * `a` - Any [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::uint::i16_to_u32_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(i16_to_u32_res(0_i16), Ok(0_u32));
/// assert_eq!(i16_to_u32_res(32767_i16), Ok(32767_u32));
/// assert_eq!(i16_to_u32_res(-1_i16), Err(InvalidNumber(String::from("-1"))));
/// ```
pub fn i16_to_u32_res(a: i16)
    -> ParseResultU32 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u32) }
}

/// Parse [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::uint::i16_to_u32(-1_i16);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::uint::i16_to_u32;
///
/// assert_eq!(i16_to_u32(0_i16), 0_u32);
/// assert_eq!(i16_to_u32(32767_i16), 32767_u32);
/// ```
pub fn i16_to_u32(a: i16)
    -> u32 {

    match i16_to_u32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
///
/// # Arguments
///
/// * `a` - Any [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::uint::i32_to_u32_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(i32_to_u32_res(0_i32), Ok(0_u32));
/// assert_eq!(i32_to_u32_res(2147483647_i32), Ok(2147483647_u32));
/// assert_eq!(i32_to_u32_res(-1_i32), Err(InvalidNumber(String::from("-1"))));
/// ```
pub fn i32_to_u32_res(a: i32)
    -> ParseResultU32 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u32) }
}

/// Parse [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::uint::i32_to_u32(-1_i32);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::uint::i32_to_u32;
///
/// assert_eq!(i32_to_u32(0_i32), 0_u32);
/// assert_eq!(i32_to_u32(2147483647_i32), 2147483647_u32);
/// ```
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

/// Parse [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
///
/// # Arguments
///
/// * `a` - Any [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::uint::i64_to_u32_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(i64_to_u32_res(0_i64), Ok(0_u32));
/// assert_eq!(i64_to_u32_res(4294967295_i64), Ok(4294967295_u32));
/// assert_eq!(i64_to_u32_res(-1_i64), Err(InvalidNumber(String::from("-1"))));
/// assert_eq!(i64_to_u32_res(4294967296_i64), Err(InvalidNumber(String::from("4294967296"))));
/// ```
pub fn i64_to_u32_res(a: i64)
    -> ParseResultU32 {

    let max = u32::max_value() as i64;

    if a < 0 || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u32) }
}

/// Parse [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::uint::i64_to_u32(-1_i64);
/// rustils::parse::uint::i64_to_u32(4294967296_i64);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::uint::i64_to_u32;
///
/// assert_eq!(i64_to_u32(0_i64), 0_u32);
/// assert_eq!(i64_to_u32(4294967295_i64), 4294967295_u32);
/// ```
pub fn i64_to_u32(a: i64)
    -> u32 {

    match i64_to_u32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
///
/// # Arguments
///
/// * `a` - Any [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::uint::u64_to_u32_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(u64_to_u32_res(0_u64), Ok(0_u32));
/// assert_eq!(u64_to_u32_res(4294967295_u64), Ok(4294967295_u32));
/// assert_eq!(u64_to_u32_res(4294967296_u64), Err(InvalidNumber(String::from("4294967296"))));
/// ```
pub fn u64_to_u32_res(a: u64)
    -> ParseResultU32 {

    let max = u32::max_value() as u64;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u32) }
}

/// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::uint::u64_to_u32(4294967296_u64);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::uint::u64_to_u32;
///
/// assert_eq!(u64_to_u32(0_u64), 0_u32);
/// assert_eq!(u64_to_u32(4294967295_u64), 4294967295_u32);
/// ```
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

/// Parse [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
///
/// # Arguments
///
/// * `a` - Any [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::uint::isize_to_u32_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(isize_to_u32_res(0_isize), Ok(0_u32));
/// assert_eq!(isize_to_u32_res(4294967295_isize), Ok(4294967295_u32));
/// assert_eq!(isize_to_u32_res(-1_isize), Err(InvalidNumber(String::from("-1"))));
/// assert_eq!(isize_to_u32_res(4294967296_isize), Err(InvalidNumber(String::from("4294967296"))));
/// ```
pub fn isize_to_u32_res(a: isize)
    -> ParseResultU32 {

    let max = u32::max_value() as isize;

    if a < 0 || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u32) }
}

/// Parse [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::uint::isize_to_u32(-1_isize);
/// rustils::parse::uint::isize_to_u32(4294967296_isize);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::uint::isize_to_u32;
///
/// assert_eq!(isize_to_u32(0_isize), 0_u32);
/// assert_eq!(isize_to_u32(4294967295_isize), 4294967295_u32);
/// ```
pub fn isize_to_u32(a: isize)
    -> u32 {

    match isize_to_u32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
///
/// # Arguments
///
/// * `a` - Any [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::uint::usize_to_u32_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(usize_to_u32_res(0_usize), Ok(0_u32));
/// assert_eq!(usize_to_u32_res(4294967295_usize), Ok(4294967295_u32));
/// assert_eq!(usize_to_u32_res(4294967296_usize), Err(InvalidNumber(String::from("4294967296"))));
/// ```
pub fn usize_to_u32_res(a: usize)
    -> ParseResultU32 {

    let max = u32::max_value() as usize;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u32) }
}

/// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::uint::usize_to_u32(4294967296_usize);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::uint::usize_to_u32;
///
/// assert_eq!(usize_to_u32(0_usize), 0_u32);
/// assert_eq!(usize_to_u32(4294967295_usize), 4294967295_u32);
/// ```
pub fn usize_to_u32(a: usize)
    -> u32 {

    match usize_to_u32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`String`](https://doc.rust-lang.org/std/string/struct.String.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
///
/// # Arguments
///
/// * `a` - Any [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::uint::string_to_u32_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(string_to_u32_res("0".to_string()), Ok(0_u32));
/// assert_eq!(string_to_u32_res("4294967295".to_string()), Ok(4294967295_u32));
/// assert_eq!(string_to_u32_res("-1".to_string()), Err(InvalidNumber(String::from("-1"))));
/// assert_eq!(
///     string_to_u32_res("4294967296".to_string()),
///     Err(InvalidNumber(String::from("4294967296")))
/// );
/// ```
pub fn string_to_u32_res(a: String)
    -> ParseResultU32 {

    match a.parse::<u32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

/// Parse [`String`](https://doc.rust-lang.org/std/string/struct.String.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::uint::string_to_u32("-1".to_string());
/// rustils::parse::uint::string_to_u32("4294967296".to_string());
/// ```
///
/// # Arguments
///
/// * `a` - Any [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::uint::string_to_u32;
///
/// assert_eq!(string_to_u32("0".to_string()), 0_u32);
/// assert_eq!(string_to_u32("4294967295".to_string()), 4294967295_u32);
/// ```
pub fn string_to_u32(a: String)
    -> u32 {

    match string_to_u32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`&str`](https://doc.rust-lang.org/std/primitive.str.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
///
/// # Arguments
///
/// * `a` - Any [`&str`](https://doc.rust-lang.org/std/primitive.str.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::uint::str_to_u32_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(str_to_u32_res("0"), Ok(0_u32));
/// assert_eq!(str_to_u32_res("4294967295"), Ok(4294967295_u32));
/// assert_eq!(str_to_u32_res("-1"), Err(InvalidNumber(String::from("-1"))));
/// assert_eq!(str_to_u32_res("4294967296"), Err(InvalidNumber(String::from("4294967296"))));
/// ```
pub fn str_to_u32_res(a: &str)
    -> ParseResultU32 {

    match a.parse::<u32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

/// Parse [`&str`](https://doc.rust-lang.org/std/primitive.str.html) to
/// [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::uint::str_to_u32("-1");
/// rustils::parse::uint::str_to_u32("4294967296");
/// ```
///
/// # Arguments
///
/// * `a` - Any [`&str`](https://doc.rust-lang.org/std/primitive.str.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::uint::str_to_u32;
///
/// assert_eq!(str_to_u32("0"), 0_u32);
/// assert_eq!(str_to_u32("4294967295"), 4294967295_u32);
/// ```
pub fn str_to_u32(a: &str)
    -> u32 {

    match str_to_u32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub type ParseResultU32 = Result<u32, ParseError>;
