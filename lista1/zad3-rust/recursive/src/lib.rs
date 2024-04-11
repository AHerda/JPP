#[repr(C)]
pub struct Solution {
    pub x: i64,
    pub y: i64,
    pub valid: bool,
}

#[no_mangle]
pub extern "C" fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[no_mangle]
pub extern "C" fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[no_mangle]
pub extern "C" fn diophantine(a: i64, b: i64, c: i64) -> Solution {
    if a == 0 && b == 0 && c != 0 {
        return Solution {
            x: 0,
            y: 0,
            valid: false,
        };
    } else if a == 0 {
        return Solution {
            x: 0,
            y: c / b,
            valid: true,
        };
    } else if b == 0 {
        return Solution {
            x: c / a,
            y: 0,
            valid: true,
        };
    } else {
        let mut s = diophantine(b, a % b, c);
        std::mem::swap(&mut s.x, &mut s.y);
        s.y = s.y - (a / b) * s.x;
        return s;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_test() {
        let result = factorial(5);
        assert_eq!(result, 120);

        let result = factorial(7);
        assert_eq!(result, 5040);
    }

    #[test]
    fn gcd_test() {
        let result = gcd(12, 15);
        assert_eq!(result, 3);

        let result = gcd(15, 25);
        assert_eq!(result, 5);

        let result = gcd(22, 55);
        assert_eq!(result, 11);

        let result = gcd(0, 15);
        assert_eq!(result, 15);
    }

    #[test]
    fn diophantine_test() {
        let s = diophantine(3, 5, 10);
        assert_eq!(3 * s.x + 5 * s.y, 10);

        let s = diophantine(3, 5, 11);
        assert_eq!(3 * s.x + 5 * s.y, 11);

        let s = diophantine(3, 5, 12);
        assert_eq!(3 * s.x + 5 * s.y, 12);

        let s = diophantine(3, 7, 22);
        assert_eq!(3 * s.x + 7 * s.y, 22);
    }
}
