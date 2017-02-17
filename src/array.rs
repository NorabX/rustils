use random::in_range;
use ArrayUtils;
use std::usize::MAX;

impl<T: Ord + Copy> ArrayUtils<T> for [T] {
    fn swap(&mut self, a: usize, b: usize) -> bool {
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
        for _ in 0..self.len() {
            let a = in_range(0, self.len()-1);
            let b = in_range(0, self.len()-1);
            self.swap(a, b);
        }
    }
/*
    fn is_same_len_as(&self, other: &[T]) -> bool {
        self.len() == other.len()
    }

    fn is_same_len_as_mut(&mut self, other: &mut [T]) -> bool {
        self.len() == other.len()
    }
*/
    fn index_of(&self, search: &T) -> usize {
        for i in 0..self.len() {
            if &self[i] == search {
                return i;
            }
        }

        MAX
    }
}
