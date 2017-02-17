extern crate rand;

use ArrayUtils;
use std::usize::MAX;
use rand::{thread_rng, Rng, SeedableRng, StdRng};

impl<T: Ord + Copy> ArrayUtils<T> for [T] {
    fn swaping(&mut self, a: usize, b: usize) -> bool {
        if a == b { return true }
        else{
            if a > self.len()-1 || b > self.len()-1 { return false }
            else{
                let temp = self[a];
                self[a] = self[b];
                self[b] = temp;
                true
            }
        }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        rng.shuffle(self);
    }

    fn shuffle_seed(&mut self, seed: &[usize]){
        let mut rng_seed: StdRng = SeedableRng::from_seed(seed);
        rng_seed.shuffle(self);
    }

    fn index_of(&self, search: &T) -> usize {
        for i in 0..self.len() {
            if &self[i] == search {
                return i;
            }
        }

        MAX
    }
}
