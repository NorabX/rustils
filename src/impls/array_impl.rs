extern crate rand;

use array;

impl<T: Ord + Copy> array::ArrayUtils<T> for [T] {

    fn swaping(&mut self, a: usize, b: usize)
        -> bool {

        array::swaping(self, a, b)
    }

    fn index_of(&self, search: &T) -> usize {

        array::index_of(self, search)
    }

    fn chunk(&self, size: usize) -> Vec<Vec<T>> {
        array::chunk(self, size)
    }

    fn fill_mut(&mut self, value: &T) {
        array::fill_mut(self, value)
    }
}
