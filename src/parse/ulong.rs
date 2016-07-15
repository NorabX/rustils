pub fn tobool(l:u64) -> bool {
    if l == 0 { false }
    else { true }
}

pub fn toi8(l:u64) -> i8 {
    let max = i8::max_value() as u64;

    if l > max { panic!("Invalid Number"); }
    else{ l as i8  }
}

pub fn toi16(l:u64) -> i16 {
    let max = i16::max_value() as u64;

    if l > max { panic!("Invalid Number"); }
    else{ l as i16  }
}

pub fn toi32(l:u64) -> i32 {
    let max = i32::max_value() as u64;

    if l > max { panic!("Invalid Number"); }
    else{ l as i32  }
}

pub fn toi64(l:u64) -> i64 {
    let max = i64::max_value() as u64;

    if l > max { panic!("Invalid Number"); }
    else{ l as i64  }
}

pub fn tou8(l:u64) -> u8 {
    let max = u8::max_value() as u64;

    if l > max { panic!("Invalid Number"); }
    else{ l as u8  }
}

pub fn tou16(l:u64) -> u16 {
    let max = u16::max_value() as u64;

    if l > max { panic!("Invalid Number"); }
    else{ l as u16  }
}

pub fn tou32(l:u64) -> u32 {
    let max = u32::max_value() as u64;

    if l > max { panic!("Invalid Number"); }
    else{ l as u32  }
}
