pub fn tobool(b:i8) -> bool {
    if b == 0 { false }
    else { true }
}

pub fn tou8(b:i8) -> u8 {
    if b < 0 { panic!("Invalid Number"); }
    else{ b as u8  }
}

pub fn tou16(b:i8) -> u16 {
    if b < 0 { panic!("Invalid Number"); }
    else{ b as u16  }
}

pub fn tou32(b:i8) -> u32 {
    if b < 0 { panic!("Invalid Number"); }
    else{ b as u32  }
}

pub fn tou64(b:i8) -> u64 {
    if b < 0 { panic!("Invalid Number"); }
    else{ b as u64  }
}
