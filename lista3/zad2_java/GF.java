package JPP.lista3.zad2_java;

import JPP.lista3.zad2_java.Arithmetics;

public class GF implements Arithmetics {
	public static final long P = 1234567891;

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

	@Override
    public Arithmetics mul(Arithmetics other) {
        return mul((GF) other);
    }

    @Override
    public Arithmetics div(Arithmetics other) {
        return div((GF) other);
    }

    @Override
    public Arithmetics fromLong(long value) {
        return new GF(value);
    }
}
