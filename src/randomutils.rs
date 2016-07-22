use rand;
use rand::distributions::{IndependentSample, Range};

pub fn rand_i8_vec(count: usize) -> Vec<i8>{
    let mut x = Vec::<i8>::with_capacity(count);

    for _ in 0..count {
        x.push(rand::random::<i8>());
    }

    x
}

pub fn rand_i8_vec_range(count: usize, min: i8, max: i8) -> Vec<i8>{
    let between = Range::new(min,max+1);
    let mut rng = rand::thread_rng();
    let mut x = Vec::<i8>::with_capacity(count);

    for _ in 0..count {
        x.push(between.ind_sample(&mut rng));
    }

    x
}

pub fn rand_i16_vec(count: usize) -> Vec<i16>{
    let mut x = Vec::<i16>::with_capacity(count);

    for _ in 0..count {
        x.push(rand::random::<i16>());
    }

    x
}

pub fn rand_i16_vec_range(count: usize, min: i16, max: i16) -> Vec<i16>{
    let between = Range::new(min,max+1);
    let mut rng = rand::thread_rng();
    let mut x = Vec::<i16>::with_capacity(count);

    for _ in 0..count {
        x.push(between.ind_sample(&mut rng));
    }

    x
}

pub fn rand_i32_vec(count: usize) -> Vec<i32>{
    let mut x = Vec::<i32>::with_capacity(count);

    for _ in 0..count {
        x.push(rand::random::<i32>());
    }

    x
}

pub fn rand_i32_vec_range(count: usize, min: i32, max: i32) -> Vec<i32>{
    let between = Range::new(min,max+1);
    let mut rng = rand::thread_rng();
    let mut x = Vec::<i32>::with_capacity(count);

    for _ in 0..count {
        x.push(between.ind_sample(&mut rng));
    }

    x
}

pub fn rand_i64_vec(count: usize) -> Vec<i64>{
    let mut x = Vec::<i64>::with_capacity(count);

    for _ in 0..count {
        x.push(rand::random::<i64>());
    }

    x
}

pub fn rand_i64_vec_range(count: usize, min: i64, max: i64) -> Vec<i64>{
    let between = Range::new(min,max+1);
    let mut rng = rand::thread_rng();
    let mut x = Vec::<i64>::with_capacity(count);

    for _ in 0..count {
        x.push(between.ind_sample(&mut rng));
    }

    x
}

pub fn rand_u8_vec(count: usize) -> Vec<u8>{
    let mut x = Vec::<u8>::with_capacity(count);

    for _ in 0..count {
        x.push(rand::random::<u8>());
    }

    x
}

pub fn rand_u8_vec_range(count: usize, min: u8, max: u8) -> Vec<u8>{
    let between = Range::new(min,max+1);
    let mut rng = rand::thread_rng();
    let mut x = Vec::<u8>::with_capacity(count);

    for _ in 0..count {
        x.push(between.ind_sample(&mut rng));
    }

    x
}

pub fn rand_u16_vec(count: usize) -> Vec<u16>{
    let mut x = Vec::<u16>::with_capacity(count);

    for _ in 0..count {
        x.push(rand::random::<u16>());
    }

    x
}

pub fn rand_u16_vec_range(count: usize, min: u16, max: u16) -> Vec<u16>{
    let between = Range::new(min,max+1);
    let mut rng = rand::thread_rng();
    let mut x = Vec::<u16>::with_capacity(count);

    for _ in 0..count {
        x.push(between.ind_sample(&mut rng));
    }

    x
}

pub fn rand_u32_vec(count: usize) -> Vec<u32>{
    let mut x = Vec::<u32>::with_capacity(count);

    for _ in 0..count {
        x.push(rand::random::<u32>());
    }

    x
}

pub fn rand_u32_vec_range(count: usize, min: u32, max: u32) -> Vec<u32>{
    let between = Range::new(min,max+1);
    let mut rng = rand::thread_rng();
    let mut x = Vec::<u32>::with_capacity(count);

    for _ in 0..count {
        x.push(between.ind_sample(&mut rng));
    }

    x
}

pub fn rand_u64_vec(count: usize) -> Vec<u64>{
    let mut x = Vec::<u64>::with_capacity(count);

    for _ in 0..count {
        x.push(rand::random::<u64>());
    }

    x
}

pub fn rand_u64_vec_range(count: usize, min: u64, max: u64) -> Vec<u64>{
    let between = Range::new(min,max+1);
    let mut rng = rand::thread_rng();
    let mut x = Vec::<u64>::with_capacity(count);

    for _ in 0..count {
        x.push(between.ind_sample(&mut rng));
    }

    x
}

pub fn rand_f32_vec(count: usize) -> Vec<f32>{
    let mut x = Vec::<f32>::with_capacity(count);

    for _ in 0..count {
        x.push(rand::random::<f32>());
    }

    x
}

pub fn rand_f32_vec_range(count: usize, min: f32, max: f32) -> Vec<f32>{
    let between = Range::new(min,max+1.0);
    let mut rng = rand::thread_rng();
    let mut x = Vec::<f32>::with_capacity(count);

    for _ in 0..count {
        x.push(between.ind_sample(&mut rng));
    }

    x
}

pub fn rand_f64_vec(count: usize) -> Vec<f64>{
    let mut x = Vec::<f64>::with_capacity(count);

    for _ in 0..count {
        x.push(rand::random::<f64>());
    }

    x
}

pub fn rand_f64_vec_range(count: usize, min: f64, max: f64) -> Vec<f64>{
    let between = Range::new(min,max+1.0);
    let mut rng = rand::thread_rng();
    let mut x = Vec::<f64>::with_capacity(count);

    for _ in 0..count {
        x.push(between.ind_sample(&mut rng));
    }

    x
}
