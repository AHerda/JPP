pub static P: u64 = 1234577;

#[derive(Clone, Copy, Debug)]
pub struct GF {
    value: u64,
}

impl GF {
    pub fn new(value: i128) -> GF {
        let p = P as i128;
        GF {
            value: ((value % p + p) % p) as u64,
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn inv(&self) -> GF {
        let mut t = 0_i128;
        let mut newt = 1_i128;
        let mut r = P as i128;
        let mut newr = self.value as i128;

        while newr != 0 {
            let q = r / newr;
            let tmp = newt;
            newt = t - q * newt;
            t = tmp;
            let tmp = newr;
            newr = r - q * newr;
            r = tmp;
        }

        if r > 1 {
            panic!("{} is not invertible", self.value);
        }

        GF::new(t)
    }

    pub fn characteristic() -> (u64, u64) {
        let mut p = P;
        let mut prime = 0;
        let mut n = 0;
        let mut i = 2;

        while i * i <= p {
            if p % i == 0 {
                prime = i;
                break;
            }

            i += 1;
        }

        if prime == 0 {
            return (p, 1);
        }

        while p != 1 {
            if p % prime == 0 {
                p /= prime;
                n += 1;
            } else {
                panic!("Not a prime power");
            }
        }

        (prime, n)
    }
}
