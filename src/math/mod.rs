use std::ops::{Add,Sub,Mul,Div};
use std::fmt;

pub mod average;

pub trait Half<T> {
    fn half(self) -> T;
}

impl Half<u8> for u8{
    fn half(self) -> u8{
        self / 2
    }
}

impl Half<f64> for f64 {
    fn half(self) -> f64 {
        self / 2.0
    }
}

impl Half<Frac> for Frac {
    fn half(self) -> Frac {
        self * Frac(1,2)
    }
}

pub trait One: Sized {
    fn one() -> Self;
}

impl One for i32 {
    fn one() -> Self { 1 }
}

pub trait Zero: Sized {
    fn zero() -> Self;
}

impl Zero for i32 {
    fn zero() -> Self { 0 }
}

impl Zero for Frac {
    fn zero() -> Self { Frac(0,1) }
}

pub trait Sum<T>{
    fn sum(&self) -> T;
}

impl<T: Add<Output=T> + Zero + Copy> Sum<T> for [T] {
    fn sum(&self) -> T{
        let mut sum = T::zero();
        for i in 0..self.len() {
            sum = sum + self[i];
        }
        sum
    }
}

pub trait Product<T> {
    fn product(&self) -> T;
}

impl<T: Mul<Output=T> + One + Copy> Product<T> for [T]{
    fn product(&self) -> T {
        let mut product = T::one();
        for i in 0..self.len() {
            product = product * self[i];
        }
        product
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Frac(pub i64, pub i64);

impl Add<Frac> for Frac{
    type Output = Frac;

    fn add(self, other: Frac) -> Frac {
        Frac(self.0 * other.1 + self.1 * other.0, self.1 * other.1)
    }
}

impl Sub<Frac> for Frac{
    type Output = Frac;

    fn sub(self, other: Frac) -> Frac {
        Frac(self.0 * other.1 - self.1 * other.0, self.1 * other.1)
    }
}

impl Div<Frac> for Frac{
    type Output = Frac;

    fn div(self, other: Frac) -> Frac {
        Frac(self.0 * other.1, self.1 * other.0)
    }
}

impl Mul<Frac> for Frac{
    type Output = Frac;

    fn mul(self, other: Frac) -> Frac {
        Frac(self.0 * other.0, self.1 * other.1)
    }
}

impl<'a> Mul<Frac> for &'a Frac{
    type Output = Frac;

    fn mul(self, other: Frac) -> Frac {
        Frac(self.0 * other.0, self.1 * other.1)
    }
}

impl<'a> Mul<&'a Frac> for Frac{
    type Output = Frac;

    fn mul(self, other: &'a Frac) -> Frac {
        Frac(self.0 * other.0, self.1 * other.1)
    }
}

impl<'a, 'b> Mul<&'a Frac> for &'b Frac{
    type Output = Frac;

    fn mul(self, other: &'a Frac) -> Frac {
        Frac(self.0 * other.0, self.1 * other.1)
    }
}

impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}
