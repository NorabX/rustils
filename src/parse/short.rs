pub fn tobool(s:i16) -> bool {
    if s == 0 { false }
    else { true }
}

pub fn toi8(s:i16) -> i8 {
    let min = i8::min_value() as i16;
    let max = i8::max_value() as i16;

    if s < min || s > max { panic!("Invalid Number"); }
    else{ s as i8  }
}

pub fn tou8(s:i16) -> u8 {
    let max = u8::max_value() as i16;

    if s < 0 || s > max { panic!("Invalid Number"); }
    else{ s as u8  }
}

pub fn tou16(s:i16) -> u16 {
    if s < 0 { panic!("Invalid Number"); }
    else{ s as u16  }
}

pub fn tou32(s:i16) -> u32 {
    if s < 0 { panic!("Invalid Number"); }
    else{ s as u32  }
}

pub fn tou64(s:i16) -> u64 {
    if s < 0 { panic!("Invalid Number"); }
    else{ s as u64  }
}
