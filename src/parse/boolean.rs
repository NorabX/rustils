// <editor-fold> # Uses
use error::ParseError;
// </editor-fold>

// <editor-fold> # Traits
pub trait ToBool {

    fn to_bool(self)
        -> bool;
}
// </editor-fold>

// <editor-fold> # Functions

// <editor-fold> ## 8


/// Parse i8 to bool
///
/// If `a` is 0 then returns false,
/// otherwise true.
///
/// # Arguments
///
/// * `a` - Any i8 number
///
/// # Examples
///
/// ```
/// use rustils::parse::boolean::i8_to_bool;
///
/// let a = 42_i8;
/// let b = -1_i8;
/// let c = 0_i8;
///
/// assert!(i8_to_bool(a));
/// assert!(i8_to_bool(b));
/// assert!(!i8_to_bool(c));
/// ```
pub fn i8_to_bool(a: i8)
    -> bool {

    if a == 0 { false } else { true }
}

pub fn u8_to_bool(a: u8)
    -> bool {

    if a == 0 { false } else { true }
}
// </editor-fold>

// <editor-fold> ## 16
pub fn i16_to_bool(a: i16)
    -> bool {

    if a == 0 { false } else { true }
}

pub fn u16_to_bool(a: u16)
    -> bool {

    if a == 0 { false } else { true }
}
// </editor-fold>

// <editor-fold> ## 32
pub fn i32_to_bool(a: i32)
    -> bool {

    if a == 0 { false } else { true }
}

pub fn u32_to_bool(a: u32)
    -> bool {

    if a == 0 { false } else { true }
}

pub fn f32_to_bool(a: f32)
    -> bool {

    if a.is_nan() || a == 0.0 { false } else { true }
}
// </editor-fold>

// <editor-fold> ## 64
pub fn i64_to_bool(a: i64)
    -> bool {

    if a == 0 { false } else { true }
}

pub fn u64_to_bool(a: u64)
    -> bool {

    if a == 0 { false } else { true }
}

pub fn f64_to_bool(a: f64)
    -> bool {

    if a.is_nan() || a == 0.0 { false } else { true }
}
// </editor-fold>

// <editor-fold> ## size
pub fn isize_to_bool(a: isize)
    -> bool {

    if a == 0 { false } else { true }
}

pub fn usize_to_bool(a: usize)
    -> bool {

    if a == 0 { false } else { true }
}
// </editor-fold>

// <editor-fold> ## string
pub fn string_to_bool(a: String)
    -> bool {

    let x = &*a.to_lowercase();

    match x {
        "true" => true,
        "t" => true,
        "false" => false,
        "f" => false,
        "yes" => true,
        "y" => true,
        "no" => false,
        "n" => false,
        "1" => true,
        "0" => false,
        _ => panic!("{}", ParseError::InvalidString(a))
    }
}

pub fn str_to_bool(a: &str)
    -> bool {

    let x = &*a.to_lowercase();

    match x {
        "true" => true,
        "t" => true,
        "false" => false,
        "f" => false,
        "yes" => true,
        "y" => true,
        "no" => false,
        "n" => false,
        "1" => true,
        "0" => false,
        _ => panic!("{}", ParseError::InvalidString(a.to_string()))
    }
}
// </editor-fold>

// </editor-fold>

// use parse::{
//     ToI8, ToI16, ToI32, ToI64,
//     ToU8, ToU16, ToU32, ToU64,
//     ToF32, ToF64, ToIsize, ToUsize
// };
// use parse::{
//     ParseResultI8, ParseResultI16, ParseResultI32, ParseResultI64,
//     ParseResultU8, ParseResultU16, ParseResultU32, ParseResultU64,
//     ParseResultF32, ParseResultF64, ParseResultIsize, ParseResultUsize
// };
//
// impl ToI8 for bool {
//     fn to_i8_res(self) -> ParseResultI8 {
//         if self { Ok(1) } else { Ok(0) }
//     }
//
//     fn to_i8(self) -> i8 {
//         if self { 1 } else { 0 }
//     }
// }
//

//
// impl ToI32 for bool {
//     fn to_i32_res(self) -> ParseResultI32 {
//         if self { Ok(1) } else { Ok(0) }
//     }
//
//     fn to_i32(self) -> i32 {
//         if self { 1 } else { 0 }
//     }
// }
//
// impl ToI64 for bool {
//     fn to_i64_res(self) -> ParseResultI64 {
//         if self { Ok(1) } else { Ok(0) }
//     }
//
//     fn to_i64(self) -> i64 {
//         if self { 1 } else { 0 }
//     }
// }
//

//
// impl ToU16 for bool {
//     fn to_u16_res(self) -> ParseResultU16 {
//         if self { Ok(1) } else { Ok(0) }
//     }
//
//     fn to_u16(self) -> u16 {
//         if self { 1 } else { 0 }
//     }
// }
//
// impl ToU32 for bool {
//     fn to_u32_res(self) -> ParseResultU32 {
//         if self { Ok(1) } else { Ok(0) }
//     }
//
//     fn to_u32(self) -> u32 {
//         if self { 1 } else { 0 }
//     }
// }
//
// impl ToU64 for bool {
//     fn to_u64_res(self) -> ParseResultU64 {
//         if self { Ok(1) } else { Ok(0) }
//     }
//
//     fn to_u64(self) -> u64 {
//         if self { 1 } else { 0 }
//     }
// }
//
// impl ToF32 for bool {
//     fn to_f32_res(self) -> ParseResultF32 {
//         if self { Ok(1.0) } else { Ok(0.0) }
//     }
//
//     fn to_f32(self) -> f32 {
//         if self { 1.0 } else { 0.0 }
//     }
// }
//
// impl ToF64 for bool {
//     fn to_f64_res(self) -> ParseResultF64 {
//         if self { Ok(1.0) } else { Ok(0.0) }
//     }
//
//     fn to_f64(self) -> f64 {
//         if self { 1.0 } else { 0.0 }
//     }
// }
//
// impl ToIsize for bool {
//     fn to_isize_res(self) -> ParseResultIsize {
//         if self { Ok(1) } else { Ok(0) }
//     }
//
//     fn to_isize(self) -> isize {
//         if self { 1 } else { 0 }
//     }
// }
//
// impl ToUsize for bool {
//     fn to_usize_res(self) -> ParseResultUsize {
//         if self { Ok(1) } else { Ok(0) }
//     }
//
//     fn to_usize(self) -> usize {
//         if self { 1 } else { 0 }
//     }
// }
