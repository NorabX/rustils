use parse::{
    ToI8, ToI16,
    ToU8, ToU16, ToU32, ToU64,
    ToUsize,
    ToBool
};

use parse::{
    ParseResultI8, ParseResultI16,
    ParseResultU8, ParseResultU16, ParseResultU32, ParseResultU64,
    ParseResultUsize
};

use ParseError;

impl ToBool for i32 {
    fn to_bool(self) -> bool {
        if self == 0 { false } else { true }
    }
}

impl ToI8 for i32 {
    fn to_i8_res(self) -> ParseResultI8 {
        let min = i8::min_value() as i32;
        let max = i8::max_value() as i32;

        if self < min || self > max {
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

impl ToI16 for i32 {
    fn to_i16_res(self) -> ParseResultI16 {
        let min = i16::min_value() as i32;
        let max = i16::max_value() as i32;

        if self < min || self > max {
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

impl ToU8 for i32 {
    fn to_u8_res(self) -> ParseResultU8 {
        let max = u8::max_value() as i32;

        if self < 0 || self > max {
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

impl ToU16 for i32 {
    fn to_u16_res(self) -> ParseResultU16 {
        let max = u16::max_value() as i32;

        if self < 0 || self > max {
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

impl ToU32 for i32 {
    fn to_u32_res(self) -> ParseResultU32 {
        if self < 0 {
            Err(ParseError::InvalidNumber(self.to_string()))
        } else { Ok(self as u32) }
    }

    fn to_u32(self) -> u32 {
        match self.to_u32_res() {
            Ok(i) => i,
            Err(err) => panic!("{}",err)
        }
    }
}

impl ToU64 for i32 {
    fn to_u64_res(self) -> ParseResultU64 {
        if self < 0 {
            Err(ParseError::InvalidNumber(self.to_string()))
        } else { Ok(self as u64) }
    }

    fn to_u64(self) -> u64 {
        match self.to_u64_res() {
            Ok(i) => i,
            Err(err) => panic!("{}",err)
        }
    }
}

impl ToUsize for i32 {
    fn to_usize_res(self) -> ParseResultUsize {
        if self < 0 {
            Err(ParseError::InvalidNumber(self.to_string()))
        } else { Ok(self as usize) }
    }

    fn to_usize(self) -> usize {
        match self.to_usize_res() {
            Ok(i) => i,
            Err(err) => panic!("{}",err)
        }
    }
}
