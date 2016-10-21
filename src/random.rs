use rand;
use rand::distributions::{IndependentSample, Range};

/*
pub fn rand_range_vec<T: SampleRange + PartialOrd>(count: usize, min: T, max: T) -> Vec<T>{
    let mut x = Vec::<T>::with_capacity(count);
    for _ in 0..count { x.push(rand_range(min,max)); }
    x
}
*/
//#################################################################################################
pub fn rand_bool() -> bool { rand::random::<bool>() }
pub fn rand_char() -> char { rand::random::<char>() }
pub fn rand_i8() -> i8 { rand::random::<i8>() }
pub fn rand_i16() -> i16 { rand::random::<i16>() }
pub fn rand_i32() -> i32 { rand::random::<i32>() }
pub fn rand_i64() -> i64 { rand::random::<i64>() }
pub fn rand_u8() -> u8 { rand::random::<u8>() }
pub fn rand_u16() -> u16 { rand::random::<u16>() }
pub fn rand_u32() -> u32 { rand::random::<u32>() }
pub fn rand_u64() -> u64 { rand::random::<u64>() }
pub fn rand_f32() -> f32 { rand::random::<f32>() }
pub fn rand_f64() -> f64 { rand::random::<f64>() }
pub fn rand_usize() -> usize { rand::random::<usize>() }
pub fn rand_isize() -> isize { rand::random::<isize>() }

//#################################################################################################
pub fn rand_bool_vec(count: usize) -> Vec<bool>{
    let mut x = Vec::<bool>::with_capacity(count);
    for _ in 0..count { x.push(rand_bool()); }
    x
}

pub fn rand_char_vec(count: usize) -> Vec<char>{
    let mut x = Vec::<char>::with_capacity(count);
    for _ in 0..count { x.push(rand_char()); }
    x
}

pub fn rand_i8_vec(count: usize) -> Vec<i8>{
    let mut x = Vec::<i8>::with_capacity(count);
    for _ in 0..count { x.push(rand_i8()); }
    x
}

pub fn rand_i16_vec(count: usize) -> Vec<i16>{
    let mut x = Vec::<i16>::with_capacity(count);
    for _ in 0..count { x.push(rand_i16()); }
    x
}

pub fn rand_i32_vec(count: usize) -> Vec<i32>{
    let mut x = Vec::<i32>::with_capacity(count);
    for _ in 0..count { x.push(rand_i32()); }
    x
}

pub fn rand_i64_vec(count: usize) -> Vec<i64>{
    let mut x = Vec::<i64>::with_capacity(count);
    for _ in 0..count { x.push(rand_i64()); }
    x
}

pub fn rand_u8_vec(count: usize) -> Vec<u8>{
    let mut x = Vec::<u8>::with_capacity(count);
    for _ in 0..count { x.push(rand_u8()); }
    x
}

pub fn rand_u16_vec(count: usize) -> Vec<u16>{
    let mut x = Vec::<u16>::with_capacity(count);
    for _ in 0..count { x.push(rand_u16()); }
    x
}

pub fn rand_u32_vec(count: usize) -> Vec<u32>{
    let mut x = Vec::<u32>::with_capacity(count);
    for _ in 0..count { x.push(rand_u32()); }
    x
}

pub fn rand_u64_vec(count: usize) -> Vec<u64>{
    let mut x = Vec::<u64>::with_capacity(count);
    for _ in 0..count { x.push(rand_u64()); }
    x
}

pub fn rand_f32_vec(count: usize) -> Vec<f32>{
    let mut x = Vec::<f32>::with_capacity(count);
    for _ in 0..count { x.push(rand_f32()); }
    x
}

pub fn rand_f64_vec(count: usize) -> Vec<f64>{
    let mut x = Vec::<f64>::with_capacity(count);
    for _ in 0..count { x.push(rand_f64()); }
    x
}

pub fn rand_isize_vec(count: usize) -> Vec<isize>{
    let mut x = Vec::<isize>::with_capacity(count);
    for _ in 0..count { x.push(rand_isize()); }
    x
}

pub fn rand_usize_vec(count: usize) -> Vec<usize>{
    let mut x = Vec::<usize>::with_capacity(count);
    for _ in 0..count { x.push(rand_usize()); }
    x
}

//#################################################################################################
pub fn rand_i8_range(min: i8, max: i8) -> i8{
    let between = Range::new(min,max+1);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}

