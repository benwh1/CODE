use std::{
    num::ParseIntError,
    ops::{Add, Div, Mul, Rem, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Z(pub(crate) i128);

impl Add for Z {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Z {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Mul for Z {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_mul(rhs.0))
    }
}

impl Div for Z {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.checked_div(rhs.0).unwrap_or(i128::MAX))
    }
}

impl Rem for Z {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0.checked_rem(rhs.0).unwrap_or(i128::MAX))
    }
}

impl FromStr for Z {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i128>().map(Z)
    }
}
