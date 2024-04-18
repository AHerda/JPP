#ifndef GF_HPP
#define GF_HPP

#include <ostream>
#include <cstdint>

const uint64_t P = 1234577;

class GF {
public:
	GF(int64_t value_) : value((value_ % P + P) % P) {}

	GF operator+(const GF& other) const { return GF(value + other.value); }
	GF& operator+=(const GF& other) { value = (value + other.value) % P; return *this; }
	GF operator-(const GF& other) const { return GF(value + P - other.value); }
	GF& operator-=(const GF& other) { value = (value + P - other.value) % P; return *this; }
	GF operator*(const GF& other) const { return GF(value * other.value); }
	GF& operator*=(const GF& other) { value = (value * other.value) % P; return *this; }
	GF operator/(const GF& other) const { return *this * other.inv(); }
	GF& operator/=(const GF& other) { *this *= other.inv(); return *this; }

	friend bool operator==(const GF& lhs, const GF& rhs) { return lhs.value == rhs.value; }
	friend bool operator!=(const GF& lhs, const GF& rhs) { return lhs.value != rhs.value; }
	friend bool operator<(const GF& lhs, const GF& rhs) { return lhs.value < rhs.value; }
	friend bool operator>(const GF& lhs, const GF& rhs) { return lhs.value > rhs.value; }
	friend bool operator<=(const GF& lhs, const GF& rhs) { return lhs.value <= rhs.value; }
	friend bool operator>=(const GF& lhs, const GF& rhs) { return lhs.value >= rhs.value; }
	friend std::ostream& operator<<(std::ostream& os, const GF& gf) { return os << gf.value; }


	GF inv() const;
	std::pair<uint64_t, uint64_t> characteristic() const;
	uint64_t getValue() const;
private:
	uint64_t value;
};

#endif // GF_HPP
