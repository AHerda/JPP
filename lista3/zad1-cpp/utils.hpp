#ifndef UTILS_HPP
#define UTILS_HPP

#include <cstdint>
#include <cmath>

#include "../../lista2/zad1-cpp/gf.hpp"

const uint64_t P = 1234567891;

bool check(uint64_t number) {
    uint64_t i = 2;
    uint64_t tmp = number;

    while (i * i <= tmp) {
        if (tmp % i == 0) {
            if (std::pow(number, (P - 1) / i) == 1) {
                return false;
            }
            tmp /= i;
        } else {
            i++;
        }
    }
    if (tmp > 1 && std::pow(number, (P - 1) / tmp) == 1) {
        return false;
    }
    return true;
}

#endif // UTILS_HPP
