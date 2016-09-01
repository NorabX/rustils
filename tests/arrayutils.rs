extern crate rustils;
use rustils::arrayutils;

#[test]
fn swap_i8(){
    let mut x = [1,2,3,4,5,6,7,8,9,10];
    arrayutils::swap_i8(&mut x, 2, 8);
    assert_eq!([1,2,9,4,5,6,7,8,3,10],x);
}

#[test]
fn swap_i16(){
    let mut x = [1,2,3,4,5,6,7,8,9,10];
    arrayutils::swap_i16(&mut x, 2, 8);
    assert_eq!([1,2,9,4,5,6,7,8,3,10],x);
}

#[test]
fn swap_i32(){
    let mut x = [1,2,3,4,5,6,7,8,9,10];
    arrayutils::swap_i32(&mut x, 2, 8);
    assert_eq!([1,2,9,4,5,6,7,8,3,10],x);
}

#[test]
fn swap_i64(){
    let mut x = [1,2,3,4,5,6,7,8,9,10];
    arrayutils::swap_i64(&mut x, 2, 8);
    assert_eq!([1,2,9,4,5,6,7,8,3,10],x);
}

#[test]
fn swap_u8(){
    let mut x = [1,2,3,4,5,6,7,8,9,10];
    arrayutils::swap_u8(&mut x, 2, 8);
    assert_eq!([1,2,9,4,5,6,7,8,3,10],x);
}

#[test]
fn swap_u16(){
    let mut x = [1,2,3,4,5,6,7,8,9,10];
    arrayutils::swap_u16(&mut x, 2, 8);
    assert_eq!([1,2,9,4,5,6,7,8,3,10],x);
}

#[test]
fn swap_u32(){
    let mut x = [1,2,3,4,5,6,7,8,9,10];
    arrayutils::swap_u32(&mut x, 2, 8);
    assert_eq!([1,2,9,4,5,6,7,8,3,10],x);
}

#[test]
fn swap_u64(){
    let mut x = [1,2,3,4,5,6,7,8,9,10];
    arrayutils::swap_u64(&mut x, 2, 8);
    assert_eq!([1,2,9,4,5,6,7,8,3,10],x);
}

#[test]
fn swap_f32(){
    let mut x = [1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0];
    arrayutils::swap_f32(&mut x, 2, 8);
    assert_eq!([1.0,2.0,9.0,4.0,5.0,6.0,7.0,8.0,3.0,10.0],x);
}

#[test]
fn swap_f64(){
    let mut x = [1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0];
    arrayutils::swap_f64(&mut x, 2, 8);
    assert_eq!([1.0,2.0,9.0,4.0,5.0,6.0,7.0,8.0,3.0,10.0],x);
}

#[test]
fn swap_usize(){
    let mut x = [1,2,3,4,5,6,7,8,9,10];
    arrayutils::swap_usize(&mut x, 2, 8);
    assert_eq!([1,2,9,4,5,6,7,8,3,10],x);
}

#[test]
fn swap_isize(){
    let mut x = [1,2,3,4,5,6,7,8,9,10];
    arrayutils::swap_isize(&mut x, 2, 8);
    assert_eq!([1,2,9,4,5,6,7,8,3,10],x);
}
