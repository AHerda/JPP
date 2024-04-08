#include "lib.h"

uint64_t factorial(uint64_t n)
{
	if (n == 0 || n == 1)
		return 1;
	else
		return n * factorial(n - 1);
}

uint64_t gcd(uint64_t a, uint64_t b)
{
	if (b == 0)
		return a;
	else
		return gcd(b, a % b);
}

Solution diophantine(int64_t a, int64_t b, int64_t c)
{
	if (a == 0 && b == 0 && c != 0) {
        return (Solution){.valid = false};
    } else if (a == 0) {
        return (Solution){0, c / b, true};
    } else if (b == 0) {
        return (Solution){c / a, 0, true};
    } else {
        Solution result = diophantine(b, a % b, c);
        if (!result.valid) {
            return (Solution){.valid = false};
        }
		int x = result.y;
        int y = result.x - (a / b) * result.y;
        return (Solution){x, y, true};
    }
}
