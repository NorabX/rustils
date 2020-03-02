use error::ParseError;
use RoundingMode;

pub trait ToF64 {

    fn to_f64_res(self)
        -> ParseResultF64;

    fn to_f64(self)
        -> f64;
}

pub trait ToF64RM {

    fn to_f64_rm_res(self, rm: RoundingMode)
        -> ParseResultF64;

    fn to_f64_rm(self, rm: RoundingMode)
        -> f64;
}

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`f64`](https://doc.rust-lang.org/std/primitive.f64.html)
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
/// use rustils::parse::double::bool_to_f64_res;
///
/// assert_eq!(bool_to_f64_res(true), Ok(1.0_f64));
/// assert_eq!(bool_to_f64_res(false), Ok(0.0_f64));
/// ```
pub fn bool_to_f64_res(a: bool)
    -> ParseResultF64 {

    if a { Ok(1.0) } else { Ok(0.0) }
}

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`f64`](https://doc.rust-lang.org/std/primitive.f64.html)
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
/// use rustils::parse::double::bool_to_f64;
///
/// assert_eq!(bool_to_f64(true), 1_f64);
/// assert_eq!(bool_to_f64(false), 0_f64);
/// ```
pub fn bool_to_f64(a: bool)
    -> f64 {

    if a { 1.0 } else { 0.0 }
}

pub fn string_to_f64_res(a: String)
    -> ParseResultF64 {

    match a.parse::<f64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

pub fn string_to_f64(a: String)
    -> f64 {

    match string_to_f64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn str_to_f64_res(a: &str)
    -> ParseResultF64 {

    match a.parse::<f64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

pub fn str_to_f64(a: &str)
    -> f64 {

    match str_to_f64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub type ParseResultF64 = Result<f64, ParseError>;
