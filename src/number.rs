use crate::fraction::Fraction;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Number {
    pub m: Fraction,
    pub val: Fraction,
}

impl Number {
    pub fn new(m: Fraction, val: Fraction) -> Self {
        Number { m, val }
    }

    pub fn zero() -> Self {
        Number::new(Fraction::zero(), Fraction::zero())
    }

    pub fn from_val(val: Fraction) -> Self {
        Number::new(Fraction::zero(), val)
    }

    pub fn from_m(m: impl Into<Fraction>) -> Self {
        Number::new(m.into(), Fraction::zero())
    }

    pub fn has_m(&self) -> bool {
        self.m != Fraction::zero()
    }

    pub fn is_integer(&self) -> bool {
        self.m.num == 0 && self.val.den == 1
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.m.cmp(&other.m) {
            Ordering::Equal => self.val.cmp(&other.val),
            ord => ord,
        }
    }
}

impl Add for Number {
    type Output = Number;
    fn add(self, other: Number) -> Number {
        Number::new(self.m + other.m, self.val + other.val)
    }
}

impl Sub for Number {
    type Output = Number;
    fn sub(self, other: Number) -> Number {
        Number::new(self.m - other.m, self.val - other.val)
    }
}

impl Mul for Number {
    type Output = Number;
    fn mul(self, other: Number) -> Number {
        // (m1*M + v1) * (m2*M + v2) = (m1*m2)*M^2 + (m1*v2 + m2*v1)*M + v1*v2
        if self.m != Fraction::zero() && other.m != Fraction::zero() {
            panic!("Multiplying two Big-M values results in M^2 which is unsupported");
        }
        Number::new(
            self.m * other.val + other.m * self.val,
            self.val * other.val,
        )
    }
}

impl Mul<Fraction> for Number {
    type Output = Number;
    fn mul(self, other: Fraction) -> Number {
        Number::new(self.m * other, self.val * other)
    }
}

impl Div<Fraction> for Number {
    type Output = Number;
    fn div(self, other: Fraction) -> Number {
        Number::new(self.m / other, self.val / other)
    }
}

impl Neg for Number {
    type Output = Number;
    fn neg(self) -> Number {
        Number::new(-self.m, -self.val)
    }
}

impl From<i64> for Number {
    fn from(n: i64) -> Self {
        Number::from_val(Fraction::from(n))
    }
}

impl From<Fraction> for Number {
    fn from(f: Fraction) -> Self {
        Number::from_val(f)
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let m_zero = self.m == Fraction::zero();
        let val_zero = self.val == Fraction::zero();

        if m_zero && val_zero {
            return write!(f, "0");
        }

        if !m_zero {
            if self.m == Fraction::one() {
                write!(f, "M")?;
            } else if self.m.num == -1 && self.m.den == 1 {
                write!(f, "-M")?;
            } else {
                write!(f, "{}M", self.m)?;
            }
        }

        if !val_zero {
            if !m_zero && self.val.num > 0 {
                write!(f, " + {}", self.val)?;
            } else if !m_zero && self.val.num < 0 {
                write!(f, " - {}", self.val.abs())?;
            } else {
                write!(f, "{}", self.val)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplex_number_ordering() {
        let a = Number::new(Fraction::one(), Fraction::zero()); // 1M
        let b = Number::new(Fraction::zero(), Fraction::new(1000000, 1)); // 1000000
        assert!(a > b);
    }
}
