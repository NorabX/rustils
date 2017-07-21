use parse::{
    ToI8, ToI16,
    ToU8,
    ToBool
};

use parse::{
    ParseResultI8, ParseResultI16,
    ParseResultU8
};

use error::ParseError;

impl ToBool for u16 {
    fn to_bool(self) -> bool {
        if self == 0 { false } else { true }
    }
}

impl ToI8 for u16 {
    fn to_i8_res(self) -> ParseResultI8 {
        let max = i8::max_value() as u16;

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

impl ToI16 for u16 {
    fn to_i16_res(self) -> ParseResultI16 {
        let max = i16::max_value() as u16;

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

impl ToU8 for u16 {
    fn to_u8_res(self) -> ParseResultU8 {
        let max = u8::max_value() as u16;

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
