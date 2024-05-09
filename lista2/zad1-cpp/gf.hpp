#ifndef GF_HPP
#define GF_HPP

#include <ostream>
#include <cstdint>

//const uint64_t P = 1234567891;

class GF {
public:
	GF() : value(0) {}
	GF(int64_t value_);

	GF operator+(const GF& other) const;
	GF& operator+=(const GF& other);
	GF operator-(const GF& other) const;
	GF& operator-=(const GF& other);
	GF operator*(const GF& other) const;
	GF& operator*=(const GF& other);
	GF operator/(const GF& other) const;
	GF& operator/=(const GF& other);

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
