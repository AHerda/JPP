#[repr(C)]
pub struct Solution {
    pub x: i64,
    pub y: i64,
    pub valid: bool,
}

extern "C" {
    pub fn factorial(n: u64) -> u64;
    pub fn gcd(a: u64, b: u64) -> u64;
    pub fn diophantine(a: i64, b: i64, c: i64) -> Solution;
}

fn main() {
    println!("Factorial of 5: {}", unsafe { factorial(5) });
    println!("GCD of 12 and 15: {}", unsafe { gcd(12, 15) });
    let solution = unsafe { diophantine(3, 6, 18) };
    println!("Diophantine solution: x = {}, y = {}", solution.x, solution.y);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_factorial() {
        unsafe {
            assert_eq!(factorial(0), 1);
            assert_eq!(factorial(1), 1);
            assert_eq!(factorial(2), 2);
            assert_eq!(factorial(3), 6);
            assert_eq!(factorial(4), 24);
            assert_eq!(factorial(5), 120);
        }
    }

    #[test]
    fn test_gcd() {
        unsafe {
            assert_eq!(gcd(0, 0), 0);
            assert_eq!(gcd(0, 1), 1);
            assert_eq!(gcd(1, 0), 1);
            assert_eq!(gcd(1, 1), 1);
            assert_eq!(gcd(2, 3), 1);
            assert_eq!(gcd(3, 2), 1);
            assert_eq!(gcd(4, 6), 2);
            assert_eq!(gcd(6, 4), 2);
            assert_eq!(gcd(6, 9), 3);
            assert_eq!(gcd(9, 6), 3);
            assert_eq!(gcd(12, 15), 3);
            assert_eq!(gcd(15, 12), 3);
            assert_eq!(gcd(15, 25), 5);
            assert_eq!(gcd(25, 15), 5);
        }
    }

    #[test]
    fn test_diophantine() {
        unsafe {
            let a = 2;
            let b = 3;
            let c = 5;
            let s = diophantine(a, b, c);
            assert!(s.valid);
            if s.valid {
                assert_eq!(a * s.x + b * s.y, c);
            }

            let c = 6;
            let s = diophantine(a, b, c);
            assert!(s.valid);
            if s.valid {
                assert_eq!(a * s.x + b * s.y, c);
            }

            let c = 7;
            let s = diophantine(a, b, c);
            assert!(s.valid);
            if s.valid {
                assert_eq!(a * s.x + b * s.y, c);
            }

            let c = 8;
            let s = diophantine(a, b, c);
            assert!(s.valid);
            if s.valid {
                assert_eq!(a * s.x + b * s.y, c);
            }

            let c = 9;
            let s = diophantine(a, b, c);
            assert!(s.valid);
            if s.valid {
                assert_eq!(a * s.x + b * s.y, c);
            }

            let c = 10;
            let s = diophantine(a, b, c);
            assert!(s.valid);
            if s.valid {
                assert_eq!(a * s.x + b * s.y, c);
            }

            let c = 11;
            let s = diophantine(a, b, c);
            assert!(s.valid);
            if s.valid {
                assert_eq!(a * s.x + b * s.y, c);
            }

            let c = 12;
            let s = diophantine(a, b, c);
            assert!(s.valid);
            if s.valid {
                assert_eq!(a * s.x + b * s.y, c);
            }

            let c = 13;
            let s = diophantine(a, b, c);
            assert!(s.valid);
            if s.valid {
                assert_eq!(a * s.x + b * s.y, c);
            }

            let c = 14;
            let s = diophantine(a, b, c);
            assert!(s.valid);
            if s.valid {
                assert_eq!(a * s.x + b * s.y, c);
            }
        }
    }
}
