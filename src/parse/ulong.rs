use error::ParseError;
use RoundingMode;
use RoundingMode::*;

// <editor-fold> # Traits

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
// </editor-fold>

// <editor-fold> # Functions

// <editor-fold> ## bool

pub fn bool_to_u64_res(a: bool)
    -> ParseResultU64 {

    if a { Ok(1) } else { Ok(0) }
}

pub fn bool_to_u64(a: bool)
    -> u64 {

    if a { 1 } else { 0 }
}
// </editor-fold>

// <editor-fold> ## 8

pub fn i8_to_u64_res(a: i8)
    -> ParseResultU64 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u64) }
}

pub fn i8_to_u64(a: i8)
    -> u64 {

    match i8_to_u64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## 16

pub fn i16_to_u64_res(a: i16)
    -> ParseResultU64 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u64) }
}

pub fn i16_to_u64(a: i16)
    -> u64 {

    match i16_to_u64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## 32

pub fn i32_to_u64_res(a: i32)
    -> ParseResultU64 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u64) }
}

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
// </editor-fold>

// <editor-fold> ## 64

pub fn i64_to_u64_res(a: i64)
    -> ParseResultU64 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u64) }
}

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
// </editor-fold>

// <editor-fold> ## size

pub fn isize_to_u64_res(a: isize)
    -> ParseResultU64 {

    if a < 0 {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as u64) }
}

pub fn isize_to_u64(a: isize)
    -> u64 {

    match isize_to_u64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## string

pub fn string_to_u64_res(a: String)
    -> ParseResultU64 {

    match a.parse::<u64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

pub fn string_to_u64(a: String)
    -> u64 {

    match string_to_u64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn str_to_u64_res(a: &str)
    -> ParseResultU64 {

    match a.parse::<u64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

pub fn str_to_u64(a: &str)
    -> u64 {

    match str_to_u64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// </editor-fold>

// <editor-fold> # Types
pub type ParseResultU64 = Result<u64, ParseError>;
// </editor-fold>

// use parse::{
//     ToI8, ToI16, ToI32, ToI64,
//     ToU8, ToU16, ToU32,
//     ToIsize, ToBool
// };
//
// use parse::{
//     ParseResultI8, ParseResultI16, ParseResultI32, ParseResultI64,
//     ParseResultU8, ParseResultU16, ParseResultU32,
//     ParseResultIsize
// };
//
// use error::ParseError;
//

//
// impl ToI8 for u64 {
//     fn to_i8_res(self) -> ParseResultI8 {
//         let max = i8::max_value() as u64;
//
//         if self > max {
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
// impl ToI32 for u64 {
//     fn to_i32_res(self) -> ParseResultI32 {
//         let max = i32::max_value() as u64;
//
//         if self > max {
//             Err(ParseError::InvalidNumber(self.to_string()))
//         } else { Ok(self as i32) }
//     }
//
//     fn to_i32(self) -> i32 {
//         match self.to_i32_res() {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//

//
// impl ToIsize for u64 {
//     fn to_isize_res(self) -> ParseResultIsize {
//         let max = isize::max_value() as u64;
//
//         if self > max {
//             Err(ParseError::InvalidNumber(self.to_string()))
//         } else { Ok(self as isize) }
//     }
//
//     fn to_isize(self) -> isize {
//         match self.to_isize_res() {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//

//
// impl ToU16 for u64 {
//     fn to_u16_res(self) -> ParseResultU16 {
//         let max = u16::max_value() as u64;
//
//         if self > max {
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
