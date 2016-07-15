pub fn tobool(s:u16) -> bool {
    if s == 0 { false }
    else { true }
}

pub fn toi8(s:u16) -> i8 {
    let max = i8::max_value() as u16;

    if s > max { panic!("Invalid Number"); }
    else{ s as i8  }
}

pub fn toi16(s:u16) -> i16 {
    let max = i16::max_value() as u16;

    if s > max { panic!("Invalid Number"); }
    else{ s as i16  }
}

pub fn tou8(s:u16) -> u8 {
    let max = u8::max_value() as u16;

    if s > max { panic!("Invalid Number"); }
    else{ s as u8  }
}
