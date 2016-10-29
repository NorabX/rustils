use parse::{
    ToI8, ToI16, ToI32, ToI64,
    ToU8, ToU16, ToU32, ToU64,
    ToF32, ToF64, ToIsize, ToUsize
};
use parse::{
    ParseResultI8, ParseResultI16, ParseResultI32, ParseResultI64,
    ParseResultU8, ParseResultU16, ParseResultU32, ParseResultU64,
    ParseResultF32, ParseResultF64, ParseResultIsize, ParseResultUsize
};

impl ToI8 for bool {
    fn to_i8_res(self) -> ParseResultI8 {
        if self { Ok(1) } else { Ok(0) }
    }

    fn to_i8(self) -> i8 {
        if self { 1 } else { 0 }
    }
}

impl ToI16 for bool {
    fn to_i16_res(self) -> ParseResultI16 {
        if self { Ok(1) } else { Ok(0) }
    }

    fn to_i16(self) -> i16 {
        if self { 1 } else { 0 }
    }
}

impl ToI32 for bool {
    fn to_i32_res(self) -> ParseResultI32 {
        if self { Ok(1) } else { Ok(0) }
    }

    fn to_i32(self) -> i32 {
        if self { 1 } else { 0 }
    }
}

impl ToI64 for bool {
    fn to_i64_res(self) -> ParseResultI64 {
        if self { Ok(1) } else { Ok(0) }
    }

    fn to_i64(self) -> i64 {
        if self { 1 } else { 0 }
    }
}

impl ToU8 for bool {
    fn to_u8_res(self) -> ParseResultU8 {
        if self { Ok(1) } else { Ok(0) }
    }

    fn to_u8(self) -> u8 {
        if self { 1 } else { 0 }
    }
}

impl ToU16 for bool {
    fn to_u16_res(self) -> ParseResultU16 {
        if self { Ok(1) } else { Ok(0) }
    }

    fn to_u16(self) -> u16 {
        if self { 1 } else { 0 }
    }
}

impl ToU32 for bool {
    fn to_u32_res(self) -> ParseResultU32 {
        if self { Ok(1) } else { Ok(0) }
    }

    fn to_u32(self) -> u32 {
        if self { 1 } else { 0 }
    }
}

impl ToU64 for bool {
    fn to_u64_res(self) -> ParseResultU64 {
        if self { Ok(1) } else { Ok(0) }
    }

    fn to_u64(self) -> u64 {
        if self { 1 } else { 0 }
    }
}

impl ToF32 for bool {
    fn to_f32_res(self) -> ParseResultF32 {
        if self { Ok(1.0) } else { Ok(0.0) }
    }

    fn to_f32(self) -> f32 {
        if self { 1.0 } else { 0.0 }
    }
}

impl ToF64 for bool {
    fn to_f64_res(self) -> ParseResultF64 {
        if self { Ok(1.0) } else { Ok(0.0) }
    }

    fn to_f64(self) -> f64 {
        if self { 1.0 } else { 0.0 }
    }
}

impl ToIsize for bool {
    fn to_isize_res(self) -> ParseResultIsize {
        if self { Ok(1) } else { Ok(0) }
    }

    fn to_isize(self) -> isize {
        if self { 1 } else { 0 }
    }
}

impl ToUsize for bool {
    fn to_usize_res(self) -> ParseResultUsize {
        if self { Ok(1) } else { Ok(0) }
    }

    fn to_usize(self) -> usize {
        if self { 1 } else { 0 }
    }
}
