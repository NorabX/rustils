// <editor-fold> # Uses

use error::ParseError;
use RoundingMode;
// </editor-fold>

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

// <editor-fold> ## string

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

// </editor-fold>

// <editor-fold> # Types

pub type ParseResultF32 = Result<f32, ParseError>;
// </editor-fold>
