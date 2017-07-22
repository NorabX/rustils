extern crate rand;

// <editor-fold> # Uses

use std::usize::MAX;
use rand::{ thread_rng, Rng, SeedableRng, StdRng };
// </editor-fold>

// <editor-fold> # Traits

pub trait ArrayUtils<T> {

    fn swaping(&mut self, a: usize, b: usize)
        -> bool;

    fn shuffle(&mut self);

    fn shuffle_seed(&mut self, seed: &[usize]);

    fn index_of(&self, search: &T)
        -> usize;
}
// </editor-fold>

// <editor-fold> # Functions

pub fn swaping<T: Ord + Copy>(ary: &mut [T], a: usize, b: usize)
    -> bool {

    if a == b { return true }
    else{
        if a > ary.len()-1 || b > ary.len()-1 { return false }
        else{
            let temp = ary[a];
            ary[a] = ary[b];
            ary[b] = temp;
            true
        }
    }
}

pub fn shuffle<T: Ord + Copy>(ary: &mut [T]) {

    let mut rng = thread_rng();
    rng.shuffle(ary);
}

pub fn shuffle_seed<T: Ord + Copy>(ary: &mut [T], seed: &[usize]) {

    let mut rng_seed: StdRng = SeedableRng::from_seed(seed);
    rng_seed.shuffle(ary);
}

pub fn index_of<T: Ord + Copy>(ary: &[T], search: &T)
    -> usize {

    for i in 0..ary.len() {
        if &ary[i] == search {
            return i;
        }
    }

    MAX
}
// </editor-fold>
