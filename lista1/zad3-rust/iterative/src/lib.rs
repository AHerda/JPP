pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

pub fn gcd(a: u64, b: u64) -> u64 {
    let mut a2 = a;
    let mut b2 = b;

    while b2 != 0 {
        let t = b2;
        b2 = a2 % b2;
        a2 = t;
    }

    a2
}

pub fn diophantine(mut a: i64, mut b: i64, c: i64) -> Option<(i64, i64)> {
    let mut x1 = 1;
    let mut y1 = 0;
    let mut x2 = 0;
    let mut y2 = 1;

    while b != 0 {
        let q = a / b;
        let r = a % b;

        let temp = x1 - q * x2;
        x1 = x2;
        x2 = temp;

        let temp = y1 - q * y2;
        y1 = y2;
        y2 = temp;

        a = b;
        b = r;
    }

    if c % a != 0 {
        None
    } else {
        let x = x1 * (c / a);
        let y = y1 * (c / a);
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
        assert_eq!(3 * x + 5 * y, 10);

        let (x, y) = diophantine(3, 5, 11).unwrap();
        assert_eq!(3 * x + 5 * y, 11);

        let (x, y) = diophantine(3, 5, 12).unwrap();
        assert_eq!(3 * x + 5 * y, 12);

        let (x, y) = diophantine(3, 7, 22).unwrap();
        assert_eq!(3 * x + 7 * y, 22);
    }
}
