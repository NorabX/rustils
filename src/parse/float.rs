pub fn tobool(f:f32) -> bool {
    if f.is_nan() || f == 0.0 { false }
    else { true }
}

pub fn toi8(f:f32) -> i8 {
    let min = i8::min_value() as f32;
    let max = i8::max_value() as f32;

    if f.is_nan() || f < min || f > max { panic!("Invalid Number"); }
    else{ f as i8  }
}

pub fn toi16(f:f32) -> i16 {
    let min = i16::min_value() as f32;
    let max = i16::max_value() as f32;

    if f.is_nan() || f < min || f > max { panic!("Invalid Number"); }
    else{ f as i16  }
}

pub fn toi32(f:f32) -> i32 {
    let min = i32::min_value() as f32;
    let max = i32::max_value() as f32;

    if f.is_nan() || f < min || f > max { panic!("Invalid Number"); }
    else{ f as i32  }
}

pub fn toi64(f:f32) -> i64 {
    let min = i64::min_value() as f32;
    let max = i64::max_value() as f32;

    if f.is_nan() || f < min || f > max { panic!("Invalid Number"); }
    else{ f as i64  }
}

pub fn tou8(f:f32) -> u8 {
    let max = u8::max_value() as f32;

    if f.is_nan() || f < 0.0 || f > max { panic!("Invalid Number"); }
    else{ f as u8  }
}

pub fn tou16(f:f32) -> u16 {
    let max = u16::max_value() as f32;

    if f.is_nan() || f < 0.0 || f > max { panic!("Invalid Number"); }
    else{ f as u16  }
}

/*
pub fn tou32(f:f32) -> u32 {
    let max = u32::max_value() as f32;

    if f.is_nan() || f < 0.0 || f > max { panic!("Invalid Number"); }
    else{ f as u32  }
}
*/

/*
pub fn tou64(f:f32) -> u64 {
    let max = u64::max_value() as f32;

    if f.is_nan() || f < 0.0 || f > max { panic!("Invalid Number"); }
    else{ f as u64  }
}
*/
