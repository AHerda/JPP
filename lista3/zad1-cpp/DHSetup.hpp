#ifndef DHSETUP_HPP
#define DHSETUP_HPP

#include <cstdint>
#include <random>

#include "utils.hpp"

template <typename T>
class DHSetup {
private:
	T generator;

public:
	DHSetup() {
		std::random_device rd;
		std::mt19937 rng(rd());
		std::uniform_int_distribution<unsigned long> dist(1, P - 1);

		uint64_t g;
		do {
			g = dist(rng);
		} while (!check(g));

		this->generator = T(g);
	}

	T getGenerator() {
		return this->generator;
	}

	T power(T a, uint64_t b) {
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
};

#endif // DHSETUP_HPP
