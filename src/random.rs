use rand::{ thread_rng, Rng };
use rand::distributions::uniform::{ SampleUniform };

pub fn in_range<T: SampleUniform + PartialOrd>(min: T, max: T) -> T {
    let mut rng = thread_rng();
    let res: T = rng.gen_range(min, max);
    res
}

pub fn vec_in_range<T: SampleUniform + PartialOrd + Copy>
    (count: usize, min: T, max: T) -> Vec<T> {

    let mut x = Vec::<T>::with_capacity(count);
    for _ in 0..count { x.push(in_range(min, max)); }
    x
}
