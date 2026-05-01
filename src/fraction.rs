use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fraction {
    pub num: i64,
    pub den: i64,
}

impl Fraction {
    pub fn new(num: i64, den: i64) -> Self {
        if den == 0 {
            panic!("Denominator cannot be zero");
        }
        let g = gcd(num, den);
        let mut n = num / g;
        let mut d = den / g;
        if d < 0 {
            n = -n;
            d = -d;
        }
        Fraction { num: n, den: d }
    }

    pub fn zero() -> Self {
        Fraction::new(0, 1)
    }

    pub fn one() -> Self {
        Fraction::new(1, 1)
    }

    pub fn abs(self) -> Self {
        Fraction::new(self.num.abs(), self.den.abs())
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = self.num * other.den;
        let rhs = other.num * self.den;
        (lhs).cmp(&rhs)
    }
}

impl From<i64> for Fraction {
    fn from(n: i64) -> Self {
        Fraction::new(n, 1)
    }
}

impl Add for Fraction {
    type Output = Fraction;
    fn add(self, other: Fraction) -> Fraction {
        Fraction::new(
            self.num * other.den + other.num * self.den,
            self.den * other.den,
        )
    }
}

impl Sub for Fraction {
    type Output = Fraction;
    fn sub(self, other: Fraction) -> Fraction {
        Fraction::new(
            self.num * other.den - other.num * self.den,
            self.den * other.den,
        )
    }
}

impl Mul for Fraction {
    type Output = Fraction;
    fn mul(self, other: Fraction) -> Fraction {
        Fraction::new(self.num * other.num, self.den * other.den)
    }
}

impl Div for Fraction {
    type Output = Fraction;
    fn div(self, other: Fraction) -> Fraction {
        Fraction::new(self.num * other.den, self.den * other.num)
    }
}

impl Neg for Fraction {
    type Output = Fraction;
    fn neg(self) -> Fraction {
        Fraction { num: -self.num, den: self.den }
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

impl fmt::Debug for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraction() {
        let a = Fraction::new(2, 4);
        assert_eq!(a.num, 1);
        assert_eq!(a.den, 2);
        
        let b = Fraction::new(1, 3);
        let c = a + b;
        assert_eq!(c, Fraction::new(5, 6));
    }
}
