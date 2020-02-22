use error::ParseError;
use RoundingMode;

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

pub fn bool_to_f32_res(a: bool)
    -> ParseResultF32 {

    if a { Ok(1.0) } else { Ok(0.0) }
}

pub fn bool_to_f32(a: bool)
    -> f32 {

    if a { 1.0 } else { 0.0 }
}

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

pub type ParseResultF32 = Result<f32, ParseError>;
