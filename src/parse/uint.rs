pub fn tobool(i:u32) -> bool {
    if i == 0 { false }
    else { true }
}

pub fn toi8(i:u32) -> i8 {
    let max = i8::max_value() as u32;

    if i > max { panic!("Invalid Number"); }
    else{ i as i8  }
}

pub fn toi16(i:u32) -> i16 {
    let max = i16::max_value() as u32;

    if i > max { panic!("Invalid Number"); }
    else{ i as i16  }
}

pub fn toi32(i:u32) -> i32 {
    let max = i32::max_value() as u32;

    if i > max { panic!("Invalid Number"); }
    else{ i as i32  }
}

pub fn tou8(i:u32) -> u8 {
    let max = u8::max_value() as u32;

    if i > max { panic!("Invalid Number"); }
    else{ i as u8  }
}

pub fn tou16(i:u32) -> u16 {
    let max = u16::max_value() as u32;

    if i > max { panic!("Invalid Number"); }
    else{ i as u16  }
}
