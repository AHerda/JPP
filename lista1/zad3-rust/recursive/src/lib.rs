pub fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn diophantine(a: i64, b: i64, c: i64) -> Option<(i64, i64)> {
    if a == 0 && b == 0 && c != 0 {
        return None;
    } else if a == 0 {
        return Some((0, c / b));
    } else if b == 0 {
        return Some((c / a, 0));
    } else {
        let (y, x) = diophantine(b, a % b, c)?;
        let y = y - (a / b) * x;
        Some((x, y))
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
        let (x, y) = diophantine(3, 5, 10).unwrap();
        assert_eq!(3*x + 5*y, 10);

        let (x, y) = diophantine(3, 5, 11).unwrap();
        assert_eq!(3*x + 5*y, 11);

        let (x, y) = diophantine(3, 5, 12).unwrap();
        assert_eq!(3*x + 5*y, 12);

        let (x, y) = diophantine(3, 7, 22).unwrap();
        assert_eq!(3*x + 7*y, 22);
    }
}
