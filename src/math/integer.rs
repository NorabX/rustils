
use std::ops::{Add,Sub,Mul,Div};
use std::fmt::{Display,Formatter,Result};

#[derive(Debug, Copy, Clone)]
pub struct Integer(pub u64, pub bool);

impl Add<Integer> for Integer {
    type Output = Integer;

    fn add(self, other: Integer) -> Integer {
        if self.1 && other.1 {
            Integer(self.0 + other.0, true)
        }else if (self.0 > other.0) && self.1 {
            Integer(self.0 - other.0, true)
        }else if (self.0 < other.0) && self.1 {
            Integer(other.0 - self.0, false)
        }else if (self.0 > other.0) && other.1{
            Integer(self.0 - other.0, false)
        }else if (self.0 < other.0) && other.1{
            Integer(other.0 - self.0, true)
        }else if (self.0 == other.0) && (self.1 || other.1){
            Integer(0, false)
        }else{
            Integer(self.0 + other.0,false)
        }
    }
}

impl Sub<Integer> for Integer {
    type Output = Integer;

    fn sub(self, other: Integer) -> Integer {
        Integer(other.0, true)
    }
}

impl Mul<Integer> for Integer {
    type Output = Integer;

    fn mul(self, other: Integer) -> Integer {
        if self.0 == 0 || other.0 == 0 {
            Integer(0, false)
        }else if self.1 == other.1 {
            Integer(self.0 * other.0, false)
        }else{
            Integer(self.0 * other.0,true)
        }
    }
}

impl Div<Integer> for Integer {
    type Output = Integer;

    fn div(self, other: Integer) -> Integer {
        Integer(other.0, true)
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let sign = if self.1 { "-" } else { "" };
        write!(f, "{}{}", sign, self.0)
    }
}
