use parse::{
    ToI8, ToI16, ToI32, ToI64, ToISize,
    ToU8, ToU16, ToU32, ToU64, ToUSize,
    ToF32, ToF64
};

impl ToI8 for bool{
    fn to_i8(self) -> i8{
        if self { 1 } else { 0 }
    }
}

impl ToI16 for bool{
    fn to_i16(self) -> i16{
        if self { 1 } else { 0 }
    }
}

impl ToI32 for bool{
    fn to_i32(self) -> i32{
        if self { 1 } else { 0 }
    }
}

impl ToI64 for bool{
    fn to_i64(self) -> i64{
        if self { 1 } else { 0 }
    }
}

impl ToISize for bool{
    fn to_isize(self) -> isize{
        if self { 1 } else { 0 }
    }
}

impl ToU8 for bool{
    fn to_u8(self) -> u8{
        if self { 1 } else { 0 }
    }
}

impl ToU16 for bool{
    fn to_u16(self) -> u16{
        if self { 1 } else { 0 }
    }
}

impl ToU32 for bool{
    fn to_u32(self) -> u32{
        if self { 1 } else { 0 }
    }
}

impl ToU64 for bool{
    fn to_u64(self) -> u64{
        if self { 1 } else { 0 }
    }
}

impl ToUSize for bool{
    fn to_usize(self) -> usize{
        if self { 1 } else { 0 }
    }
}

impl ToF32 for bool{
    fn to_f32(self) -> f32{
        if self { 1.0 } else { 0.0 }
    }
}

impl ToF64 for bool{
    fn to_f64(self) -> f64{
        if self { 1.0 } else { 0.0 }
    }
}
