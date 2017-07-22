
// <editor-fold> # Traits

pub trait ToStr {

    fn to_str(self)
        -> &'static str;
}
// </editor-fold>



// use parse::{
//     ToI8, ToI16, ToI32, ToI64,
//     ToU8, ToU16, ToU32, ToU64,
//     ToIsize, ToUsize, ToF32, ToF64,
//     ToBool, ToStr
// };
//
// use parse::{
//     ParseResultI8, ParseResultI16, ParseResultI32, ParseResultI64,
//     ParseResultU8, ParseResultU16, ParseResultU32, ParseResultU64,
//     ParseResultIsize, ParseResultUsize, ParseResultF32, ParseResultF64,
// };
//
// use error::ParseError;
// use std::mem::{forget,transmute};
//
// impl ToStr for String {
//     fn to_str(self) -> &'static str {
//         unsafe {
//             let ret = transmute(&self as &str);
//             forget(self);
//             ret
//         }
//     }
// }
//

//

//
// impl ToI8 for String {
//     fn to_i8_res(self) -> ParseResultI8 {
//         match self.parse::<i8>() {
//             Ok(n) => Ok(n),
//             Err(_) => Err(ParseError::InvalidNumber(self))
//         }
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
// impl ToI8 for &'static str {
//     fn to_i8_res(self) -> ParseResultI8 {
//         match self.parse::<i8>() {
//             Ok(n) => Ok(n),
//             Err(_) => Err(ParseError::InvalidNumber(self.to_string()))
//         }
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
// impl ToI32 for String {
//     fn to_i32_res(self) -> ParseResultI32 {
//         match self.parse::<i32>() {
//             Ok(n) => Ok(n),
//             Err(_) => Err(ParseError::InvalidNumber(self))
//         }
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
// impl ToI32 for &'static str {
//     fn to_i32_res(self) -> ParseResultI32 {
//         match self.parse::<i32>() {
//             Ok(n) => Ok(n),
//             Err(_) => Err(ParseError::InvalidNumber(self.to_string()))
//         }
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
// impl ToIsize for String {
//     fn to_isize_res(self) -> ParseResultIsize {
//         match self.parse::<isize>() {
//             Ok(n) => Ok(n),
//             Err(_) => Err(ParseError::InvalidNumber(self))
//         }
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
// impl ToIsize for &'static str {
//     fn to_isize_res(self) -> ParseResultIsize {
//         match self.parse::<isize>() {
//             Ok(n) => Ok(n),
//             Err(_) => Err(ParseError::InvalidNumber(self.to_string()))
//         }
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
// impl ToU16 for String {
//     fn to_u16_res(self) -> ParseResultU16 {
//         match self.parse::<u16>() {
//             Ok(n) => Ok(n),
//             Err(_) => Err(ParseError::InvalidNumber(self))
//         }
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
// impl ToU16 for &'static str {
//     fn to_u16_res(self) -> ParseResultU16 {
//         match self.parse::<u16>() {
//             Ok(n) => Ok(n),
//             Err(_) => Err(ParseError::InvalidNumber(self.to_string()))
//         }
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
// impl ToU64 for String {
//     fn to_u64_res(self) -> ParseResultU64 {
//         match self.parse::<u64>() {
//             Ok(n) => Ok(n),
//             Err(_) => Err(ParseError::InvalidNumber(self))
//         }
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
// impl ToU64 for &'static str {
//     fn to_u64_res(self) -> ParseResultU64 {
//         match self.parse::<u64>() {
//             Ok(n) => Ok(n),
//             Err(_) => Err(ParseError::InvalidNumber(self.to_string()))
//         }
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
// impl ToUsize for String {
//     fn to_usize_res(self) -> ParseResultUsize {
//         match self.parse::<usize>() {
//             Ok(n) => Ok(n),
//             Err(_) => Err(ParseError::InvalidNumber(self))
//         }
//     }
//
//     fn to_usize(self) -> usize {
//         match self.to_usize_res() {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//
// impl ToUsize for &'static str {
//     fn to_usize_res(self) -> ParseResultUsize {
//         match self.parse::<usize>() {
//             Ok(n) => Ok(n),
//             Err(_) => Err(ParseError::InvalidNumber(self.to_string()))
//         }
//     }
//
//     fn to_usize(self) -> usize {
//         match self.to_usize_res() {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//
// impl ToF32 for String {
//     fn to_f32_res(self) -> ParseResultF32 {
//         match self.parse::<f32>() {
//             Ok(n) => Ok(n),
//             Err(_) => Err(ParseError::InvalidNumber(self))
//         }
//     }
//
//     fn to_f32(self) -> f32 {
//         match self.to_f32_res() {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//
// impl ToF32 for &'static str {
//     fn to_f32_res(self) -> ParseResultF32 {
//         match self.parse::<f32>() {
//             Ok(n) => Ok(n),
//             Err(_) => Err(ParseError::InvalidNumber(self.to_string()))
//         }
//     }
//
//     fn to_f32(self) -> f32 {
//         match self.to_f32_res() {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//
// impl ToF64 for String {
//     fn to_f64_res(self) -> ParseResultF64 {
//         match self.parse::<f64>() {
//             Ok(n) => Ok(n),
//             Err(_) => Err(ParseError::InvalidNumber(self))
//         }
//     }
//
//     fn to_f64(self) -> f64 {
//         match self.to_f64_res() {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//
// impl ToF64 for &'static str {
//     fn to_f64_res(self) -> ParseResultF64 {
//         match self.parse::<f64>() {
//             Ok(n) => Ok(n),
//             Err(_) => Err(ParseError::InvalidNumber(self.to_string()))
//         }
//     }
//
//     fn to_f64(self) -> f64 {
//         match self.to_f64_res() {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
