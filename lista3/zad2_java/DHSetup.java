package JPP.lista3.zad2_java;

import java.util.Random;
import java.util.function.Function;
import java.lang.Math;

import JPP.lista3.zad2_java.Arithmetics;

public class DHSetup<T extends Arithmetics> {

	private T generator;

	public static boolean check(long number) {
		long i = 2;
		long tmp = number;

		while (i * i <= tmp) {
			if (tmp % i == 0) {
				if (Math.pow(number, (GF.P - 1) / i) == 1) {
					return false;
				}
				tmp /= i;
			} else {
				i++;
			}
		}
		if (tmp > 1 && Math.pow(number, (GF.P - 1) / tmp) == 1) {
			return false;
		}
		return true;
	}

	public DHSetup(Function<Long, T> constructor) {
		Random rand = new Random();
		long g;

		do {
			g = 1 + rand.nextLong() % (GF.P - 1);
		} while (!check(g));

		this.generator = constructor.apply(g);
	}

	public T getGenerator() {
		return generator;
	}

	public T power(T number, long b) {
		T result = (T) number.fromLong(1);
		while (b > 0) {
			if (b % 2 == 1) {
				result = (T) result.mul(number);
			}
			number = (T) number.mul(number);
			b >>= 1;
		}
		return result;
	}
}
