extern crate rustils;

use rustils::array::ArrayUtils;
use std::usize::MAX;

#[test]
pub fn test_swaping_integer(){
    let ary = &mut [1, 2, 3, 4];

    assert_eq!(true, ary.swaping(1, 3));
    assert_eq!(&mut [1, 4, 3, 2], ary);

    assert_eq!(false, ary.swaping(0, 4));
    assert_eq!(&mut [1, 4, 3, 2], ary);
}

#[test]
pub fn test_swaping_str(){
    let ary = &mut ["one", "two", "three", "four"];

    assert_eq!(true, ary.swaping(1, 3));
    assert_eq!(&mut ["one", "four", "three", "two"], ary);

    assert_eq!(false, ary.swaping(0, 4));
    assert_eq!(&mut ["one", "four", "three", "two"], ary);
}

#[test]
pub fn test_index_of_integer(){
    let ary = [1, 2, 2, 4];

    assert_eq!(1, ary.index_of(&2));
    assert_eq!(MAX, ary.index_of(&3));
}

#[test]
pub fn test_index_of_str(){
    let ary = ["one", "two", "two", "four"];

    assert_eq!(1, ary.index_of(&"two"));
    assert_eq!(MAX, ary.index_of(&"three"));
}

#[test]
pub fn test_chunk_integer() {
    let ary = [1, 3, 5, 7, 9];

    assert_eq!([[1, 3].to_vec(), [5, 7].to_vec(), [9].to_vec()].to_vec(),
        ary.chunk(2));
}

#[test]
pub fn test_fill_mut_integer() {
    let mut ary = [1, 2, 3, 4, 5];

    ary.fill_mut(&7);

    assert_eq!(ary, [7, 7, 7, 7, 7]);
}

#[test]
pub fn test_unique_integer() {
    let ary = [1, 2, 2, 3, 4, 4];

    assert_eq!(vec![1, 2, 3, 4], ary.unique());
}

#[test]
pub fn test_unique_str() {
    let ary = ["one", "one", "two", "two"];

    assert_eq!(vec!["one", "two"], ary.unique());
}

#[test]
pub fn test_unique_adv_integer() {
    let ary = [1, 2, 2, 3, 4, 4];

    assert_eq!(
        (vec![1, 2, 3, 4], vec![2, 4]),
        ary.unique_adv());
}

#[test]
pub fn test_unique_adv_str() {
    let ary = ["one", "one", "two", "two"];

    assert_eq!(
        (vec!["one", "two"], vec!["one", "two"]),
        ary.unique_adv());
}
