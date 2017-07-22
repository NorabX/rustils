use error::ParseError;
use RoundingMode;

// <editor-fold> # Traits

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
// </editor-fold>

// <editor-fold> # Functions

// <editor-fold> ## bool

pub fn bool_to_f64_res(a: bool)
    -> ParseResultF64 {

    if a { Ok(1.0) } else { Ok(0.0) }
}

pub fn bool_to_f64(a: bool)
    -> f64 {

    if a { 1.0 } else { 0.0 }
}
// </editor-fold>

// <editor-fold> ## string

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
// </editor-fold>

// </editor-fold>

// <editor-fold> # Types
pub type ParseResultF64 = Result<f64, ParseError>;
// </editor-fold>

// use parse::{
//     ToI8, ToI16, ToI32, ToI64,
//     ToU8, ToU16, ToU32, ToU64,
//     ToIsize, ToUsize,
//     ToBool,
//     ToI8RM, ToI16RM, ToI32RM, ToI64RM,
//     ToU8RM, ToU16RM, ToU32RM, ToU64RM,
//     ToIsizeRM, ToUsizeRM
// };
//
//
// use parse::{
//     ParseResultI8, ParseResultI16, ParseResultI32, ParseResultI64,
//     ParseResultU8, ParseResultU16, ParseResultU32, ParseResultU64,
//     ParseResultIsize, ParseResultUsize
// };
//
// use error::ParseError;
// use RoundingMode;
// use RoundingMode:: { Trunc, Ceil, Floor, Round };
//

//
// impl ToI8 for f64 {
//     fn to_i8_res(self) -> ParseResultI8{
//         self.to_i8_rm_res(Trunc)
//     }
//
//     fn to_i8(self) -> i8 {
//         self.to_i8_rm(Trunc)
//     }
// }
//
// impl ToI8RM for f64 {
//     fn to_i8_rm_res(self, rm: RoundingMode) -> ParseResultI8 {
//         let min = i8::min_value() as f64;
//         let max = i8::max_value() as f64;
//
//         let x = match rm {
//             Round => self.round(),
//             Ceil => self.ceil(),
//             Floor => self.floor(),
//             Trunc => self.trunc()
//         };
//
//         if x.is_nan() || x < min || x > max {
//             Err(ParseError::InvalidNumber(self.to_string()))
//         } else { Ok(x as i8) }
//     }
//
//     fn to_i8_rm(self, rm: RoundingMode) -> i8 {
//         match self.to_i8_rm_res(rm) {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//

//
// impl ToI32 for f64 {
//     fn to_i32_res(self) -> ParseResultI32 {
//         self.to_i32_rm_res(Trunc)
//     }
//
//     fn to_i32(self) -> i32 {
//         self.to_i32_rm(Trunc)
//     }
// }
//
// impl ToI32RM for f64 {
//     fn to_i32_rm_res(self, rm: RoundingMode) -> ParseResultI32 {
//         let min = i32::min_value() as f64;
//         let max = i32::max_value() as f64;
//
//         let x = match rm {
//             Round => self.round(),
//             Ceil => self.ceil(),
//             Floor => self.floor(),
//             Trunc => self.trunc()
//         };
//
//         if x.is_nan() || x < min || x > max {
//             Err(ParseError::InvalidNumber(self.to_string()))
//         } else { Ok(x as i32) }
//     }
//
//     fn to_i32_rm(self, rm: RoundingMode) -> i32 {
//         match self.to_i32_rm_res(rm) {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//

//

//
// impl ToIsize for f64 {
//     fn to_isize_res(self) -> ParseResultIsize {
//         self.to_isize_rm_res(Trunc)
//     }
//
//     fn to_isize(self) -> isize {
//         self.to_isize_rm(Trunc)
//     }
// }
//
// impl ToIsizeRM for f64 {
//     fn to_isize_rm_res(self, rm: RoundingMode) -> ParseResultIsize {
//         let min = -9007199254740991_f64;
//         let max = 9007199254740991_f64;
//
//         let x = match rm {
//             Round => self.round(),
//             Ceil => self.ceil(),
//             Floor => self.floor(),
//             Trunc => self.trunc()
//         };
//
//         if x.is_nan() || x < min || x > max {
//             Err(ParseError::InvalidNumber(self.to_string()))
//         } else { Ok(x as isize) }
//     }
//
//     fn to_isize_rm(self, rm: RoundingMode) -> isize {
//         match self.to_isize_rm_res(rm) {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//

