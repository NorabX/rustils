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
