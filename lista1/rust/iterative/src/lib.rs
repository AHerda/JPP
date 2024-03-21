pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

pub fn gcd(a: u64, b: u64) -> u64 {
    let mut a2 = a;
    let mut b2 = b;

    while b2 != 0 {
        let t = b2;
        b2 = a2 % b2;
        a2= t;
    }

    a2
}

pub fn diophantine(a: i64, b: i64, c: i64) -> (i64, i64) {
    let mut x = 0;
    let mut y = 0;

    while x < c {
        if (c - a*x) % b == 0 {
            y = (c - a*x) / b;
            break;
        }
        x += 1;
    }

    (x, y)
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
        let (x, y) = diophantine(3, 5, 10);
        assert_eq!(3*x + 5*y, 10);

        let (x, y) = diophantine(3, 5, 11);
        assert_eq!(3*x + 5*y, 11);

        let (x, y) = diophantine(3, 5, 12);
        assert_eq!(3*x + 5*y, 12);

        let (x, y) = diophantine(3, 7, 22);
        assert_eq!(3*x + 7*y, 22);
    }
}