//
// impl ToU16 for f64 {
//     fn to_u16_res(self) -> ParseResultU16 {
//         self.to_u16_rm_res(Trunc)
//     }
//
//     fn to_u16(self) -> u16 {
//         self.to_u16_rm(Trunc)
//     }
// }
//
// impl ToU16RM for f64 {
//     fn to_u16_rm_res(self, rm: RoundingMode) -> ParseResultU16 {
//         let max = u16::max_value() as f64;
//
//         let x = match rm {
//             Round => self.round(),
//             Ceil => self.ceil(),
//             Floor => self.floor(),
//             Trunc => self.trunc()
//         };
//
//         if x.is_nan() || x < 0.0 || x > max {
//             Err(ParseError::InvalidNumber(self.to_string()))
//         } else { Ok(x as u16) }
//     }
//
//     fn to_u16_rm(self, rm: RoundingMode) -> u16 {
//         match self.to_u16_rm_res(rm) {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//
// impl ToU32 for f64 {
//     fn to_u32_res(self) -> ParseResultU32 {
//         self.to_u32_rm_res(Trunc)
//     }
//
//     fn to_u32(self) -> u32 {
//         self.to_u32_rm(Trunc)
//     }
// }
//
// impl ToU32RM for f64 {
//     fn to_u32_rm_res(self, rm: RoundingMode) -> ParseResultU32 {
//         let max = u32::max_value() as f64;
//
//         let x = match rm {
//             Round => self.round(),
//             Ceil => self.ceil(),
//             Floor => self.floor(),
//             Trunc => self.trunc()
//         };
//
//         if x.is_nan() || x < 0.0 || x > max {
//             Err(ParseError::InvalidNumber(self.to_string()))
//         } else { Ok(x as u32) }
//     }
//
//     fn to_u32_rm(self, rm: RoundingMode) -> u32 {
//         match self.to_u32_rm_res(rm) {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//
// impl ToU64 for f64 {
//     fn to_u64_res(self) -> ParseResultU64 {
//         self.to_u64_rm_res(Trunc)
//     }
//
//     fn to_u64(self) -> u64 {
//         self.to_u64_rm(Trunc)
//     }
// }
//
// impl ToU64RM for f64 {
//     fn to_u64_rm_res(self, rm: RoundingMode) -> ParseResultU64 {
//         let max = 9007199254740991_f64;
//
//         let x = match rm {
//             Round => self.round(),
//             Ceil => self.ceil(),
//             Floor => self.floor(),
//             Trunc => self.trunc()
//         };
//
//         if x.is_nan() || x < 0.0 || x > max {
//             Err(ParseError::InvalidNumber(self.to_string()))
//         } else { Ok(x as u64) }
//     }
//
//     fn to_u64_rm(self, rm: RoundingMode) -> u64 {
//         match self.to_u64_rm_res(rm) {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//
// impl ToUsize for f64 {
//     fn to_usize_res(self) -> ParseResultUsize {
//         self.to_usize_rm_res(Trunc)
//     }
//
//     fn to_usize(self) -> usize {
//         self.to_usize_rm(Trunc)
//     }
// }
//
// impl ToUsizeRM for f64 {
//     fn to_usize_rm_res(self, rm: RoundingMode) -> ParseResultUsize {
//         let max = 9007199254740991_f64;
//
//         let x = match rm {
//             Round => self.round(),
//             Ceil => self.ceil(),
//             Floor => self.floor(),
//             Trunc => self.trunc()
//         };
//
//         if x.is_nan() || x < 0.0 || x > max {
//             Err(ParseError::InvalidNumber(self.to_string()))
//         } else { Ok(x as usize) }
//     }
//
//     fn to_usize_rm(self, rm: RoundingMode) -> usize {
//         match self.to_usize_rm_res(rm) {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
