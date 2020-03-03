use error::ParseError;
use RoundingMode;
use RoundingMode::*;

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

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
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
/// use rustils::parse::short::bool_to_i16_res;
///
/// assert_eq!(bool_to_i16_res(true), Ok(1_i16));
/// assert_eq!(bool_to_i16_res(false), Ok(0_i16));
/// ```
pub fn bool_to_i16_res(a: bool)
    -> ParseResultI16 {

    if a { Ok(1) } else { Ok(0) }
}

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
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
/// use rustils::parse::short::bool_to_i16;
///
/// assert_eq!(bool_to_i16(true), 1_i16);
/// assert_eq!(bool_to_i16(false), 0_i16);
/// ```
pub fn bool_to_i16(a: bool)
    -> i16 {

    if a { 1 } else { 0 }
}

/// Parse [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) to
/// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
///
/// # Arguments
///
/// * `a` - Any [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::short::u16_to_i16_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(u16_to_i16_res(0_u16), Ok(0_i16));
/// assert_eq!(u16_to_i16_res(32767_u16), Ok(32767_i16));
/// assert_eq!(u16_to_i16_res(32768_u16), Err(InvalidNumber(String::from("32768"))));
/// ```
pub fn u16_to_i16_res(a: u16)
    -> ParseResultI16 {

    let max = i16::max_value() as u16;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i16) }
}

/// Parse [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) to
/// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::short::u16_to_i16(32768_u16);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::short::u16_to_i16;
///
/// assert_eq!(u16_to_i16(0_u16), 0_i16);
/// assert_eq!(u16_to_i16(32767_u16), 32767_i16);
/// ```
pub fn u16_to_i16(a: u16)
    -> i16 {

    match u16_to_i16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

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

/// Parse [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) to
/// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
///
/// # Arguments
///
/// * `a` - Any [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::short::u32_to_i16_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(u32_to_i16_res(0_u32), Ok(0_i16));
/// assert_eq!(u32_to_i16_res(32767_u32), Ok(32767_i16));
/// assert_eq!(u32_to_i16_res(32768_u32), Err(InvalidNumber(String::from("32768"))));
/// ```
pub fn u32_to_i16_res(a: u32)
    -> ParseResultI16 {

    let max = i16::max_value() as u32;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i16) }
}

/// Parse [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) to
/// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::short::u32_to_i16(32768_u32);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::short::u32_to_i16;
///
/// assert_eq!(u32_to_i16(0_u32), 0_i16);
/// assert_eq!(u32_to_i16(32767_u32), 32767_i16);
/// ```
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

/// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
/// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
///
/// # Arguments
///
/// * `a` - Any [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::short::u64_to_i16_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(u64_to_i16_res(0_u64), Ok(0_i16));
/// assert_eq!(u64_to_i16_res(32767_u64), Ok(32767_i16));
/// assert_eq!(u64_to_i16_res(32768_u64), Err(InvalidNumber(String::from("32768"))));
/// ```
pub fn u64_to_i16_res(a: u64)
    -> ParseResultI16 {

    let max = i16::max_value() as u64;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i16) }
}

/// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
/// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::short::u64_to_i16(32768_u64);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::short::u64_to_i16;
///
/// assert_eq!(u64_to_i16(0_u64), 0_i16);
/// assert_eq!(u64_to_i16(32767_u64), 32767_i16);
/// ```
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

/// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
/// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
///
/// # Arguments
///
/// * `a` - Any [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::short::usize_to_i16_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(usize_to_i16_res(0_usize), Ok(0_i16));
/// assert_eq!(usize_to_i16_res(32767_usize), Ok(32767_i16));
/// assert_eq!(usize_to_i16_res(32768_usize), Err(InvalidNumber(String::from("32768"))));
/// ```
pub fn usize_to_i16_res(a: usize)
    -> ParseResultI16 {

    let max = i16::max_value() as usize;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i16) }
}

/// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
/// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::short::usize_to_i16(32768_usize);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::short::usize_to_i16;
///
/// assert_eq!(usize_to_i16(0_usize), 0_i16);
/// assert_eq!(usize_to_i16(32767_usize), 32767_i16);
/// ```
pub fn usize_to_i16(a: usize)
    -> i16 {

    match usize_to_i16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`String`](https://doc.rust-lang.org/std/string/struct.String.html) to
/// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
///
/// # Arguments
///
/// * `a` - Any [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::short::string_to_i16_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(string_to_i16_res("-32768".to_string()), Ok(-32768_i16));
/// assert_eq!(string_to_i16_res("32767".to_string()), Ok(32767_i16));
///
/// assert_eq!(
///     string_to_i16_res("-32769".to_string()),
///     Err(InvalidNumber(String::from("-32769")))
/// );
///
/// assert_eq!(
///     string_to_i16_res("32768".to_string()),
///     Err(InvalidNumber(String::from("32768")))
/// );
/// ```
pub fn string_to_i16_res(a: String)
    -> ParseResultI16 {

    match a.parse::<i16>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

/// Parse [`String`](https://doc.rust-lang.org/std/string/struct.String.html) to
/// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::short::string_to_i16("-32769".to_string());
/// rustils::parse::short::string_to_i16("32768".to_string());
/// ```
///
/// # Arguments
///
/// * `a` - Any [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::short::string_to_i16;
///
/// assert_eq!(string_to_i16("-32768".to_string()), -32768_i16);
/// assert_eq!(string_to_i16("32767".to_string()), 32767_i16);
/// ```
pub fn string_to_i16(a: String)
    -> i16 {

    match string_to_i16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`&str`](https://doc.rust-lang.org/std/primitive.str.html) to
/// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
///
/// # Arguments
///
/// * `a` - Any [`&str`](https://doc.rust-lang.org/std/primitive.str.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::short::str_to_i16_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(str_to_i16_res("-32768"), Ok(-32768_i16));
/// assert_eq!(str_to_i16_res("32767"), Ok(32767_i16));
/// assert_eq!(str_to_i16_res("-32769"), Err(InvalidNumber(String::from("-32769"))));
/// assert_eq!(str_to_i16_res("32768"), Err(InvalidNumber(String::from("32768"))));
/// ```
pub fn str_to_i16_res(a: &str)
    -> ParseResultI16 {

    match a.parse::<i16>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

/// Parse [`&str`](https://doc.rust-lang.org/std/primitive.str.html) to
/// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::short::str_to_i16("-32769");
/// rustils::parse::short::str_to_i16("32768");
/// ```
///
/// # Arguments
///
/// * `a` - Any [`&str`](https://doc.rust-lang.org/std/primitive.str.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::short::str_to_i16;
///
/// assert_eq!(str_to_i16("-32768"), -32768_i16);
/// assert_eq!(str_to_i16("32767"), 32767_i16);
/// ```
pub fn str_to_i16(a: &str)
    -> i16 {

    match str_to_i16_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub type ParseResultI16 = Result<i16, ParseError>;
