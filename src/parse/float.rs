use error::ParseError;
use RoundingMode;

pub trait ToF32 {

    fn to_f32_res(self)
        -> ParseResultF32;

    fn to_f32(self)
        -> f32;
}

pub trait ToF32RM {

    fn to_f32_rm_res(self, rm: RoundingMode)
        -> ParseResultF32;

    fn to_f32_rm(self, rm: RoundingMode)
        -> f32;
}

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`f32`](https://doc.rust-lang.org/std/primitive.f32.html)
///
/// If `a` is `false` then returns `Ok(0.0)`.<br>
/// If `a` is `true` then returns `Ok(1.0)`.
///
/// # Arguments
///
/// * `a` - [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::float::bool_to_f32_res;
///
/// assert_eq!(bool_to_f32_res(true), Ok(1.0_f32));
/// assert_eq!(bool_to_f32_res(false), Ok(0.0_f32));
/// ```
pub fn bool_to_f32_res(a: bool)
    -> ParseResultF32 {

    if a { Ok(1.0) } else { Ok(0.0) }
}

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`f32`](https://doc.rust-lang.org/std/primitive.f32.html)
///
/// If `a` is `false` then returns 0.0.<br>
/// If `a` is `true` then returns 1.0.
///
/// # Arguments
///
/// * `a` - [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::float::bool_to_f32;
///
/// assert_eq!(bool_to_f32(true), 1.0_f32);
/// assert_eq!(bool_to_f32(false), 0.0_f32);
/// ```
pub fn bool_to_f32(a: bool)
    -> f32 {

    if a { 1.0 } else { 0.0 }
}

pub fn string_to_f32_res(a: String)
    -> ParseResultF32 {

    match a.parse::<f32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

pub fn string_to_f32(a: String)
    -> f32 {

    match string_to_f32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn str_to_f32_res(a: &str)
    -> ParseResultF32 {

    match a.parse::<f32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

pub fn str_to_f32(a: &str)
    -> f32 {

    match str_to_f32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub type ParseResultF32 = Result<f32, ParseError>;
