#ifndef WRAPPER_H
#define WRAPPER_H

#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>

typedef struct {
	int32_t x;
	int32_t y;
	bool valid;
} Result;

extern uint32_t factorial_iter(uint32_t n);
extern uint32_t gcd_iter(uint32_t a, uint32_t b);
extern Result diophantine_equation_iter(int32_t a, int32_t b, int32_t c);

extern uint32_t factorial_rec(uint32_t n);
extern uint32_t gcd_rec(uint32_t a, uint32_t b);
extern Result diophantine_equation_rec(int32_t a, int32_t b, int32_t c);

#endif // WRAPPER_H
