use destiny_helpers::num::F64NumSupport;
use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

#[derive(Clone, Copy, Default)]
pub struct Decimal(f64);

impl Decimal {
    pub fn new(value: f64) -> Self {
        Self(value.to_safe())
    }

    pub fn to_f64(self) -> f64 {
        self.0
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0.
    }

    #[inline]
    pub fn zero() -> Self {
        Self(0.0)
    }

    #[inline]
    pub fn one() -> Self {
        Self(1.0)
    }

    #[inline]
    pub fn one_neg() -> Self {
        Self(-1.0)
    }
}

impl Add for Decimal {
    type Output = Decimal;

    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0).to_safe())
    }
}

impl AddAssign for Decimal {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = (self.0 + rhs.0).to_safe();
    }
}

impl Sub for Decimal {
    type Output = Decimal;

    fn sub(self, rhs: Self) -> Self::Output {
        Self((self.0 - rhs.0).to_safe())
    }
}

impl SubAssign for Decimal {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = (self.0 - rhs.0).to_safe();
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 * rhs.0).to_safe())
    }
}

impl MulAssign for Decimal {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = (self.0 * rhs.0).to_safe();
    }
}

impl Div for Decimal {
    type Output = Decimal;

    fn div(self, rhs: Self) -> Self::Output {
        Self((self.0 / rhs.0).to_safe())
    }
}

impl DivAssign for Decimal {
    fn div_assign(&mut self, rhs: Self) {
        self.0 = (self.0 / rhs.0).to_safe();
    }
}

impl Rem for Decimal {
    type Output = Decimal;

    fn rem(self, rhs: Self) -> Self::Output {
        Self((self.0 % rhs.0).to_safe())
    }
}

impl RemAssign for Decimal {
    fn rem_assign(&mut self, rhs: Self) {
        self.0 = (self.0 % rhs.0).to_safe();
    }
}

impl Neg for Decimal {
    type Output = Decimal;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Sum for Decimal {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Decimal::zero(), |acc, item| acc + item)
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
