use parse::{
    ToI8, ToI16, ToI32, ToI64,
    ToU8, ToU16, ToU32,
    ToIsize, ToBool
};

use parse::{
    ParseResultI8, ParseResultI16, ParseResultI32, ParseResultI64,
    ParseResultU8, ParseResultU16, ParseResultU32,
    ParseResultIsize
};

use ParseError;

impl ToBool for usize {
    fn to_bool(self) -> bool {
        if self == 0 { false } else { true }
    }
}

impl ToI8 for usize {
    fn to_i8_res(self) -> ParseResultI8 {
        let max = i8::max_value() as usize;

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

impl ToI16 for usize {
    fn to_i16_res(self) -> ParseResultI16 {
        let max = i16::max_value() as usize;

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

impl ToI32 for usize {
    fn to_i32_res(self) -> ParseResultI32 {
        let max = i32::max_value() as usize;

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

impl ToI64 for usize {
    fn to_i64_res(self) -> ParseResultI64 {
        let max = i64::max_value() as usize;

        if self > max {
            Err(ParseError::InvalidNumber(self.to_string()))
        } else { Ok(self as i64) }
    }

    fn to_i64(self) -> i64 {
        match self.to_i64_res() {
            Ok(i) => i,
            Err(err) => panic!("{}",err)
        }
    }
}

impl ToIsize for usize {
    fn to_isize_res(self) -> ParseResultIsize {
        let max = isize::max_value() as usize;

        if self > max {
            Err(ParseError::InvalidNumber(self.to_string()))
        } else { Ok(self as isize) }
    }

    fn to_isize(self) -> isize {
        match self.to_isize_res() {
            Ok(i) => i,
            Err(err) => panic!("{}",err)
        }
    }
}

impl ToU8 for usize {
    fn to_u8_res(self) -> ParseResultU8 {
        let max = u8::max_value() as usize;

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

impl ToU16 for usize {
    fn to_u16_res(self) -> ParseResultU16 {
        let max = u16::max_value() as usize;

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

impl ToU32 for usize {
    fn to_u32_res(self) -> ParseResultU32 {
        let max = u32::max_value() as usize;

        if self > max {
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
