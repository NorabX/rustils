// <editor-fold> # Uses

use rand::{ Rand, random, thread_rng };
use rand::distributions::{ IndependentSample, Range };
use rand::distributions::range::SampleRange;
// </editor-fold>

// <editor-fold> # Functions

pub fn in_range<T: SampleRange + PartialOrd>(min: T, max: T) -> T {
    let between = Range::new(min, max);
    let mut rng = thread_rng();
    between.ind_sample(&mut rng)
}

pub fn vec<T: Rand>(count: usize) -> Vec<T> {
    let mut x = Vec::<T>::with_capacity(count);
    for _ in 0..count { x.push(random::<T>()); }
    x
}

pub fn vec_in_range<T: SampleRange + PartialOrd + Copy>
    (count: usize, min: T, max: T) -> Vec<T> {

    let mut x = Vec::<T>::with_capacity(count);
    for _ in 0..count { x.push(in_range(min, max)); }
    x
}
// </editor-fold>
