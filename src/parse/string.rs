pub fn to_i8p(s:&str) -> i8 {
    match s.parse::<i8>() {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}

pub fn to_i16p(s:&str) -> i16 {
    match s.parse::<i16>() {
      Ok(n) => n,
      Err(err) => panic!("{}",err)
    }
}

pub fn to_i32p(s:&str) -> i32 {
    match s.parse::<i32>() {
      Ok(n) => n,
      Err(err) => panic!("{}",err)
    }
}

pub fn to_i64p(s:&str) -> i64 {
    match s.parse::<i64>() {
      Ok(n) => n,
      Err(err) => panic!("{}",err)
    }
}

pub fn to_u8p(s:&str) -> u8 {
    match s.parse::<u8>() {
      Ok(n) => n,
      Err(err) => panic!("{}",err)
    }
}

pub fn to_u16p(s:&str) -> u16 {
    match s.parse::<u16>() {
      Ok(n) => n,
      Err(err) => panic!("{}",err)
    }
}

pub fn to_u32p(s:&str) -> u32 {
    match s.parse::<u32>() {
      Ok(n) => n,
      Err(err) => panic!("{}",err)
    }
}

pub fn to_u64p(s:&str) -> u64 {
    match s.parse::<u64>() {
      Ok(n) => n,
      Err(err) => panic!("{}",err)
    }
}

pub fn to_f32p(s:&str) -> f32 {
    match s.parse::<f32>() {
      Ok(n) => n,
      Err(err) => panic!("{}",err)
    }
}

pub fn to_f64p(s:&str) -> f64 {
    match s.parse::<f64>() {
      Ok(n) => n,
      Err(err) => panic!("{}",err)
    }
}
