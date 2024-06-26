#include <stdexcept>

#include "consts.hpp"
#include "gf.hpp"

GF::GF(int64_t value_) : value((value_ % P + P) % P) {}

GF GF::operator+(const GF& other) const { return GF(value + other.value); }
GF& GF::operator+=(const GF& other) { value = (value + other.value) % P; return *this; }
GF GF::operator-(const GF& other) const { return GF(value + P - other.value); }
GF& GF::operator-=(const GF& other) { value = (value + P - other.value) % P; return *this; }
GF GF::operator*(const GF& other) const { return GF(value * other.value); }
GF& GF::operator*=(const GF& other) { value = (value * other.value) % P; return *this; }
GF GF::operator/(const GF& other) const { return *this * other.inv(); }
GF& GF::operator/=(const GF& other) { *this *= other.inv(); return *this; }

GF GF::inv() const {
	int64_t t = 0;
	int64_t newt = 1;
	int64_t r = P;
	int64_t newr = static_cast<int64_t>(value);
	while (newr != 0) {
		int64_t quotient = r / newr;
		int64_t tmp = newt;
		newt = t - quotient * newt;
		t = tmp;
		tmp = newr;
		newr = r - quotient * newr;
		r = tmp;
	}
	if (r > 1) {
		throw std::runtime_error(value + " is not invertible");
	}
	if (t < 0) {
		t += P;
	}
	return GF(t);
}

std::pair<uint64_t, uint64_t> GF::characteristic() const {
	uint64_t p = P;
	uint64_t prime = 0;
	uint64_t n = 0;
	for (uint64_t i = 2; i * i <= P; i++) {
		if (p % i == 0) {
			prime = 1;
			break;
		}
	}

	if (prime == 0) {
		return std::make_pair(P, 1);
	}

	while (p != 1) {
		if (p % prime == 0) {
			p /= prime;
			n++;
		} else {
			std::runtime_error("Not a prime power");
		}
	}
	return std::make_pair(prime, n);
}

uint64_t GF::getValue() const {
	return value;
}
