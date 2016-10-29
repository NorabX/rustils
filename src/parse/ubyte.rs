
/*
pub fn to_bool(b:u8) -> bool {
    if b == 0 { false }
    else { true }
}

pub fn toi8(b:u8) -> ParseResultI8 {
    let max = i8::max_value() as u8;

    if b > max {
        Err(ParseError::InvalidNumber(b.to_string()))
    } else { Ok(b as i8) }
}

pub fn toi8p(b:u8) -> i8 {
    match toi8(b) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}*/
