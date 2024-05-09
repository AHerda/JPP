#include "DHSetup.hpp"

template <typename T>
DHSetup<T>::DHSetup() {
	std::random_device rd;
	std::mt19937 rng(rd());
	std::uniform_int_distribution<unsigned long> dist(1, P - 1);

	uint64_t g;
	do {
		g = dist(rng);
	} while (!check(g));

	this->generator = T(g);
}

template <typename T>
T DHSetup<T>::getGenerator() {
	return this->generator;
}

template <typename T>
T DHSetup<T>::power(T a, uint64_t b) {
	T result = T(1);
	while (b > 0) {
		if (b & 1) {
			result *= a;
		}
		a *= a;
		b >>= 1;
	}
	return result;
}
