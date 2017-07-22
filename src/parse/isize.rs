use error::ParseError;
use parse::RoundingMode;
use parse::RoundingMode::*;

// <editor-fold> # Traits

pub trait ToIsize {

    fn to_isize_res(self)
        -> ParseResultIsize;

    fn to_isize(self)
        -> isize;
}

pub trait ToIsizeRM {

    fn to_isize_rm_res(self, rm: RoundingMode)
        -> ParseResultIsize;

    fn to_isize_rm(self, rm: RoundingMode)
        -> isize;
}
// </editor-fold>

// <editor-fold> # Functions

// <editor-fold> ## bool

pub fn bool_to_isize_res(a: bool)
    -> ParseResultIsize {

    if a { Ok(1) } else { Ok(0) }
}

pub fn bool_to_isize(a: bool)
    -> isize {

    if a { 1 } else { 0 }
}
// </editor-fold>

// <editor-fold> ## 32

pub fn f32_to_isize_res(a: f32)
    -> ParseResultIsize {

    f32_to_isize_rm_res(a, Trunc)
}

pub fn f32_to_isize(a: f32)
    -> isize {

    f32_to_isize_rm(a, Trunc)
}

pub fn f32_to_isize_rm_res(a: f32, rm: RoundingMode)
    -> ParseResultIsize {

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
    } else { Ok(x as isize) }
}

pub fn f32_to_isize_rm(a: f32, rm: RoundingMode)
    -> isize {

    match a.to_isize_rm_res(rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## 64

pub fn u64_to_isize_res(a: u64)
    -> ParseResultIsize {

    let max = isize::max_value() as u64;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as isize) }
}

pub fn u64_to_isize(a: u64)
    -> isize {

    match u64_to_isize_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn f64_to_isize_res(a: f64)
    -> ParseResultIsize {

    f64_to_isize_rm_res(a, Trunc)
}

pub fn f64_to_isize(a: f64)
    -> isize {

    f64_to_isize_rm(a, Trunc)
}

pub fn f64_to_isize_rm_res(a: f64, rm: RoundingMode)
    -> ParseResultIsize {

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
    } else { Ok(x as isize) }
}

pub fn f64_to_isize_rm(a: f64, rm: RoundingMode)
    -> isize {

    match f64_to_isize_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## size

pub fn usize_to_isize_res(a: usize)
    -> ParseResultIsize {

    let max = isize::max_value() as usize;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as isize) }
}

pub fn usize_to_isize(a: usize)
    -> isize {

    match usize_to_isize_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## String

pub fn string_to_isize_res(a: String)
    -> ParseResultIsize {

    match a.parse::<isize>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

pub fn string_to_isize(a: String)
    -> isize {

    match string_to_isize_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn str_to_isize_res(a: &str)
    -> ParseResultIsize {

    match a.parse::<isize>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

pub fn str_to_isize(a: &str)
    -> isize {

    match str_to_isize_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// </editor-fold>

// <editor-fold> # Types

pub type ParseResultIsize = Result<isize, ParseError>;
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
// impl ToI8 for isize {
//     fn to_i8_res(self) -> ParseResultI8 {
//         let min = i8::min_value() as isize;
//         let max = i8::max_value() as isize;
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
// impl ToI32 for isize {
//     fn to_i32_res(self) -> ParseResultI32 {
//         let min = i32::min_value() as isize;
//         let max = i32::max_value() as isize;
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

//
// impl ToU16 for isize {
//     fn to_u16_res(self) -> ParseResultU16 {
//         let max = u16::max_value() as isize;
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

//
// impl ToUsize for isize {
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
