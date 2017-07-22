// use parse::{
//     ToI8, ToI16, ToI32,
//     ToU8, ToU16,
//     ToBool
// };
//
// use parse::{
//     ParseResultI8, ParseResultI16, ParseResultI32,
//     ParseResultU8, ParseResultU16
// };
//
// use error::ParseError;
//

//
// impl ToI8 for u32 {
//     fn to_i8_res(self) -> ParseResultI8 {
//         let max = i8::max_value() as u32;
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
// impl ToI32 for u32 {
//     fn to_i32_res(self) -> ParseResultI32 {
//         let max = i32::max_value() as u32;
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
// impl ToU16 for u32 {
//     fn to_u16_res(self) -> ParseResultU16 {
//         let max = u16::max_value() as u32;
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
