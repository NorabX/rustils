pub fn toi8(s:&str) -> i8 {
    match s.parse::<i8>() {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}

pub fn toi16(s:&str) -> i16 {
    match s.parse::<i16>() {
      Ok(n) => n,
      Err(err) => panic!("{}",err)
    }
}

pub fn toi32(s:&str) -> i32 {
    match s.parse::<i32>() {
      Ok(n) => n,
      Err(err) => panic!("{}",err)
    }
}

pub fn toi64(s:&str) -> i64 {
    match s.parse::<i64>() {
      Ok(n) => n,
      Err(err) => panic!("{}",err)
    }
}

pub fn tou8(s:&str) -> u8 {
    match s.parse::<u8>() {
      Ok(n) => n,
      Err(err) => panic!("{}",err)
    }
}

pub fn tou16(s:&str) -> u16 {
    match s.parse::<u16>() {
      Ok(n) => n,
      Err(err) => panic!("{}",err)
    }
}

pub fn tou32(s:&str) -> u32 {
    match s.parse::<u32>() {
      Ok(n) => n,
      Err(err) => panic!("{}",err)
    }
}

pub fn tou64(s:&str) -> u64 {
    match s.parse::<u64>() {
      Ok(n) => n,
      Err(err) => panic!("{}",err)
    }
}

pub fn tof32(s:&str) -> f32 {
    match s.parse::<f32>() {
      Ok(n) => n,
      Err(err) => panic!("{}",err)
    }
}

pub fn tof64(s:&str) -> f64 {
    match s.parse::<f64>() {
      Ok(n) => n,
      Err(err) => panic!("{}",err)
    }
}
