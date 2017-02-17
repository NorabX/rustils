extern crate rustils;

use rustils::sorting::*;
use rustils::SortingAlgorithmn::{ Bubble, Quick };

#[test]
pub fn test_adv_sort_mut_bubble(){
    let temp1 = &mut [1,5,2,0,2,5,7,45,32,8];
    let temp2 = &mut [1];

    temp1.adv_sort_mut(Bubble);
    temp2.adv_sort_mut(Bubble);

    assert_eq!(&mut [0,1,2,2,5,5,7,8,32,45], temp1);
    assert_eq!(&mut [1], temp2);
}

#[test]
pub fn test_bubble_sort_mut(){
    let temp1 = &mut [1,5,2,0,2,5,7,45,32,8];
    let temp2 = &mut [1];

     bubble_sort_mut(temp1);
     bubble_sort_mut(temp2);

    assert_eq!(&mut [0,1,2,2,5,5,7,8,32,45], temp1);
    assert_eq!(&mut [1], temp2);
}

#[test]
pub fn test_adv_sort_mut_quick(){
    let temp1 = &mut [1,5,2,0,2,5,7,45,32,8];
    let temp2 = &mut [1];

    temp1.adv_sort_mut(Quick);
    temp2.adv_sort_mut(Quick);

    assert_eq!(&mut [0,1,2,2,5,5,7,8,32,45], temp1);
    assert_eq!(&mut [1], temp2);
}

#[test]
pub fn test_quick_sort_mut(){
    let temp1 = &mut [1,5,2,0,2,5,7,45,32,8];
    let temp2 = &mut [1];

     quick_sort_mut(temp1);
     quick_sort_mut(temp2);

    assert_eq!(&mut [0,1,2,2,5,5,7,8,32,45], temp1);
    assert_eq!(&mut [1], temp2);
}

#[test]
pub fn test_adv_sort_by_mut_bubble(){
    let temp1 = &mut [1,5,2,0,2,5,7,45,32,8];
    let temp2 = &mut [1];

    temp1.adv_sort_by_mut(&mut |a, b| a.cmp(b), Bubble);
    temp2.adv_sort_by_mut(&mut |a, b| a.cmp(b), Bubble);

    assert_eq!(&mut [0,1,2,2,5,5,7,8,32,45], temp1);
    assert_eq!(&mut [1], temp2);
}

#[test]
pub fn test_adv_sort_by_mut_quick(){
    let temp1 = &mut [1,5,2,0,2,5,7,45,32,8];
    let temp2 = &mut [1];

    temp1.adv_sort_by_mut(&mut |a, b| a.cmp(b), Quick);
    temp2.adv_sort_by_mut(&mut |a, b| a.cmp(b), Quick);

    assert_eq!(&mut [0,1,2,2,5,5,7,8,32,45], temp1);
    assert_eq!(&mut [1], temp2);
}
