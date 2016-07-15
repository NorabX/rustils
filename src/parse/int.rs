pub fn tobool(i:i32) -> bool {
    if i == 0 { false }
    else { true }
}

pub fn toi8(i:i32) -> i8 {
    let min = i8::min_value() as i32;
    let max = i8::max_value() as i32;

    if i < min || i > max { panic!("Invalid Number"); }
    else{ i as i8  }
}

pub fn toi16(i:i32) -> i16 {
    let min = i16::min_value() as i32;
    let max = i16::max_value() as i32;

    if i < min || i > max { panic!("Invalid Number"); }
    else{ i as i16  }
}

pub fn tou8(i:i32) -> u8 {
    let max = u8::max_value() as i32;

    if i < 0 || i > max { panic!("Invalid Number"); }
    else{ i as u8  }
}

pub fn tou16(i:i32) -> u16 {
    let max = u16::max_value() as i32;

    if i < 0 || i > max { panic!("Invalid Number"); }
    else{ i as u16  }
}

pub fn tou32(i:i32) -> u32 {
    if i < 0 { panic!("Invalid Number"); }
    else{ i as u32  }
}

pub fn tou64(i:i32) -> u64 {
    if i < 0 { panic!("Invalid Number"); }
    else{ i as u64  }
}
