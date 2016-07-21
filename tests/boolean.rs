extern crate rustils;
use rustils::parse::boolean;

#[test]
fn parse_bool_to_i8(){
    assert_eq!(1_i8,boolean::toi8(true));
    assert_eq!(0_i8,boolean::toi8(false));
}

#[test]
fn parse_bool_to_i16(){
    assert_eq!(1_i16,boolean::toi16(true));
    assert_eq!(0_i16,boolean::toi16(false));
}

#[test]
fn parse_bool_to_i32(){
    assert_eq!(1_i32,boolean::toi32(true));
    assert_eq!(0_i32,boolean::toi32(false));
}

#[test]
fn parse_bool_to_i64(){
    assert_eq!(1_i64,boolean::toi64(true));
    assert_eq!(0_i64,boolean::toi64(false));
}

#[test]
fn parse_bool_to_u8(){
    assert_eq!(1_u8,boolean::tou8(true));
    assert_eq!(0_u8,boolean::tou8(false));
}

#[test]
fn parse_bool_to_u16(){
    assert_eq!(1_u16,boolean::tou16(true));
    assert_eq!(0_u16,boolean::tou16(false));
}

#[test]
fn parse_bool_to_u64(){
    assert_eq!(1_u64,boolean::tou64(true));
    assert_eq!(0_u64,boolean::tou64(false));
}

#[test]
fn parse_bool_to_f32(){
    assert_eq!(1_f32,boolean::tof32(true));
    assert_eq!(0_f32,boolean::tof32(false));
}

#[test]
fn parse_bool_to_f64(){
    assert_eq!(1_f64,boolean::tof64(true));
    assert_eq!(0_f64,boolean::tof64(false));
}
