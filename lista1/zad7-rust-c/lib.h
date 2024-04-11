#ifndef LIB_H
#define LIB_H

#include <stdbool.h>
#include <stdint.h>

typedef struct {
	int64_t x;
	int64_t y;
	bool valid;
} Solution;

uint64_t factorial(uint64_t n);
uint64_t gcd(uint64_t a, uint64_t b);
Solution diophantine(int64_t a, int64_t b, int64_t c);

#endif
