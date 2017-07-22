use error::ParseError;
use RoundingMode;

// <editor-fold> # Traits

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
// </editor-fold>

// <editor-fold> # Functions

// <editor-fold> ## bool

pub fn bool_to_f32_res(a: bool)
    -> ParseResultF32 {

    if a { Ok(1.0) } else { Ok(0.0) }
}

pub fn bool_to_f32(a: bool)
    -> f32 {

    if a { 1.0 } else { 0.0 }
}
// </editor-fold>

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

// </editor-fold>

// <editor-fold> # Types
pub type ParseResultF32 = Result<f32,ParseError>;
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
// use RoundingMode::{Trunc,Ceil,Floor,Round};
//

//
// impl ToI8 for f32 {
//     fn to_i8_res(self) -> ParseResultI8{
//         self.to_i8_rm_res(Trunc)
//     }
//
//     fn to_i8(self) -> i8 {
//         self.to_i8_rm(Trunc)
//     }
// }
//
// impl ToI8RM for f32 {
//     fn to_i8_rm_res(self, rm: RoundingMode) -> ParseResultI8 {
//         let min = i8::min_value() as f32;
//         let max = i8::max_value() as f32;
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
// impl ToI32 for f32 {
//     fn to_i32_res(self) -> ParseResultI32 {
//         self.to_i32_rm_res(Trunc)
//     }
//
//     fn to_i32(self) -> i32 {
//         self.to_i32_rm(Trunc)
//     }
// }
//
// impl ToI32RM for f32 {
//     fn to_i32_rm_res(self, rm: RoundingMode) -> ParseResultI32 {
//         let min = -16777215_f32;
//         let max = 16777215_f32;
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
// impl ToIsize for f32 {
//     fn to_isize_res(self) -> ParseResultIsize {
//         self.to_isize_rm_res(Trunc)
//     }
//
//     fn to_isize(self) -> isize {
//         self.to_isize_rm(Trunc)
//     }
// }
//
// impl ToIsizeRM for f32 {
//     fn to_isize_rm_res(self, rm: RoundingMode) -> ParseResultIsize {
//         let min = -16777215_f32;
//         let max = 16777215_f32;
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
// impl ToU16 for f32 {
//     fn to_u16_res(self) -> ParseResultU16 {
//         self.to_u16_rm_res(Trunc)
//     }
//
//     fn to_u16(self) -> u16 {
//         self.to_u16_rm(Trunc)
//     }
// }
//
// impl ToU16RM for f32 {
//     fn to_u16_rm_res(self, rm: RoundingMode) -> ParseResultU16 {
//         let max = u16::max_value() as f32;
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
// impl ToU32 for f32 {
//     fn to_u32_res(self) -> ParseResultU32 {
//         self.to_u32_rm_res(Trunc)
//     }
//
//     fn to_u32(self) -> u32 {
//         self.to_u32_rm(Trunc)
//     }
// }
//
// impl ToU32RM for f32 {
//     fn to_u32_rm_res(self, rm: RoundingMode) -> ParseResultU32 {
//         let max = 16777215_f32;
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
// impl ToU64 for f32 {
//     fn to_u64_res(self) -> ParseResultU64 {
//         self.to_u64_rm_res(Trunc)
//     }
//
//     fn to_u64(self) -> u64 {
//         self.to_u64_rm(Trunc)
//     }
// }
//
// impl ToU64RM for f32 {
//     fn to_u64_rm_res(self, rm: RoundingMode) -> ParseResultU64 {
//         let max = 16777215_f32;
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
// impl ToUsize for f32 {
//     fn to_usize_res(self) -> ParseResultUsize {
//         self.to_usize_rm_res(Trunc)
//     }
//
//     fn to_usize(self) -> usize {
//         self.to_usize_rm(Trunc)
//     }
// }
//
// impl ToUsizeRM for f32 {
//     fn to_usize_rm_res(self, rm: RoundingMode) -> ParseResultUsize {
//         let max = 16777215_f32;
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
