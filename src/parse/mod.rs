pub mod boolean;
pub mod string;
pub mod double;
pub mod float;
pub mod ulong;
pub mod uint;
pub mod ushort;
pub mod ubyte;
pub mod long;
pub mod int;
pub mod short;
pub mod byte;

use { RoundingMode, ParseError };

pub type ParseResultI8 = Result<i8,ParseError>;
pub type ParseResultI16 = Result<i16,ParseError>;
pub type ParseResultI32 = Result<i32,ParseError>;
pub type ParseResultI64 = Result<i64,ParseError>;

pub type ParseResultU8 = Result<u8,ParseError>;
pub type ParseResultU16 = Result<u16,ParseError>;
pub type ParseResultU32 = Result<u32,ParseError>;
pub type ParseResultU64 = Result<u64,ParseError>;

pub type ParseResultF32 = Result<f32,ParseError>;
pub type ParseResultF64 = Result<f64,ParseError>;

pub type ParseResultIsize = Result<isize,ParseError>;
pub type ParseResultUsize = Result<usize,ParseError>;

pub trait ToStr{ fn to_str(self) -> &'static str; }

pub trait ToBool { fn to_bool(self) -> bool; }

pub trait ToI8 {
    fn to_i8_res(self) -> ParseResultI8;
    fn to_i8(self) -> i8;
}

pub trait ToI8RM {
    fn to_i8_rm_res(self, rm: RoundingMode) -> ParseResultI8;
    fn to_i8_rm(self, rm: RoundingMode) -> i8;
}

pub trait ToI16 {
    fn to_i16_res(self) -> ParseResultI16;
    fn to_i16(self) -> i16;
}

pub trait ToI16RM {
    fn to_i16_rm_res(self, rm: RoundingMode) -> ParseResultI16;
    fn to_i16_rm(self, rm: RoundingMode) -> i16;
}

pub trait ToI32 {
    fn to_i32_res(self) -> ParseResultI32;
    fn to_i32(self) -> i32;
}

pub trait ToI32RM {
    fn to_i32_rm_res(self, rm: RoundingMode) -> ParseResultI32;
    fn to_i32_rm(self, rm: RoundingMode) -> i32;
}

pub trait ToI64 {
    fn to_i64_res(self) -> ParseResultI64;
    fn to_i64(self) -> i64;
}

pub trait ToI64RM {
    fn to_i64_rm_res(self, rm: RoundingMode) -> ParseResultI64;
    fn to_i64_rm(self, rm: RoundingMode) -> i64;
}

pub trait ToU8 {
    fn to_u8_res(self) -> ParseResultU8;
    fn to_u8(self) -> u8;
}

pub trait ToU8RM {
    fn to_u8_rm_res(self, rm: RoundingMode) -> ParseResultU8;
    fn to_u8_rm(self, rm: RoundingMode) -> u8;
}

pub trait ToU16 {
    fn to_u16_res(self) -> ParseResultU16;
    fn to_u16(self) -> u16;
}

pub trait ToU16RM {
    fn to_u16_rm_res(self, rm: RoundingMode) -> ParseResultU16;
    fn to_u16_rm(self, rm: RoundingMode) -> u16;
}

pub trait ToU32 {
    fn to_u32_res(self) -> ParseResultU32;
    fn to_u32(self) -> u32;
}

pub trait ToU32RM {
    fn to_u32_rm_res(self, rm: RoundingMode) -> ParseResultU32;
    fn to_u32_rm(self, rm: RoundingMode) -> u32;
}

pub trait ToU64 {
    fn to_u64_res(self) -> ParseResultU64;
    fn to_u64(self) -> u64;
}

pub trait ToU64RM {
    fn to_u64_rm_res(self, rm: RoundingMode) -> ParseResultU64;
    fn to_u64_rm(self, rm: RoundingMode) -> u64;
}

pub trait ToF32 {
    fn to_f32_res(self) -> ParseResultF32;
    fn to_f32(self) -> f32;
}

pub trait ToF32RM {
    fn to_f32_rm_res(self, rm: RoundingMode) -> ParseResultF32;
    fn to_f32_rm(self, rm: RoundingMode) -> f32;
}

pub trait ToF64 {
    fn to_f64_res(self) -> ParseResultF64;
    fn to_f64(self) -> f64;
}

pub trait ToF64RM {
    fn to_f64_rm_res(self, rm: RoundingMode) -> ParseResultF64;
    fn to_f64_rm(self, rm: RoundingMode) -> f64;
}

pub trait ToIsize {
    fn to_isize_res(self) -> ParseResultIsize;
    fn to_isize(self) -> isize;
}

pub trait ToIsizeRM {
    fn to_isize_rm_res(self, rm: RoundingMode) -> ParseResultIsize;
    fn to_isize_rm(self, rm: RoundingMode) -> isize;
}

pub trait ToUsize {
    fn to_usize_res(self) -> ParseResultUsize;
    fn to_usize(self) -> usize;
}

pub trait ToUsizeRM {
    fn to_usize_rm_res(self, rm: RoundingMode) -> ParseResultUsize;
    fn to_usize_rm(self, rm: RoundingMode) -> usize;
}
