extern crate rustils;

use rustils::parse::byte;

#[test]
fn parse_i8_to_bool(){
    assert_eq!(true,byte::tobool(-1_i8));
    assert_eq!(true,byte::tobool(1_i8));
    assert_eq!(false,byte::tobool(0_i8));
}

#[test]
fn parse_i8_to_u8(){
    assert_eq!(0_u8,byte::tou8p(0_i8));
    assert_eq!(127_u8,byte::tou8p(i8::max_value()));
}

#[test]
fn parse_i8_to_u16(){
    assert_eq!(0_u16,byte::tou16p(0_i8));
    assert_eq!(127_u16,byte::tou16p(i8::max_value()));
}

#[test]
fn parse_i8_to_u32(){
    assert_eq!(0_u32,byte::tou32p(0_i8));
    assert_eq!(127_u32,byte::tou32p(i8::max_value()));
}

#[test]
fn parse_i8_to_u64(){
    assert_eq!(0_u64,byte::tou64p(0_i8));
    assert_eq!(127_u64,byte::tou64p(i8::max_value()));
}
