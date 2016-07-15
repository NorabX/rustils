pub fn tobool(d:f64) -> bool {
    if d.is_nan() || d == 0.0 { false }
    else { true }
}

pub fn toi8(d:f64) -> i8 {
    let min = i8::min_value() as f64;
    let max = i8::max_value() as f64;

    if d.is_nan() || d < min || d > max { panic!("Invalid Number"); }
    else{ d as i8  }
}

pub fn toi16(d:f64) -> i16 {
    let min = i16::min_value() as f64;
    let max = i16::max_value() as f64;

    if d.is_nan() || d < min || d > max { panic!("Invalid Number"); }
    else{ d as i16  }
}

pub fn toi32(d:f64) -> i32 {
    let min = i32::min_value() as f64;
    let max = i32::max_value() as f64;

    if d.is_nan() || d < min || d > max { panic!("Invalid Number"); }
    else{ d as i32  }
}

pub fn toi64(d:f64) -> i64 {
    let min = i64::min_value() as f64;
    let max = i64::max_value() as f64;

    if d.is_nan() || d < min || d > max { panic!("Invalid Number"); }
    else{ d as i64  }
}

pub fn tou8(d:f64) -> u8 {
    let max = u8::max_value() as f64;

    if d.is_nan() || d < 0.0 || d > max { panic!("Invalid Number"); }
    else{ d as u8  }
}

pub fn tou16(d:f64) -> u16 {
    let max = u16::max_value() as f64;

    if d.is_nan() || d < 0.0 || d > max { panic!("Invalid Number"); }
    else{ d as u16  }
}

pub fn tou32(d:f64) -> u32 {
    let max = u32::max_value() as f64;

    if d.is_nan() || d < 0.0 || d > max { panic!("Invalid Number"); }
    else{ d as u32  }
}

/*
pub fn tou64(d:f64) -> u64 {
    let max = u64::max_value() as f64;

    if d.is_nan() || d < 0.0 || d > max { panic!("Invalid Number"); }
    else{ d as u64  }
}
*/

//pub fn tof32(d:f64) -> f32 {
// **NO MIN MAX VALUE**
//}
