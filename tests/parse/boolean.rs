extern crate rustils;
use rustils::parse::boolean;

#[test]
fn parse_bool_to_i8(){
    assert_eq!(1_i8,boolean::to_i8(true));
    assert_eq!(0_i8,boolean::to_i8(false));
}

#[test]
fn parse_bool_to_i16(){
    assert_eq!(1_i16,boolean::to_i16(true));
    assert_eq!(0_i16,boolean::to_i16(false));
}

#[test]
fn parse_bool_to_i32(){
    assert_eq!(1_i32,boolean::to_i32(true));
    assert_eq!(0_i32,boolean::to_i32(false));
}

#[test]
fn parse_bool_to_i64(){
    assert_eq!(1_i64,boolean::to_i64(true));
    assert_eq!(0_i64,boolean::to_i64(false));
}

#[test]
fn parse_bool_to_u8(){
    assert_eq!(1_u8,boolean::to_u8(true));
    assert_eq!(0_u8,boolean::to_u8(false));
}

#[test]
fn parse_bool_to_u16(){
    assert_eq!(1_u16,boolean::to_u16(true));
    assert_eq!(0_u16,boolean::to_u16(false));
}

#[test]
fn parse_bool_to_u64(){
    assert_eq!(1_u64,boolean::to_u64(true));
    assert_eq!(0_u64,boolean::to_u64(false));
}

#[test]
fn parse_bool_to_f32(){
    assert_eq!(1_f32,boolean::to_f32(true));
    assert_eq!(0_f32,boolean::to_f32(false));
}

#[test]
fn parse_bool_to_f64(){
    assert_eq!(1_f64,boolean::to_f64(true));
    assert_eq!(0_f64,boolean::to_f64(false));
}

#[test]
fn parse_bool_to_usize(){
    assert_eq!(1_usize,boolean::to_usize(true));
    assert_eq!(0_usize,boolean::to_usize(false));
}
#[test]
fn parse_bool_to_isize(){
    assert_eq!(1_isize,boolean::to_isize(true));
    assert_eq!(0_isize,boolean::to_isize(false));
}
