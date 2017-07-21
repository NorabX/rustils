use parse::{
    ToI8, ToI16, ToI32,
    ToU8, ToU16,
    ToBool
};

use parse::{
    ParseResultI8, ParseResultI16, ParseResultI32,
    ParseResultU8, ParseResultU16
};

use error::ParseError;

impl ToBool for u32 {
    fn to_bool(self) -> bool {
        if self == 0 { false } else { true }
    }
}

impl ToI8 for u32 {
    fn to_i8_res(self) -> ParseResultI8 {
        let max = i8::max_value() as u32;

        if self > max {
            Err(ParseError::InvalidNumber(self.to_string()))
        } else { Ok(self as i8) }
    }

    fn to_i8(self) -> i8 {
        match self.to_i8_res() {
            Ok(i) => i,
            Err(err) => panic!("{}",err)
        }
    }
}

impl ToI16 for u32 {
    fn to_i16_res(self) -> ParseResultI16 {
        let max = i16::max_value() as u32;

        if self > max {
            Err(ParseError::InvalidNumber(self.to_string()))
        } else { Ok(self as i16) }
    }

    fn to_i16(self) -> i16 {
        match self.to_i16_res() {
            Ok(i) => i,
            Err(err) => panic!("{}",err)
        }
    }
}

impl ToI32 for u32 {
    fn to_i32_res(self) -> ParseResultI32 {
        let max = i32::max_value() as u32;

        if self > max {
            Err(ParseError::InvalidNumber(self.to_string()))
        } else { Ok(self as i32) }
    }

    fn to_i32(self) -> i32 {
        match self.to_i32_res() {
            Ok(i) => i,
            Err(err) => panic!("{}",err)
        }
    }
}

impl ToU8 for u32 {
    fn to_u8_res(self) -> ParseResultU8 {
        let max = u8::max_value() as u32;

        if self > max {
            Err(ParseError::InvalidNumber(self.to_string()))
        } else { Ok(self as u8) }
    }

    fn to_u8(self) -> u8 {
        match self.to_u8_res() {
            Ok(i) => i,
            Err(err) => panic!("{}",err)
        }
    }
}

impl ToU16 for u32 {
    fn to_u16_res(self) -> ParseResultU16 {
        let max = u16::max_value() as u32;

        if self > max {
            Err(ParseError::InvalidNumber(self.to_string()))
        } else { Ok(self as u16) }
    }

    fn to_u16(self) -> u16 {
        match self.to_u16_res() {
            Ok(i) => i,
            Err(err) => panic!("{}",err)
        }
    }
}
