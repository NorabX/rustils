extern crate rustils;

use rustils::parse::*;
use rustils::parse::boolean::*;
use rustils::parse::byte::*;
use rustils::parse::double::*;
use rustils::parse::float::*;
use rustils::parse::int::*;
use rustils::parse::isize::*;
use rustils::parse::long::*;
use rustils::parse::short::*;
use rustils::parse::string::*;
use rustils::parse::ubyte::*;
use rustils::parse::uint::*;
use rustils::parse::ulong::*;
use rustils::parse::ushort::*;
use rustils::parse::usize::*;

#[test]
fn parse_u8_to_bool() {
    let a = 1_u8;
    let b = 0_u8;

    assert_eq!(true, a.to_bool());
    assert_eq!(false, b.to_bool());
}


#[test]
fn parse_u8_to_i8_0() {
    let a = 0_u8;
    let b = 127_u8;

    assert_eq!(0_i8, a.to_i8());
    assert_eq!(127_i8, b.to_i8());
}

#[test]
#[should_panic(expected = "Invalid Number: 128")]
fn parse_u8_to_i8_1() { (128_u8).to_i8(); }
