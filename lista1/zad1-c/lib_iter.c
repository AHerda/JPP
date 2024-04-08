#include "lib.h"

uint64_t factorial(uint64_t n)
{
	uint64_t result = 1;
	for (uint64_t i = 1; i <= n; i++)
		result *= i;
	return result;
}

uint64_t gcd(uint64_t a, uint64_t b)
{
	while (b != 0)
	{
		uint64_t temp = b;
		b = a % b;
		a = temp;
	}
	return a;
}

Solution diophantine(int64_t a, int64_t b, int64_t c)
{
	int x1 = 1;
    int y1 = 0;
    int x2 = 0;
    int y2 = 1;

    while (b != 0) {
        int q = a / b;
        int r = a % b;

        int temp = x1 - q * x2;
        x1 = x2;
        x2 = temp;

        temp = y1 - q * y2;
        y1 = y2;
        y2 = temp;

        a = b;
        b = r;
    }

    if (c % a != 0) {
        return (Solution){0, 0, false};
    } else {
        int x = x1 * (c / a);
        int y = y1 * (c / a);
        return (Solution){x, y, true};
    }
}
