#ifndef LIB_RECUR_H
#define LIB_RECUR_H

#include <stdbool.h>
#include <stdint.h>

typedef struct {
	int64_t x;
	int64_t y;
	bool valid;
} RSolution;

uint64_t RFactorial(uint64_t n);
uint64_t RGcd(uint64_t a, uint64_t b);
RSolution RDiophantine(int64_t a, int64_t b, int64_t c);

#endif
