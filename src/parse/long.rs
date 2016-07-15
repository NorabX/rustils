pub fn tobool(l:i64) -> bool {
    if l == 0 { false }
    else { true }
}

pub fn toi8(l:i64) -> i8 {
    let min = i8::min_value() as i64;
    let max = i8::max_value() as i64;

    if l < min || l > max { panic!("Invalid Number"); }
    else{ l as i8  }
}

pub fn toi16(l:i64) -> i16 {
    let min = i16::min_value() as i64;
    let max = i16::max_value() as i64;

    if l < min || l > max { panic!("Invalid Number"); }
    else{ l as i16  }
}

pub fn toi32(l:i64) -> i32 {
    let min = i32::min_value() as i64;
    let max = i32::max_value() as i64;

    if l < min || l > max { panic!("Invalid Number"); }
    else{ l as i32  }
}

pub fn tou8(l:i64) -> u8 {
    let max = u8::max_value() as i64;

    if l < 0 || l > max { panic!("Invalid Number"); }
    else{ l as u8  }
}

pub fn tou16(l:i64) -> u16 {
    let max = u16::max_value() as i64;

    if l < 0 || l > max { panic!("Invalid Number"); }
    else{ l as u16  }
}

pub fn tou32(l:i64) -> u32 {
    let max = u32::max_value() as i64;

    if l < 0 || l > max { panic!("Invalid Number"); }
    else{ l as u32  }
}

pub fn tou64(l:i64) -> u64 {
    if l < 0 { panic!("Invalid Number"); }
    else{ l as u64  }
}
