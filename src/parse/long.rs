use error::ParseError;
use RoundingMode;
use RoundingMode::*;

// <editor-fold> # Traits
pub trait ToI64 {

    fn to_i64_res(self)
        -> ParseResultI64;

    fn to_i64(self)
        -> i64;
}

pub trait ToI64RM {

    fn to_i64_rm_res(self, rm: RoundingMode)
        -> ParseResultI64;

    fn to_i64_rm(self, rm: RoundingMode)
        -> i64;
}
// </editor-fold>

// <editor-fold> # Functions

// <editor-fold> ## bool

pub fn bool_to_i64_res(a: bool)
    -> ParseResultI64 {

    if a { Ok(1) } else { Ok(0) }
}

pub fn bool_to_i64(a: bool)
    -> i64 {

    if a { 1 } else { 0 }
}
// </editor-fold>

// <editor-fold> ## 32

pub fn f32_to_i64_res(a: f32)
    -> ParseResultI64 {

    f32_to_i64_rm_res(a, Trunc)
}

pub fn f32_to_i64(a: f32)
    -> i64 {

    f32_to_i64_rm(a, Trunc)
}

pub fn f32_to_i64_rm_res(a: f32, rm: RoundingMode)
    -> ParseResultI64 {

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
    } else { Ok(x as i64) }
}

pub fn f32_to_i64_rm(a: f32, rm: RoundingMode)
    -> i64 {

    match a.to_i64_rm_res(rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## 64

pub fn u64_to_i64_res(a: u64)
    -> ParseResultI64 {

    let max = i64::max_value() as u64;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i64) }
}

pub fn u64_to_i64(a: u64)
    -> i64 {

    match u64_to_i64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn f64_to_i64_res(a: f64)
    -> ParseResultI64 {

    f64_to_i64_rm_res(a, Trunc)
}

pub fn f64_to_i64(a: f64)
    -> i64 {

    f64_to_i64_rm(a, Trunc)
}

pub fn f64_to_i64_rm_res(a: f64, rm: RoundingMode)
    -> ParseResultI64 {

    let min = -9007199254740991_f64;
    let max = 9007199254740991_f64;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < min || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as i64) }
}

pub fn f64_to_i64_rm(a: f64, rm: RoundingMode)
    -> i64 {

    match f64_to_i64_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## size

pub fn usize_to_i64_res(a: usize)
    -> ParseResultI64 {

    let max = i64::max_value() as usize;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i64) }
}

pub fn usize_to_i64(a: usize)
    -> i64 {

    match usize_to_i64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## string

pub fn string_to_i64_res(a: String)
    -> ParseResultI64 {

    match a.parse::<i64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

pub fn string_to_i64(a: String)
    -> i64 {

    match string_to_i64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn str_to_i64_res(a: &str)
    -> ParseResultI64 {

    match a.parse::<i64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

pub fn str_to_i64(a: &str)
    -> i64 {

    match str_to_i64_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// </editor-fold>

// <editor-fold> # Types
pub type ParseResultI64 = Result<i64, ParseError>;
// </editor-fold>

// use parse::{
//     ToI8, ToI16, ToI32,
//     ToU8, ToU16, ToU32, ToU64,
//     ToUsize,
//     ToBool
// };
//
// use parse::{
//     ParseResultI8, ParseResultI16, ParseResultI32,
//     ParseResultU8, ParseResultU16, ParseResultU32, ParseResultU64,
//     ParseResultUsize
// };
//
// use error::ParseError;
//

//
// impl ToI8 for i64 {
//     fn to_i8_res(self) -> ParseResultI8 {
//         let min = i8::min_value() as i64;
//         let max = i8::max_value() as i64;
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
// impl ToI32 for i64 {
//     fn to_i32_res(self) -> ParseResultI32 {
//         let min = i32::min_value() as i64;
//         let max = i32::max_value() as i64;
//
//         if self < min || self > max {
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
// impl ToU8 for i64 {
//     fn to_u8_res(self) -> ParseResultU8 {
//         let max = u8::max_value() as i64;
//
//         if self < 0 || self > max {
//             Err(ParseError::InvalidNumber(self.to_string()))
//         } else { Ok(self as u8) }
//     }
//
//     fn to_u8(self) -> u8 {
//         match self.to_u8_res() {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//
// impl ToU16 for i64 {
//     fn to_u16_res(self) -> ParseResultU16 {
//         let max = u16::max_value() as i64;
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
// impl ToU32 for i64 {
//     fn to_u32_res(self) -> ParseResultU32 {
//         let max = u32::max_value() as i64;
//
//         if self < 0 || self > max {
//             Err(ParseError::InvalidNumber(self.to_string()))
//         } else { Ok(self as u32) }
//     }
//
//     fn to_u32(self) -> u32 {
//         match self.to_u32_res() {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//
// impl ToU64 for i64 {
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
// impl ToUsize for i64 {
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
