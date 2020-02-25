extern crate rand;

use std::usize::MAX;

pub trait ArrayUtils<T> {

    fn swaping(&mut self, a: usize, b: usize)
        -> bool;

    fn index_of(&self, search: &T)
        -> usize;

    fn chunk(&self, size: usize)
        -> Vec<Vec<T>>;

    fn fill_mut(&mut self, value: &T);
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

pub fn chunk<T: Clone>(ary: &[T], size: usize)
    -> Vec<Vec<T>> {

    let mut iter = ary.chunks(size);
    let mut next = iter.next();
    let mut res = Vec::<Vec<T>>::new();

    while !next.is_none() {
        res.push((*(next.unwrap())).to_vec());
        next = iter.next();
    }

    res
}

pub fn fill_mut<T: Copy>(ary: &mut [T], value: &T) {
    for i in 0..ary.len() {
        ary[i] = *value;
    }
}
