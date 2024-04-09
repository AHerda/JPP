#include "wrapper.h"
#include <stdio.h>

int main()
{
	printf(" ==== Iterative ====\n");
	printf("Factorial of 5: %u\n", factorial_iter(5));
	printf("\tCorrect: 120\n");

	printf("\nGCD of 15 and 25: %u\n", gcd_iter(15, 25));
	printf("\tCorrect: 5\n");

	Result solution = diophantine_equation_iter(3, 7, 22);
	printf("\nDiophantine solution for 3x + 7y = 22: x = %d, y = %d\n", solution.x, solution.y);
	printf("\tCorrect: x = 5, y = 1\n");
	printf("\tOther correct: x = -44, y = 22\n");

	printf("\n\n ==== Recursive ====\n");
	printf("Factorial of 5: %u\n", factorial_rec(5));
	printf("\tCorrect: 120\n");

	printf("\nGCD of 15 and 25: %u\n", gcd_rec(15, 25));
	printf("\tCorrect: 5\n");

	solution = diophantine_equation_rec(3, 7, 22);
	printf("\nDiophantine solution for 3x + 7y = 22: x = %d, y = %d\n", solution.x, solution.y);
	printf("\tCorrect: x = 5, y = 1\n");
	printf("\tOther correct: x = -44, y = 22\n");
	return 0;
}
