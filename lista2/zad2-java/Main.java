public class Main {
    public static void main(String[] args) {
        GF a = new GF(1234560);
        GF b = new GF(10);

        System.out.println("a = " + a);
        System.out.println("b = " + b);

        GF c = a.add(b);
        System.out.println("a + b = " + c);
        c = a.sub(b);
        System.out.println("a - b = " + c);
        c = a.mul(b);
        System.out.println("a * b = " + c);
        c = a.div(b);
        System.out.println("a / b = " + c);

        System.out.println("a == b <=> " + a.eq(b));
        System.out.println("a != b <=> " + a.neq(b));
        System.out.println("a < b <=> " + (a.lt(b)));
        System.out.println("a > b <=> " + (a.gt(b)));
        System.out.println("a <= b <=> " + (a.le(b)));
        System.out.println("a >= b <=> " + (a.ge(b)));

        System.out.println("\nCharacteristic od a = " + GF.characteristic());
    }
}

class GF {
	private static final long P = 1234577;

	private long value;

	public static long characteristic() {
        long p = P;
        long prime = 0;
        for (int i = 2; i * i <= P; i++) {
            if (p % i == 0) {
                prime = i;
                break;
            }
        }

        if (prime == 0) {
            return p;
        }

        while (p != 1) {
            if (p % prime == 0) {
                p /= prime;
            } else {
                throw new ArithmeticException("Not a prime power");
            }
        }
        return prime;
    }

	public GF(long value) {
		this.value = (value % P + P) % P;
	}

	public GF add(GF other) {
		return new GF(value + other.value);
	}

	public void addAssign(GF other) {
		value = (value + other.value) % P;
	}

	public GF sub(GF other) {
		return new GF(value + P - other.value);
	}

	public void subAssign(GF other) {
		value = (value + P - other.value) % P;
	}

	public GF mul(GF other) {
		return new GF(value * other.value);
	}

	public void mulAssign(GF other) {
		value = (value * other.value) % P;
	}

	public GF div(GF other) {
		return mul(other.inv());
	}

	public void divAssign(GF other) {
		mulAssign(other.inv());
	}

	public boolean eq(GF other) {
		return value == other.value;
	}

	public boolean neq(GF other) {
		return value != other.value;
	}

	public boolean lt(GF other) {
		return value < other.value;
	}

	public boolean le(GF other) {
		return value <= other.value;
	}

	public boolean gt(GF other) {
		return value > other.value;
	}

	public boolean ge(GF other) {
		return value >= other.value;
	}

	public long getValue() {
		return value;
	}

	public GF inv() {
        long t = 0;
        long newt = 1;
        long r = P;
        long newr = value;
        while (newr != 0) {
            long quotient = r / newr;
            long tmp = newt;
            newt = t - quotient * newt;
            t = tmp;
            tmp = newr;
            newr = r - quotient * newr;
            r = tmp;
        }
        if (r > 1) {
            throw new ArithmeticException(value + " is not invertible");
        }
        if (t < 0) {
            t += P;
        }
        return new GF(t);
    }

	@Override
    public String toString() {
        return String.valueOf(value);
    }
}
