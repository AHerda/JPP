use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use super::gf::{self, GF};

impl Display for GF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl Add for GF {
    type Output = GF;

    fn add(self, other: Self) -> Self::Output {
        GF::new((self.value() + other.value()) as i128)
    }
}

impl AddAssign for GF {
    fn add_assign(&mut self, other: Self) {
        *self = GF::new((self.value() + other.value()) as i128);
    }
}

impl Sub for GF {
    type Output = GF;

    fn sub(self, other: Self) -> Self::Output {
        GF::new((self.value() + gf::P - other.value()) as i128)
    }
}

impl SubAssign for GF {
    fn sub_assign(&mut self, other: Self) {
        *self = GF::new((self.value() + gf::P - other.value()) as i128);
    }
}

impl Mul for GF {
    type Output = GF;

    fn mul(self, other: Self) -> Self::Output {
        GF::new((self.value() * other.value()) as i128)
    }
}

impl MulAssign for GF {
    fn mul_assign(&mut self, other: Self) {
        *self = GF::new((self.value() * other.value()) as i128);
    }
}

impl Div for GF {
    type Output = GF;

    fn div(self, other: Self) -> Self::Output {
        self * other.inv()
    }
}

impl DivAssign for GF {
    fn div_assign(&mut self, other: Self) {
        *self *= other.inv();
    }
}

impl PartialEq for GF {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }

    fn ne(&self, other: &Self) -> bool {
        self.value() != other.value()
    }
}

impl PartialOrd for GF {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value().partial_cmp(&other.value())
    }
}
