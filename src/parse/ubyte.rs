use error::ParseError;
use RoundingMode;
use RoundingMode::*;

pub trait ToU8 {

    fn to_u8_res(self)
        -> ParseResultU8;

    fn to_u8(self)
        -> u8;
}

pub trait ToU8RM {

    fn to_u8_rm_res(self, rm: RoundingMode)
        -> ParseResultU8;

    fn to_u8_rm(self, rm: RoundingMode)
        -> u8;
}

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
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
/// use rustils::parse::ubyte::bool_to_u8_res;
///
/// assert_eq!(bool_to_u8_res(true), Ok(1_u8));
/// assert_eq!(bool_to_u8_res(false), Ok(0_u8));
/// ```
pub fn bool_to_u8_res(a: bool)
    -> ParseResultU8 {

    if a { Ok(1) } else { Ok(0) }
}


/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
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
/// use rustils::parse::ubyte::bool_to_u8;
///
/// assert_eq!(bool_to_u8(true), 1_u8);
/// assert_eq!(bool_to_u8(false), 0_u8);
/// ```
pub fn bool_to_u8(a: bool)
    -> u8 {

    if a { 1 } else { 0 }
}


/// Parse [`i8`](https://doc.rust-lang.org/std/primitive.i8.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Arguments
///
/// * `a` - Any [`i8`](https://doc.rust-lang.org/std/primitive.i8.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::i8_to_u8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(i8_to_u8_res(0_i8), Ok(0_u8));
/// assert_eq!(i8_to_u8_res(127_i8), Ok(127_u8));
/// assert_eq!(i8_to_u8_res(-1_i8), Err(InvalidNumber(String::from("-1"))));
/// ```
pub fn i8_to_u8_res(a: i8)
    -> ParseResultU8 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u8) }
}


/// Parse [`i8`](https://doc.rust-lang.org/std/primitive.i8.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ubyte::i8_to_u8(-1_i8);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`i8`](https://doc.rust-lang.org/std/primitive.i8.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::i8_to_u8;
///
/// assert_eq!(i8_to_u8(0_i8), 0_u8);
/// assert_eq!(i8_to_u8(127_i8), 127_u8);
/// ```
pub fn i8_to_u8(a: i8)
    -> u8 {

    match i8_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Arguments
///
/// * `a` - Any [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::i16_to_u8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(i16_to_u8_res(0_i16), Ok(0_u8));
/// assert_eq!(i16_to_u8_res(255_i16), Ok(255_u8));
/// assert_eq!(i16_to_u8_res(-1_i16), Err(InvalidNumber(String::from("-1"))));
/// assert_eq!(i16_to_u8_res(256_i16), Err(InvalidNumber(String::from("256"))));
/// ```
pub fn i16_to_u8_res(a: i16)
    -> ParseResultU8 {

    let max = u8::max_value() as i16;

    if a < 0 || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u8) }
}

/// Parse [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ubyte::i16_to_u8(-1_i16);
/// rustils::parse::ubyte::i16_to_u8(256_i16);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::i16_to_u8;
///
/// assert_eq!(i16_to_u8(0_i16), 0_u8);
/// assert_eq!(i16_to_u8(255_i16), 255_u8);
/// ```
pub fn i16_to_u8(a: i16)
    -> u8 {

    match i16_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Arguments
///
/// * `a` - Any [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::u16_to_u8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(u16_to_u8_res(0_u16), Ok(0_u8));
/// assert_eq!(u16_to_u8_res(255_u16), Ok(255_u8));
/// assert_eq!(u16_to_u8_res(256_u16), Err(InvalidNumber(String::from("256"))));
/// ```
pub fn u16_to_u8_res(a: u16)
    -> ParseResultU8 {

    let max = u8::max_value() as u16;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u8) }
}

/// Parse [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ubyte::u16_to_u8(256_u16);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::u16_to_u8;
///
/// assert_eq!(u16_to_u8(0_u16), 0_u8);
/// assert_eq!(u16_to_u8(255_u16), 255_u8);
/// ```
pub fn u16_to_u8(a: u16)
    -> u8 {

    match u16_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Arguments
///
/// * `a` - Any [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::i32_to_u8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(i32_to_u8_res(0_i32), Ok(0_u8));
/// assert_eq!(i32_to_u8_res(255_i32), Ok(255_u8));
/// assert_eq!(i32_to_u8_res(-1_i32), Err(InvalidNumber(String::from("-1"))));
/// assert_eq!(i32_to_u8_res(256_i32), Err(InvalidNumber(String::from("256"))));
/// ```
pub fn i32_to_u8_res(a: i32)
    -> ParseResultU8 {

    let max = u8::max_value() as i32;

    if a < 0 || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u8) }
}

