#ifndef WRAPPER_H
#define WRAPPER_H

#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>

typedef struct {
	uint32_t x;
	uint32_t y;
	bool valid;
} result;

extern uint32_t factorial(uint32_t n);
extern uint32_t gdc(uint32_t a, uint32_t b);
extern result diophantine_equation(int32_t a, int32_t b, int32_t c);

#endif // WRAPPER_H
