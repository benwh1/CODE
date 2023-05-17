use std::ops::{Add, Div, Mul, Sub};

const MODULUS: u8 = 127;
static INVERSES: [u8; 127] = [
    0, 1, 64, 85, 32, 51, 106, 109, 16, 113, 89, 104, 53, 88, 118, 17, 8, 15, 120, 107, 108, 121,
    52, 116, 90, 61, 44, 80, 59, 92, 72, 41, 4, 77, 71, 98, 60, 103, 117, 114, 54, 31, 124, 65, 26,
    48, 58, 100, 45, 70, 94, 5, 22, 12, 40, 97, 93, 78, 46, 28, 36, 25, 84, 125, 2, 43, 102, 91,
    99, 81, 49, 34, 30, 87, 115, 105, 122, 33, 57, 82, 27, 69, 79, 101, 62, 3, 96, 73, 13, 10, 24,
    67, 29, 56, 50, 123, 86, 55, 35, 68, 47, 83, 66, 37, 11, 75, 6, 19, 20, 7, 112, 119, 110, 9,
    39, 74, 23, 38, 14, 111, 18, 21, 76, 95, 42, 63, 126,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Int(pub(crate) u8);

impl Add for Int {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0) % MODULUS)
    }
}

impl Sub for Int {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self((self.0 + (MODULUS - rhs.0)) % MODULUS)
    }
}

impl Mul for Int {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(((self.0 as u16 * rhs.0 as u16) % MODULUS as u16) as u8)
    }
}

impl Div for Int {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.0 == 0 {
            Self(127)
        } else {
            self * Int(INVERSES[rhs.0 as usize])
        }
    }
}
