use error::ParseError;
use parse::RoundingMode;
use parse::RoundingMode::*;

// <editor-fold> # Traits

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
// </editor-fold>

// <editor-fold> # Functions

// <editor-fold> ## bool

pub fn bool_to_i32_res(a: bool)
    -> ParseResultI32 {

    if a { Ok(1) } else { Ok(0) }
}

pub fn bool_to_i32(a: bool)
    -> i32 {

    if a { 1 } else { 0 }
}
// </editor-fold>

// <editor-fold> ## 32

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
// </editor-fold>

// <editor-fold> ## 64

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

pub fn u64_to_i32_res(a: u64)
    -> ParseResultI32 {

    let max = i32::max_value() as u64;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i32) }
}

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
// </editor-fold>

// <editor-fold> ## size

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

pub fn usize_to_i32_res(a: usize)
    -> ParseResultI32 {

    let max = i32::max_value() as usize;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i32) }
}

pub fn usize_to_i32(a: usize)
    -> i32 {

    match usize_to_i32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## string

pub fn string_to_i32_res(a: String)
    -> ParseResultI32 {

    match a.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

pub fn string_to_i32(a: String)
    -> i32 {

    match string_to_i32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn str_to_i32_res(a: &str)
    -> ParseResultI32 {

    match a.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

pub fn str_to_i32(a: &str)
    -> i32 {

    match str_to_i32_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// </editor-fold>

// <editor-fold> # Types
pub type ParseResultI32 = Result<i32,ParseError>;
// </editor-fold>

// use parse::{
//     ToI8, ToI16,
//     ToU8, ToU16, ToU32, ToU64,
//     ToUsize,
//     ToBool
// };
//
// use parse::{
//     ParseResultI8, ParseResultI16,
//     ParseResultU8, ParseResultU16, ParseResultU32, ParseResultU64,
//     ParseResultUsize
// };
//
// use error::ParseError;
//

//
// impl ToI8 for i32 {
//     fn to_i8_res(self) -> ParseResultI8 {
//         let min = i8::min_value() as i32;
//         let max = i8::max_value() as i32;
//
//         if self < min || self > max {
//             Err(ParseError::InvalidNumber(self.to_string()))
//         } else { Ok(self as i8) }
//     }
//
//     fn to_i8(self) -> i8 {
//         match self.to_i8_res() {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//

//

//
// impl ToU16 for i32 {
//     fn to_u16_res(self) -> ParseResultU16 {
//         let max = u16::max_value() as i32;
//
//         if self < 0 || self > max {
//             Err(ParseError::InvalidNumber(self.to_string()))
//         } else { Ok(self as u16) }
//     }
//
//     fn to_u16(self) -> u16 {
//         match self.to_u16_res() {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//

//
// impl ToU64 for i32 {
//     fn to_u64_res(self) -> ParseResultU64 {
//         if self < 0 {
//             Err(ParseError::InvalidNumber(self.to_string()))
//         } else { Ok(self as u64) }
//     }
//
//     fn to_u64(self) -> u64 {
//         match self.to_u64_res() {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//
// impl ToUsize for i32 {
//     fn to_usize_res(self) -> ParseResultUsize {
//         if self < 0 {
//             Err(ParseError::InvalidNumber(self.to_string()))
//         } else { Ok(self as usize) }
//     }
//
//     fn to_usize(self) -> usize {
//         match self.to_usize_res() {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