pub fn rand_i16_range(min: i16, max: i16) -> i16{
    let between = Range::new(min,max+1);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}

pub fn rand_i32_range(min: i32, max: i32) -> i32{
    let between = Range::new(min,max+1);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}

pub fn rand_i64_range(min: i64, max: i64) -> i64{
    let between = Range::new(min,max+1);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}

pub fn rand_u8_range(min: u8, max: u8) -> u8{
    let between = Range::new(min,max+1);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}

pub fn rand_u16_range(min: u16, max: u16) -> u16{
    let between = Range::new(min,max+1);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}

pub fn rand_u32_range(min: u32, max: u32) -> u32{
    let between = Range::new(min,max+1);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}

pub fn rand_u64_range(min: u64, max: u64) -> u64{
    let between = Range::new(min,max+1);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}

pub fn rand_f32_range(min: f32, max: f32) -> f32{
    let between = Range::new(min,max+1.0);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}

pub fn rand_f64_range(min: f64, max: f64) -> f64{
    let between = Range::new(min,max+1.0);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}

pub fn rand_isize_range(min: isize, max: isize) -> isize{
    let between = Range::new(min,max+1);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}


pub fn rand_usize_range(min: usize, max: usize) -> usize{
    let between = Range::new(min,max+1);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}

//#################################################################################################
pub fn rand_i8_vec_range(count: usize, min: i8, max: i8) -> Vec<i8>{
    let mut x = Vec::<i8>::with_capacity(count);
    for _ in 0..count { x.push(rand_i8_range(min,max)); }
    x
}

pub fn rand_i16_vec_range(count: usize, min: i16, max: i16) -> Vec<i16>{
    let mut x = Vec::<i16>::with_capacity(count);
    for _ in 0..count { x.push(rand_i16_range(min,max)); }
    x
}

pub fn rand_i32_vec_range(count: usize, min: i32, max: i32) -> Vec<i32>{
    let mut x = Vec::<i32>::with_capacity(count);
    for _ in 0..count { x.push(rand_i32_range(min,max));}
    x
}

pub fn rand_i64_vec_range(count: usize, min: i64, max: i64) -> Vec<i64>{
    let mut x = Vec::<i64>::with_capacity(count);
    for _ in 0..count { x.push(rand_i64_range(min,max));}
    x
}

pub fn rand_u8_vec_range(count: usize, min: u8, max: u8) -> Vec<u8>{
    let mut x = Vec::<u8>::with_capacity(count);
    for _ in 0..count { x.push(rand_u8_range(min,max)); }
    x
}

pub fn rand_u16_vec_range(count: usize, min: u16, max: u16) -> Vec<u16>{
    let mut x = Vec::<u16>::with_capacity(count);
    for _ in 0..count { x.push(rand_u16_range(min,max)); }
    x
}

pub fn rand_u32_vec_range(count: usize, min: u32, max: u32) -> Vec<u32>{
    let mut x = Vec::<u32>::with_capacity(count);
    for _ in 0..count { x.push(rand_u32_range(min,max)); }
    x
}

pub fn rand_u64_vec_range(count: usize, min: u64, max: u64) -> Vec<u64>{
    let mut x = Vec::<u64>::with_capacity(count);
    for _ in 0..count { x.push(rand_u64_range(min,max)); }
    x
}

pub fn rand_f32_vec_range(count: usize, min: f32, max: f32) -> Vec<f32>{
    let mut x = Vec::<f32>::with_capacity(count);
    for _ in 0..count { x.push(rand_f32_range(min,max)); }
    x
}

pub fn rand_f64_vec_range(count: usize, min: f64, max: f64) -> Vec<f64>{
    let mut x = Vec::<f64>::with_capacity(count);
    for _ in 0..count { x.push(rand_f64_range(min,max)); }
    x
}

pub fn rand_isize_vec_range(count: usize, min: isize, max: isize) -> Vec<isize>{
    let mut x = Vec::<isize>::with_capacity(count);
    for _ in 0..count { x.push(rand_isize_range(min,max)); }
    x
}

pub fn rand_usize_vec_range(count: usize, min: usize, max: usize) -> Vec<usize>{
    let mut x = Vec::<usize>::with_capacity(count);
    for _ in 0..count { x.push(rand_usize_range(min,max)); }
    x
}
