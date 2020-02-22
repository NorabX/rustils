extern crate rand;

use std::usize::MAX;

pub trait ArrayUtils<T> {

    fn swaping(&mut self, a: usize, b: usize)
        -> bool;

    fn index_of(&self, search: &T)
        -> usize;
}

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

pub fn index_of<T: Ord + Copy>(ary: &[T], search: &T)
    -> usize {

    for i in 0..ary.len() {
        if &ary[i] == search {
            return i;
        }
    }

    MAX
}