/// Parse [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ubyte::i32_to_u8(-1_i32);
/// rustils::parse::ubyte::i32_to_u8(256_i32);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::i32_to_u8;
///
/// assert_eq!(i32_to_u8(0_i32), 0_u8);
/// assert_eq!(i32_to_u8(255_i32), 255_u8);
/// ```
pub fn i32_to_u8(a: i32)
    -> u8 {

    match i32_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Arguments
///
/// * `a` - Any [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::u32_to_u8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(u32_to_u8_res(0_u32), Ok(0_u8));
/// assert_eq!(u32_to_u8_res(255_u32), Ok(255_u8));
/// assert_eq!(u32_to_u8_res(256_u32), Err(InvalidNumber(String::from("256"))));
/// ```
pub fn u32_to_u8_res(a: u32)
    -> ParseResultU8 {

    let max = u8::max_value() as u32;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u8) }
}


/// Parse [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ubyte::u32_to_u8(256_u32);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::u32_to_u8;
///
/// assert_eq!(u32_to_u8(0_u32), 0_u8);
/// assert_eq!(u32_to_u8(255_u32), 255_u8);
/// ```
pub fn u32_to_u8(a: u32)
    -> u8 {

    match u32_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn f32_to_u8_res(a: f32)
    -> ParseResultU8{

    f32_to_u8_rm_res(a, Trunc)
}

pub fn f32_to_u8(a: f32)
    -> u8 {

    f32_to_u8_rm(a, Trunc)
}

pub fn f32_to_u8_rm_res(a: f32, rm: RoundingMode)
    -> ParseResultU8 {

    let max = u8::max_value() as f32;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < 0.0 || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as u8) }
}

pub fn f32_to_u8_rm(a: f32, rm: RoundingMode)
    -> u8 {

    match f32_to_u8_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Arguments
///
/// * `a` - Any [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::i64_to_u8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(i64_to_u8_res(0_i64), Ok(0_u8));
/// assert_eq!(i64_to_u8_res(255_i64), Ok(255_u8));
/// assert_eq!(i64_to_u8_res(-1_i64), Err(InvalidNumber(String::from("-1"))));
/// assert_eq!(i64_to_u8_res(256_i64), Err(InvalidNumber(String::from("256"))));
/// ```
pub fn i64_to_u8_res(a: i64)
    -> ParseResultU8 {

    let max = u8::max_value() as i64;

    if a < 0 || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u8) }
}

/// Parse [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ubyte::i64_to_u8(-1_i64);
/// rustils::parse::ubyte::i64_to_u8(256_i64);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::i64_to_u8;
///
/// assert_eq!(i64_to_u8(0_i64), 0_u8);
/// assert_eq!(i64_to_u8(255_i64), 255_u8);
/// ```
pub fn i64_to_u8(a: i64)
    -> u8 {

    match i64_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Arguments
///
/// * `a` - Any [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::u64_to_u8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(u64_to_u8_res(0_u64), Ok(0_u8));
/// assert_eq!(u64_to_u8_res(255_u64), Ok(255_u8));
/// assert_eq!(u64_to_u8_res(256_u64), Err(InvalidNumber(String::from("256"))));
/// ```
pub fn u64_to_u8_res(a: u64)
    -> ParseResultU8 {

    let max = u8::max_value() as u64;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u8) }
}

/// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ubyte::u64_to_u8(256_u64);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::u64_to_u8;
///
/// assert_eq!(u64_to_u8(0_u64), 0_u8);
/// assert_eq!(u64_to_u8(255_u64), 255_u8);
/// ```
pub fn u64_to_u8(a: u64)
    -> u8 {

    match u64_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn f64_to_u8_res(a: f64)
    -> ParseResultU8{

    f64_to_u8_rm_res(a, Trunc)
}

pub fn f64_to_u8(a: f64)
    -> u8 {

    f64_to_u8_rm(a, Trunc)
}

pub fn f64_to_u8_rm_res(a: f64, rm: RoundingMode)
    -> ParseResultU8 {

    let max = u8::max_value() as f64;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < 0.0 || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as u8) }
}

