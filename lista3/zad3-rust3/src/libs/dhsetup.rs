use rand::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub static P: u64 = 1234567891;

pub struct DHSetup<T>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + From<u64>
        + Copy,
{
    generator: T,
}

impl<T> DHSetup<T>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + From<u64>
        + Copy,
{
    pub fn new() -> Self {
        let mut g: u64 = rand::thread_rng().gen_range(1..P);

        while !Self::check(g) {
            g = rand::thread_rng().gen_range(1..P);
        }
        DHSetup {
            generator: T::from(g),
        }
    }

    fn check(number: u64) -> bool {
        let mut i: u64 = 2;
        let mut tmp = number;

        while i * i <= tmp {
            if tmp % i == 0 {
                if power(number as u128, ((P - 1) / i) as u128) == 1 {
                    return false;
                }
                tmp /= i;
            } else {
                i += 1;
            }
        }
        if tmp > 1 && power(number as u128, ((P - 1) / tmp) as u128) == 1 {
            return false;
        }
        true
    }

    pub fn getGenerator(&self) -> T {
        self.generator
    }

    pub fn power(mut a: T, mut b: u64) -> T {
        let mut result = T::from(1);
        while b > 0 {
            if b & 1 == 1 {
                result *= a;
            }
            a *= a;
            b >>= 1;
        }
        result
    }
}

fn power(mut a: u128, mut b: u128) -> u128 {
    let mut result = 1;
    while b > 0 {
        if b & 1 == 1 {
            result = (result * a) % (P as u128);
        }
        a = (a * a) % (P as u128);
        b >>= 1;
    }
    result
}
