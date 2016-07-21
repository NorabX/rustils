use parse::error::ParseError;

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
