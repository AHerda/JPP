#ifndef LIB_ITER_H
#define LIB_ITER_H

#include <stdbool.h>
#include <stdint.h>

typedef struct {
	int64_t x;
	int64_t y;
	bool valid;
} ISolution;

uint64_t IFactorial(uint64_t n);
uint64_t IGcd(uint64_t a, uint64_t b);
ISolution IDiophantine(int64_t a, int64_t b, int64_t c);

#endif
