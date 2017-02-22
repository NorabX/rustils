extern crate rustils;

use std::f32;
use rustils::RoundingMode::*;
use rustils::parse::*;

#[test]
fn parse_f32_to_bool() {
    let a = -1_i8;
    let b = 1_i8;
    let c = 0_i8;
    let d = 0.21_f32;
    let e = f32::NAN;

    assert_eq!(true, a.to_bool());
    assert_eq!(true, b.to_bool());
    assert_eq!(false, c.to_bool());
    assert_eq!(true, d.to_bool());
    assert_eq!(false, e.to_bool());
}


#[test]
fn parse_f32_to_i8_0() {
    let a = -128_f32;
    let b = 127_f32;
    let c = 1.321_f32;
    let d = 1.546_f32;

    assert_eq!(-128_i8, a.to_i8());
    assert_eq!(127_i8, b.to_i8());
    assert_eq!(1_i8, c.to_i8_rm(Trunc));
    assert_eq!(2_i8, c.to_i8_rm(Ceil));
    assert_eq!(1_i8, c.to_i8_rm(Round));
    assert_eq!(1_i8, d.to_i8_rm(Trunc));
    assert_eq!(1_i8, d.to_i8_rm(Floor));
    assert_eq!(2_i8, d.to_i8_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -129")]
fn parse_f32_to_i8_1() { (-129_f32).to_i8(); }

#[test]
#[should_panic(expected = "Invalid Number: 128")]
fn parse_f32_to_i8_2() { (128_f32).to_i8(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f32_to_i8_3() { (f32::NAN).to_i8(); }


#[test]
fn parse_f32_to_i16_0() {
    let a = -32768_f32;
    let b = 32767_f32;
    let c = 1.321_f32;
    let d = 1.546_f32;

    assert_eq!(-32768_i16, a.to_i16());
    assert_eq!(32767_i16, b.to_i16());
    assert_eq!(1_i16, c.to_i16_rm(Trunc));
    assert_eq!(2_i16, c.to_i16_rm(Ceil));
    assert_eq!(1_i16, c.to_i16_rm(Round));
    assert_eq!(1_i16, d.to_i16_rm(Trunc));
    assert_eq!(1_i16, d.to_i16_rm(Floor));
    assert_eq!(2_i16, d.to_i16_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -32769")]
fn parse_f32_to_i16_1() { (-32769_f32).to_i16(); }

#[test]
#[should_panic(expected = "Invalid Number: 32768")]
fn parse_f32_to_i16_2() { (32768_f32).to_i16(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f32_to_i16_3() { (f32::NAN).to_i16(); }


#[test]
fn parse_f32_to_i32_0() {
    let a = -16777215_f32;
    let b = 16777215_f32;
    let c = 1.321_f32;
    let d = 1.546_f32;

    assert_eq!(-16777215_i32, a.to_i32());
    assert_eq!(16777215_i32, b.to_i32());
    assert_eq!(1_i32, c.to_i32_rm(Trunc));
    assert_eq!(2_i32, c.to_i32_rm(Ceil));
    assert_eq!(1_i32, c.to_i32_rm(Round));
    assert_eq!(1_i32, d.to_i32_rm(Trunc));
    assert_eq!(1_i32, d.to_i32_rm(Floor));
    assert_eq!(2_i32, d.to_i32_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -16777216")]
fn parse_f32_to_i32_1() { (-16777216_f32).to_i32(); }

#[test]
#[should_panic(expected = "Invalid Number: 16777216")]
fn parse_f32_to_i32_2() { (16777216_f32).to_i32(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f32_to_i32_3() { (f32::NAN).to_i32(); }


#[test]
fn parse_f32_to_i64_0() {
    let a = -16777215_f32;
    let b = 16777215_f32;
    let c = 1.321_f32;
    let d = 1.546_f32;

    assert_eq!(-16777215_i64, a.to_i64());
    assert_eq!(16777215_i64, b.to_i64());
    assert_eq!(1_i64, c.to_i64_rm(Trunc));
    assert_eq!(2_i64, c.to_i64_rm(Ceil));
    assert_eq!(1_i64, c.to_i64_rm(Round));
    assert_eq!(1_i64, d.to_i64_rm(Trunc));
    assert_eq!(1_i64, d.to_i64_rm(Floor));
    assert_eq!(2_i64, d.to_i64_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -16777216")]
fn parse_f32_to_i64_1() { (-16777216_f32).to_i64(); }

#[test]
#[should_panic(expected = "Invalid Number: 16777216")]
fn parse_f32_to_i64_2() { (16777216_f32).to_i64(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f32_to_i64_3() { (f32::NAN).to_i64(); }


#[test]
fn parse_f32_to_isize_0() {
    let a = -16777215_f32;
    let b = 16777215_f32;
    let c = 1.321_f32;
    let d = 1.546_f32;

    assert_eq!(-16777215_isize, a.to_isize());
    assert_eq!(16777215_isize, b.to_isize());
    assert_eq!(1_isize, c.to_isize_rm(Trunc));
    assert_eq!(2_isize, c.to_isize_rm(Ceil));
    assert_eq!(1_isize, c.to_isize_rm(Round));
    assert_eq!(1_isize, d.to_isize_rm(Trunc));
    assert_eq!(1_isize, d.to_isize_rm(Floor));
    assert_eq!(2_isize, d.to_isize_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -16777216")]
fn parse_f32_to_isize_1() { (-16777216_f32).to_isize(); }

#[test]
#[should_panic(expected = "Invalid Number: 16777216")]
fn parse_f32_to_isize_2() { (16777216_f32).to_isize(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f32_to_isize_3() { (f32::NAN).to_isize(); }


#[test]
fn parse_f32_to_u8_0() {
    let a = 0_f32;
    let b = 255_f32;
    let c = 1.321_f32;
    let d = 1.546_f32;

    assert_eq!(0_u8, a.to_u8());
    assert_eq!(255_u8, b.to_u8());
    assert_eq!(1_u8, c.to_u8_rm(Trunc));
    assert_eq!(2_u8, c.to_u8_rm(Ceil));
    assert_eq!(1_u8, c.to_u8_rm(Round));
    assert_eq!(1_u8, d.to_u8_rm(Trunc));
    assert_eq!(1_u8, d.to_u8_rm(Floor));
    assert_eq!(2_u8, d.to_u8_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_f32_to_u8_1() { (-1_f32).to_u8(); }

#[test]
#[should_panic(expected = "Invalid Number: 256")]
fn parse_f32_to_u8_2() { (256_f32).to_u8(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f32_to_u8_3() { (f32::NAN).to_u8(); }


#[test]
fn parse_f32_to_u16_0() {
    let a = 0_f32;
    let b = 65535_f32;
    let c = 1.321_f32;
    let d = 1.546_f32;

    assert_eq!(0_u16, a.to_u16());
    assert_eq!(65535_u16, b.to_u16());
    assert_eq!(1_u16, c.to_u16_rm(Trunc));
    assert_eq!(2_u16, c.to_u16_rm(Ceil));
    assert_eq!(1_u16, c.to_u16_rm(Round));
    assert_eq!(1_u16, d.to_u16_rm(Trunc));
    assert_eq!(1_u16, d.to_u16_rm(Floor));
    assert_eq!(2_u16, d.to_u16_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_f32_to_u16_1() { (-1_f32).to_u16(); }

#[test]
#[should_panic(expected = "Invalid Number: 65536")]
fn parse_f32_to_u16_2() { (65536_f32).to_u16(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f32_to_u16_3() { (f32::NAN).to_u16(); }


#[test]
fn parse_f32_to_u32_0() {
    let a = 0_f32;
    let b = 16777215_f32;
    let c = 1.321_f32;
    let d = 1.546_f32;

    assert_eq!(0_u32, a.to_u32());
    assert_eq!(16777215_u32, b.to_u32());
    assert_eq!(1_u32, c.to_u32_rm(Trunc));
    assert_eq!(2_u32, c.to_u32_rm(Ceil));
    assert_eq!(1_u32, c.to_u32_rm(Round));
    assert_eq!(1_u32, d.to_u32_rm(Trunc));
    assert_eq!(1_u32, d.to_u32_rm(Floor));
    assert_eq!(2_u32, d.to_u32_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_f32_to_u32_1() { (-1_f32).to_u32(); }

#[test]
#[should_panic(expected = "Invalid Number: 16777216")]
fn parse_f32_to_u32_2() { (16777216_f32).to_u32(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f32_to_u32_3() { (f32::NAN).to_u32(); }


#[test]
fn parse_f32_to_u64_0() {
    let a = 0_f32;
    let b = 16777215_f32;
    let c = 1.321_f32;
    let d = 1.546_f32;

    assert_eq!(0_u64, a.to_u64());
    assert_eq!(16777215_u64, b.to_u64());
    assert_eq!(1_u64, c.to_u64_rm(Trunc));
    assert_eq!(2_u64, c.to_u64_rm(Ceil));
    assert_eq!(1_u64, c.to_u64_rm(Round));
    assert_eq!(1_u64, d.to_u64_rm(Trunc));
    assert_eq!(1_u64, d.to_u64_rm(Floor));
    assert_eq!(2_u64, d.to_u64_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_f32_to_u64_1() { (-1_f32).to_u64(); }

#[test]
#[should_panic(expected = "Invalid Number: 16777216")]
fn parse_f32_to_u64_2() { (16777216_f32).to_u64(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f32_to_u64_3() { (f32::NAN).to_u64(); }


#[test]
fn parse_f32_to_usize_0() {
    let a = 0_f32;
    let b = 16777215_f32;
    let c = 1.321_f32;
    let d = 1.546_f32;

    assert_eq!(0_usize, a.to_usize());
    assert_eq!(16777215_usize, b.to_usize());
    assert_eq!(1_usize, c.to_usize_rm(Trunc));
    assert_eq!(2_usize, c.to_usize_rm(Ceil));
    assert_eq!(1_usize, c.to_usize_rm(Round));
    assert_eq!(1_usize, d.to_usize_rm(Trunc));
    assert_eq!(1_usize, d.to_usize_rm(Floor));
    assert_eq!(2_usize, d.to_usize_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_f32_to_usize_1() { (-1_f32).to_usize(); }

#[test]
#[should_panic(expected = "Invalid Number: 16777216")]
fn parse_f32_to_usize_2() { (16777216_f32).to_usize(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f32_to_usize_3() { (f32::NAN).to_usize(); }