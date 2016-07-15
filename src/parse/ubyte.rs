pub fn tobool(b:u8) -> bool {
    if b == 0 { false }
    else { true }
}

pub fn toi8(b:u8) -> i8 {
    let max = i8::max_value() as u8;

    if b > max { panic!("Invalid Number"); }
    else{ b as i8  }
}
