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

/// Parse [`i8`](https://doc.rust-lang.org/std/primitive.i8.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Arguments
///
/// * `a` - Any [`i8`](https://doc.rust-lang.org/std/primitive.i8.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::i8_to_u16_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(i8_to_u16_res(0_i8), Ok(0_u16));
/// assert_eq!(i8_to_u16_res(127_i8), Ok(127_u16));
/// assert_eq!(i8_to_u16_res(-1_i8), Err(InvalidNumber(String::from("-1"))));
/// ```
pub fn i8_to_u16_res(a: i8)
    -> ParseResultU16 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u16) }
}

/// Parse [`i8`](https://doc.rust-lang.org/std/primitive.i8.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ushort::i8_to_u16(-1_i8);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`i8`](https://doc.rust-lang.org/std/primitive.i8.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::i8_to_u16;
///
/// assert_eq!(i8_to_u16(0_i8), 0_u16);
/// assert_eq!(i8_to_u16(127_i8), 127_u16);
/// ```
pub fn i8_to_u16(a: i8)
    -> u16 {

    match i8_to_u16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Arguments
///
/// * `a` - Any [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::i16_to_u16_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(i16_to_u16_res(0_i16), Ok(0_u16));
/// assert_eq!(i16_to_u16_res(32767_i16), Ok(32767_u16));
/// assert_eq!(i16_to_u16_res(-1_i16), Err(InvalidNumber(String::from("-1"))));
/// ```
pub fn i16_to_u16_res(a: i16)
    -> ParseResultU16 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u16) }
}

/// Parse [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ushort::i16_to_u16(-1_i16);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::i16_to_u16;
///
/// assert_eq!(i16_to_u16(0_i16), 0_u16);
/// assert_eq!(i16_to_u16(32767_i16), 32767_u16);
/// ```
pub fn i16_to_u16(a: i16)
    -> u16 {

    match i16_to_u16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Arguments
///
/// * `a` - Any [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::i32_to_u16_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(i32_to_u16_res(0_i32), Ok(0_u16));
/// assert_eq!(i32_to_u16_res(65535_i32), Ok(65535_u16));
/// assert_eq!(i32_to_u16_res(-1_i32), Err(InvalidNumber(String::from("-1"))));
/// assert_eq!(i32_to_u16_res(65536_i32), Err(InvalidNumber(String::from("65536"))));
/// ```
pub fn i32_to_u16_res(a: i32)
    -> ParseResultU16 {

    let max = u16::max_value() as i32;

    if a < 0 || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u16) }
}

/// Parse [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ushort::i32_to_u16(-1_i32);
/// rustils::parse::ushort::i32_to_u16(65536_i32);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::i32_to_u16;
///
/// assert_eq!(i32_to_u16(0_i32), 0_u16);
/// assert_eq!(i32_to_u16(65535_i32), 65535_u16);
/// ```
pub fn i32_to_u16(a: i32)
    -> u16 {

    match i32_to_u16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Arguments
///
/// * `a` - Any [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::u32_to_u16_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(u32_to_u16_res(0_u32), Ok(0_u16));
/// assert_eq!(u32_to_u16_res(65535_u32), Ok(65535_u16));
/// assert_eq!(u32_to_u16_res(65536_u32), Err(InvalidNumber(String::from("65536"))));
/// ```
pub fn u32_to_u16_res(a: u32)
    -> ParseResultU16 {

    let max = u16::max_value() as u32;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u16) }
}

/// Parse [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ushort::u32_to_u16(65536_u32);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::u32_to_u16;
///
/// assert_eq!(u32_to_u16(0_u32), 0_u16);
/// assert_eq!(u32_to_u16(65535_u32), 65535_u16);
/// ```
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

/// Parse [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Arguments
///
/// * `a` - Any [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::i64_to_u16_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(i64_to_u16_res(0_i64), Ok(0_u16));
/// assert_eq!(i64_to_u16_res(65535_i64), Ok(65535_u16));
/// assert_eq!(i64_to_u16_res(-1_i64), Err(InvalidNumber(String::from("-1"))));
/// assert_eq!(i64_to_u16_res(65536_i64), Err(InvalidNumber(String::from("65536"))));
/// ```
pub fn i64_to_u16_res(a: i64)
    -> ParseResultU16 {

    let max = u16::max_value() as i64;

    if a < 0 || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u16) }
}

/// Parse [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ushort::i64_to_u16(-1_i64);
/// rustils::parse::ushort::i64_to_u16(65536_i64);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::i64_to_u16;
///
/// assert_eq!(i64_to_u16(0_i64), 0_u16);
/// assert_eq!(i64_to_u16(65535_i64), 65535_u16);
/// ```
pub fn i64_to_u16(a: i64)
    -> u16 {

    match i64_to_u16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Arguments
///
/// * `a` - Any [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::u64_to_u16_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(u64_to_u16_res(0_u64), Ok(0_u16));
/// assert_eq!(u64_to_u16_res(65535_u64), Ok(65535_u16));
/// assert_eq!(u64_to_u16_res(65536_u64), Err(InvalidNumber(String::from("65536"))));
/// ```
pub fn u64_to_u16_res(a: u64)
    -> ParseResultU16 {

    let max = u16::max_value() as u64;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u16) }
}

/// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ushort::u64_to_u16(65536_u64);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::u64_to_u16;
///
/// assert_eq!(u64_to_u16(0_u64), 0_u16);
/// assert_eq!(u64_to_u16(65535_u64), 65535_u16);
/// ```
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

/// Parse [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Arguments
///
/// * `a` - Any [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::isize_to_u16_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(isize_to_u16_res(0_isize), Ok(0_u16));
/// assert_eq!(isize_to_u16_res(65535_isize), Ok(65535_u16));
/// assert_eq!(isize_to_u16_res(-1_isize), Err(InvalidNumber(String::from("-1"))));
/// assert_eq!(isize_to_u16_res(65536_isize), Err(InvalidNumber(String::from("65536"))));
/// ```
pub fn isize_to_u16_res(a: isize)
    -> ParseResultU16 {

    let max = u16::max_value() as isize;

    if a < 0 || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u16) }
}

/// Parse [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ushort::isize_to_u16(-1_isize);
/// rustils::parse::ushort::isize_to_u16(65536_isize);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::isize_to_u16;
///
/// assert_eq!(isize_to_u16(0_isize), 0_u16);
/// assert_eq!(isize_to_u16(65535_isize), 65535_u16);
/// ```
pub fn isize_to_u16(a: isize)
    -> u16 {

    match isize_to_u16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Arguments
///
/// * `a` - Any [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::usize_to_u16_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(usize_to_u16_res(0_usize), Ok(0_u16));
/// assert_eq!(usize_to_u16_res(65535_usize), Ok(65535_u16));
/// assert_eq!(usize_to_u16_res(65536_usize), Err(InvalidNumber(String::from("65536"))));
/// ```
pub fn usize_to_u16_res(a: usize)
    -> ParseResultU16 {

    let max = u16::max_value() as usize;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u16) }
}

/// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ushort::usize_to_u16(65536_usize);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::usize_to_u16;
///
/// assert_eq!(usize_to_u16(0_usize), 0_u16);
/// assert_eq!(usize_to_u16(65535_usize), 65535_u16);
/// ```
pub fn usize_to_u16(a: usize)
    -> u16 {

    match usize_to_u16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`String`](https://doc.rust-lang.org/std/string/struct.String.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Arguments
///
/// * `a` - Any [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::string_to_u16_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(string_to_u16_res("0".to_string()), Ok(0_u16));
/// assert_eq!(string_to_u16_res("65535".to_string()), Ok(65535_u16));
/// assert_eq!(string_to_u16_res("-1".to_string()), Err(InvalidNumber(String::from("-1"))));
/// assert_eq!(string_to_u16_res("65536".to_string()), Err(InvalidNumber(String::from("65536"))));
/// ```
pub fn string_to_u16_res(a: String)
    -> ParseResultU16 {

    match a.parse::<u16>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

/// Parse [`String`](https://doc.rust-lang.org/std/string/struct.String.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ushort::string_to_u16("-1".to_string());
/// rustils::parse::ushort::string_to_u16("65536".to_string());
/// ```
///
/// # Arguments
///
/// * `a` - Any [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::string_to_u16;
///
/// assert_eq!(string_to_u16("0".to_string()), 0_u16);
/// assert_eq!(string_to_u16("65535".to_string()), 65535_u16);
/// ```
pub fn string_to_u16(a: String)
    -> u16 {

    match string_to_u16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`&str`](https://doc.rust-lang.org/std/primitive.str.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Arguments
///
/// * `a` - Any [`&str`](https://doc.rust-lang.org/std/primitive.str.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::str_to_u16_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(str_to_u16_res("0"), Ok(0_u16));
/// assert_eq!(str_to_u16_res("65535"), Ok(65535_u16));
/// assert_eq!(str_to_u16_res("-1"), Err(InvalidNumber(String::from("-1"))));
/// assert_eq!(str_to_u16_res("65536"), Err(InvalidNumber(String::from("65536"))));
/// ```
pub fn str_to_u16_res(a: &str)
    -> ParseResultU16 {

    match a.parse::<u16>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

/// Parse [`&str`](https://doc.rust-lang.org/std/primitive.str.html) to
/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ushort::str_to_u16("-1");
/// rustils::parse::ushort::str_to_u16("65536");
/// ```
///
/// # Arguments
///
/// * `a` - Any [`&str`](https://doc.rust-lang.org/std/primitive.str.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::ushort::str_to_u16;
///
/// assert_eq!(str_to_u16("0"), 0_u16);
/// assert_eq!(str_to_u16("65535"), 65535_u16);
/// ```
pub fn str_to_u16(a: & str)
    -> u16 {

    match str_to_u16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub type ParseResultU16 = Result<u16, ParseError>;
