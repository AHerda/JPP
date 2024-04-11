#include "lib_recur.h"

uint64_t RFactorial(uint64_t n)
{
	if (n == 0 || n == 1)
		return 1;
	else
		return n * RFactorial(n - 1);
}

uint64_t RGcd(uint64_t a, uint64_t b)
{
	if (b == 0)
		return a;
	else
		return RGcd(b, a % b);
}

RSolution RDiophantine(int64_t a, int64_t b, int64_t c)
{
	if (a == 0 && b == 0 && c != 0) {
        return (RSolution){.valid = false};
    } else if (a == 0) {
        return (RSolution){0, c / b, true};
    } else if (b == 0) {
        return (RSolution){c / a, 0, true};
    } else {
        RSolution result = RDiophantine(b, a % b, c);
        if (!result.valid) {
            return (RSolution){.valid = false};
        }
		int x = result.y;
        int y = result.x - (a / b) * result.y;
        return (RSolution){x, y, true};
    }
}