pub fn f64_to_u8_rm(a: f64, rm: RoundingMode)
    -> u8 {

    match f64_to_u8_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Arguments
///
/// * `a` - Any [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::usize_to_u8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(usize_to_u8_res(0_usize), Ok(0_u8));
/// assert_eq!(usize_to_u8_res(255_usize), Ok(255_u8));
/// assert_eq!(usize_to_u8_res(256_usize), Err(InvalidNumber(String::from("256"))));
/// ```
pub fn usize_to_u8_res(a: usize)
    -> ParseResultU8 {

    let max = u8::max_value() as usize;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u8) }
}

/// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ubyte::usize_to_u8(256_usize);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::usize_to_u8;
///
/// assert_eq!(usize_to_u8(0_usize), 0_u8);
/// assert_eq!(usize_to_u8(255_usize), 255_u8);
/// ```
pub fn usize_to_u8(a: usize)
    -> u8 {

    match usize_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Arguments
///
/// * `a` - Any [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::isize_to_u8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(isize_to_u8_res(0_isize), Ok(0_u8));
/// assert_eq!(isize_to_u8_res(255_isize), Ok(255_u8));
/// assert_eq!(isize_to_u8_res(-1_isize), Err(InvalidNumber(String::from("-1"))));
/// assert_eq!(isize_to_u8_res(256_isize), Err(InvalidNumber(String::from("256"))));
/// ```
pub fn isize_to_u8_res(a: isize)
    -> ParseResultU8 {

    let max = u8::max_value() as isize;

    if a < 0 || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u8) }
}

/// Parse [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ubyte::isize_to_u8(-1_isize);
/// rustils::parse::ubyte::isize_to_u8(256_isize);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::isize_to_u8;
///
/// assert_eq!(isize_to_u8(0_isize), 0_u8);
/// assert_eq!(isize_to_u8(255_isize), 255_u8);
/// ```
pub fn isize_to_u8(a: isize)
    -> u8 {

    match isize_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`String`](https://doc.rust-lang.org/std/string/struct.String.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Arguments
///
/// * `a` - Any [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::string_to_u8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(string_to_u8_res("0".to_string()), Ok(0_u8));
/// assert_eq!(string_to_u8_res("255".to_string()), Ok(255_u8));
/// assert_eq!(string_to_u8_res("-1".to_string()), Err(InvalidNumber(String::from("-1"))));
/// assert_eq!(string_to_u8_res("256".to_string()), Err(InvalidNumber(String::from("256"))));
/// ```
pub fn string_to_u8_res(a: String)
    -> ParseResultU8 {

    match a.parse::<u8>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

/// Parse [`String`](https://doc.rust-lang.org/std/string/struct.String.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ubyte::string_to_u8("-1".to_string());
/// rustils::parse::ubyte::string_to_u8("256".to_string());
/// ```
///
/// # Arguments
///
/// * `a` - Any [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::string_to_u8;
///
/// assert_eq!(string_to_u8("0".to_string()), 0_u8);
/// assert_eq!(string_to_u8("255".to_string()), 255_u8);
/// ```
pub fn string_to_u8(a: String)
    -> u8 {

    match string_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`&str`](https://doc.rust-lang.org/std/primitive.str.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Arguments
///
/// * `a` - Any [`&str`](https://doc.rust-lang.org/std/primitive.str.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::str_to_u8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(str_to_u8_res("0"), Ok(0_u8));
/// assert_eq!(str_to_u8_res("255"), Ok(255_u8));
/// assert_eq!(str_to_u8_res("-1"), Err(InvalidNumber(String::from("-1"))));
/// assert_eq!(str_to_u8_res("256"), Err(InvalidNumber(String::from("256"))));
/// ```
pub fn str_to_u8_res(a: &str)
    -> ParseResultU8 {

    match a.parse::<u8>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

/// Parse [`&str`](https://doc.rust-lang.org/std/primitive.str.html) to
/// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::ubyte::str_to_u8("-1");
/// rustils::parse::ubyte::str_to_u8("256");
/// ```
///
/// # Arguments
///
/// * `a` - Any [`&str`](https://doc.rust-lang.org/std/primitive.str.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::ubyte::str_to_u8;
///
/// assert_eq!(str_to_u8("0"), 0_u8);
/// assert_eq!(str_to_u8("255"), 255_u8);
/// ```
pub fn str_to_u8(a: &str)
    -> u8 {

    match str_to_u8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub type ParseResultU8 = Result<u8, ParseError>;
