#include "wrapper.h"
#include <stdio.h>

int main()
{
	printf("Factorial of 5: %lu\n", factorial(5));
	printf("\tCorrect: 120\n");

	printf("GCD of 15 and 25: %lu\n", gcd(15, 25));
	printf("\tCorrect: 5\n");

	result solution = diophantine_equation(3, 7, 22);
	printf("Diophantine solution for 3x + 7y = 22: x = %ld, y = %ld\n", solution.x, solution.y);
	printf("\tCorrect: x = 5, y = 1\n");
	printf("\tOther correct: x = -44, y = 22\n");
	return 0;
}
