extern crate rand;

use array;

impl<T: Ord + Copy> array::ArrayUtils<T> for [T] {

    fn swaping(&mut self, a: usize, b: usize)
        -> bool {

        array::swaping(self, a, b)
    }

    fn shuffle(&mut self) {

        array::shuffle(self)
    }

    fn shuffle_seed(&mut self, seed: &[usize]) {

        array::shuffle_seed(self, seed)
    }

    fn index_of(&self, search: &T) -> usize {

        array::index_of(self, search)
    }
}
