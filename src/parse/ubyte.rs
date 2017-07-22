// use parse::{
//     ToI8,
//     ToBool
// };
//
// use parse::{
//     ParseResultI8
// };
//
// use error::ParseError;
//

//
// impl ToI8 for u8 {
//     fn to_i8_res(self) -> ParseResultI8 {
//         let max = i8::max_value() as u8;
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
