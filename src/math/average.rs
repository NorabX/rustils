use std::ops::{Add, Div};
use math::{Half, Zero, One};

pub trait Median<T: Add + Half + Copy>{
    fn median(&self) -> T;
}

impl<T: Add<Output=T> + Half + Copy> Median<T> for [T]{
    fn median(&self) -> T{
        let l = self.len();
        if l % 2 == 0 {
            (self[l/2 - 1] + self[l/2]).half()
        }else{
            self[(l + 1)/2 - 1]
        }
    }
}

pub trait ArithmeticMean<T: Add + Copy>{
    fn arithmetric_mean(&self) -> T;
}

impl<T: Add<Output=T> + Div<Output=T> + Zero + One + Copy> ArithmeticMean<T> for [T] {
    fn arithmetric_mean(&self) -> T{
        let mut sum = T::zero();
        let mut n = T::zero();
        for i in 0..self.len() {
            sum = sum + self[i];
            n = n + T::one();
        }
        sum / n
    }
}
