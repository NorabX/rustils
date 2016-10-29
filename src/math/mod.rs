use std::ops::{Add,Mul};

pub mod average;
pub mod integer;

pub trait Zero { fn zero() -> Self; }
pub trait One { fn one() -> Self; }
pub trait Half { fn half(self) -> Self; }

macro_rules! std_impl_i {
    ($($t:ty)*) => ($(
        impl Zero for $t { fn zero() -> Self { 0 } }
        impl One for $t { fn one() -> Self { 1 } }
        impl Half for $t { fn half(self) -> Self { self / 2 } }
    )*)
}
std_impl_i! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }

macro_rules! std_impl_f {
    ($($t:ty)*) => ($(
        impl Zero for $t { fn zero() -> Self { 0.0 } }
        impl One for $t { fn one() -> Self { 1.0 } }
        impl Half for $t { fn half(self) -> Self { self / 2.0 } }
    )*)
}
std_impl_f! { f32 f64 }


pub trait Sum<T>{ fn sum(&self) -> T; }
pub trait Product<T> { fn product(&self) -> T; }

impl<T: Add<Output=T> + Zero + Copy> Sum<T> for [T] {
    fn sum(&self) -> T{
        let mut sum = T::zero();
        for i in 0..self.len() {
            sum = sum + self[i];
        }
        sum
    }
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


/*
#[derive(Debug, Copy, Clone)]
pub struct Frac<T>(pub T, pub T);

impl<T: Mul<Output=T> + Add<Output=T> + Copy> Add<Frac<T>> for Frac<T>{
    type Output = Frac<T>;

    fn add(self, other: Frac<T>) -> Frac<T> {
        Frac(self.0 * other.1 + self.1 * other.0, self.1 * other.1)
    }
}
/*
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
}*/
/*
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
}*/

impl<T: fmt::Display> fmt::Display for Frac<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}
*/
